//! Deterministic, order-independent random draws.
//!
//! A uniform draw is a reproducible sample in `[0, 1)`, not a model probability itself.
//! Callers compare it with a probability supplied by their model code.

use std::{
    sync::mpsc::{Receiver, sync_channel},
    thread::{self, JoinHandle},
};

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const ALGORITHM: &str = "blake3-xof";
const ALGORITHM_VERSION: &str = "1";
const INDEXED_CHACHA8_ALGORITHM: &str = "blake3-seeded-chacha8-indexed";
const INDEXED_CHACHA8_ALGORITHM_VERSION: &str = "1";
const DERIVE_KEY_CONTEXT: &str = "star_sim deterministic uniform draws blake3-xof v1";
const F64_UNIT_SCALE: f64 = 1.0 / ((1_u64 << 53) as f64);
const PREFETCHED_CHUNK_CAPACITY: usize = 2;
const MAX_VALUES_PER_CHUNK: usize = 1_048_576;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum RandomDrawError {
    #[error("{field} must not be empty")]
    EmptyField { field: &'static str },
}

/// Stable address for one deterministic random draw.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "RandomDrawAddressWire")]
pub struct RandomDrawAddress {
    pub algorithm: String,
    pub algorithm_version: String,
    pub draw_namespace: String,
    pub stable_object_id: String,
    pub stream_key: String,
    pub bounded_attempt_index: u32,
}

#[derive(Deserialize)]
struct RandomDrawAddressWire {
    algorithm: String,
    algorithm_version: String,
    draw_namespace: String,
    stable_object_id: String,
    stream_key: String,
    bounded_attempt_index: u32,
}

impl RandomDrawAddress {
    pub fn new(
        algorithm: impl Into<String>,
        algorithm_version: impl Into<String>,
        draw_namespace: impl Into<String>,
        stable_object_id: impl Into<String>,
        stream_key: impl Into<String>,
        bounded_attempt_index: u32,
    ) -> Result<Self, RandomDrawError> {
        let address = Self {
            algorithm: algorithm.into(),
            algorithm_version: algorithm_version.into(),
            draw_namespace: draw_namespace.into(),
            stable_object_id: stable_object_id.into(),
            stream_key: stream_key.into(),
            bounded_attempt_index,
        };
        address.validate()?;
        Ok(address)
    }

    pub fn validate(&self) -> Result<(), RandomDrawError> {
        validate_text(&self.algorithm, "algorithm")?;
        validate_text(&self.algorithm_version, "algorithm_version")?;
        validate_text(&self.draw_namespace, "draw_namespace")?;
        validate_text(&self.stable_object_id, "stable_object_id")?;
        validate_text(&self.stream_key, "stream_key")?;
        Ok(())
    }
}

impl TryFrom<RandomDrawAddressWire> for RandomDrawAddress {
    type Error = RandomDrawError;

    fn try_from(value: RandomDrawAddressWire) -> Result<Self, Self::Error> {
        Self::new(
            value.algorithm,
            value.algorithm_version,
            value.draw_namespace,
            value.stable_object_id,
            value.stream_key,
            value.bounded_attempt_index,
        )
    }
}

/// Stable identity shared by an indexed sequence of related random draws.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RandomDrawScope {
    draw_namespace: String,
    stable_object_id: String,
    stream_key: String,
}

impl RandomDrawScope {
    pub fn new(
        draw_namespace: impl Into<String>,
        stable_object_id: impl Into<String>,
        stream_key: impl Into<String>,
    ) -> Result<Self, RandomDrawError> {
        let address = RandomDrawAddress::new(
            ALGORITHM,
            ALGORITHM_VERSION,
            draw_namespace,
            stable_object_id,
            stream_key,
            0,
        )?;
        Ok(Self {
            draw_namespace: address.draw_namespace,
            stable_object_id: address.stable_object_id,
            stream_key: address.stream_key,
        })
    }

    /// Assigns one stable bounded-attempt index within this scope.
    pub fn at(&self, bounded_attempt_index: u32) -> RandomDrawAddress {
        RandomDrawAddress {
            algorithm: ALGORITHM.to_owned(),
            algorithm_version: ALGORITHM_VERSION.to_owned(),
            draw_namespace: self.draw_namespace.clone(),
            stable_object_id: self.stable_object_id.clone(),
            stream_key: self.stream_key.clone(),
            bounded_attempt_index,
        }
    }
}

/// Reproduces stochastic draws from a simulation seed and stable draw addresses.
#[derive(Debug, Clone, Copy)]
pub struct DeterministicDraws {
    seed: u64,
}

impl DeterministicDraws {
    pub const fn new(seed: u64) -> Self {
        Self { seed }
    }

    /// Returns the uniform draw assigned to `address` in the half-open interval `[0, 1)`.
    ///
    /// Panics when the address names an unsupported algorithm or malformed indexed object.
    pub fn uniform(&self, address: &RandomDrawAddress) -> f64 {
        match (
            address.algorithm.as_str(),
            address.algorithm_version.as_str(),
        ) {
            (ALGORITHM, ALGORITHM_VERSION) => {
                let mut output = address_hasher(self.seed, address).finalize_xof();
                output.set_position(u64::from(address.bounded_attempt_index) * 8);
                let mut bytes = [0_u8; 8];
                output.fill(&mut bytes);
                uniform_from_bytes(bytes)
            }
            (INDEXED_CHACHA8_ALGORITHM, INDEXED_CHACHA8_ALGORITHM_VERSION) => {
                indexed_chacha8_uniform(self.seed, address)
            }
            _ => panic!(
                "unsupported random draw algorithm {} {}",
                address.algorithm, address.algorithm_version
            ),
        }
    }

    /// Creates an indexed stream with two chunks prepared on a background CPU thread.
    ///
    /// `values_per_chunk` must be in `1..=1_048_576`. Chunking changes only when work
    /// is prepared; index `n` always equals [`Self::uniform`] for `scope.at(n)`.
    pub fn prefetched_stream(
        &self,
        scope: RandomDrawScope,
        values_per_chunk: usize,
    ) -> PrefetchedDrawStream {
        assert!(values_per_chunk > 0, "draw chunks must not be empty");
        assert!(
            values_per_chunk <= MAX_VALUES_PER_CHUNK,
            "draw chunk exceeds the supported size"
        );
        let (sender, receiver) = sync_channel(PREFETCHED_CHUNK_CAPACITY);
        let draws = *self;
        let worker = thread::Builder::new()
            .name("simulation-draw-prefetch".into())
            .spawn(move || {
                let mut next_index = 0_u64;
                loop {
                    let Some(values) = draws.chunk(&scope, next_index, values_per_chunk) else {
                        return;
                    };
                    next_index += u64::try_from(values.len()).expect("draw chunk is too large");
                    if sender.send(values).is_err() {
                        return;
                    }
                }
            })
            .expect("failed to start draw prefetch thread");

        PrefetchedDrawStream {
            receiver: Some(receiver),
            worker: Some(worker),
            current: Vec::new().into_iter(),
        }
    }

    fn chunk(
        &self,
        scope: &RandomDrawScope,
        start_index: u64,
        requested_count: usize,
    ) -> Option<Vec<f64>> {
        let draw_limit = u64::from(u32::MAX) + 1;
        let remaining = draw_limit.checked_sub(start_index)?;
        if remaining == 0 {
            return None;
        }
        let count = usize::try_from(
            remaining.min(u64::try_from(requested_count).expect("draw chunk is too large")),
        )
        .expect("remaining draw count fits usize");
        let byte_count = count.checked_mul(8).expect("draw chunk is too large");
        let mut bytes = vec![0_u8; byte_count];
        let mut output = scope_hasher(self.seed, scope).finalize_xof();
        output.set_position(start_index * 8);
        output.fill(&mut bytes);

        Some(
            bytes
                .chunks_exact(8)
                .map(|bytes| uniform_from_bytes(bytes.try_into().expect("eight-byte draw")))
                .collect(),
        )
    }
}

/// Sequential view over the stable indexes of one [`RandomDrawScope`].
pub struct PrefetchedDrawStream {
    receiver: Option<Receiver<Vec<f64>>>,
    worker: Option<JoinHandle<()>>,
    current: std::vec::IntoIter<f64>,
}

impl PrefetchedDrawStream {
    /// Returns the next indexed uniform draw, or `None` after every `u32` index was consumed.
    pub fn next_uniform(&mut self) -> Option<f64> {
        if let Some(value) = self.current.next() {
            return Some(value);
        }

        self.current = self.receiver.as_ref()?.recv().ok()?.into_iter();
        self.current.next()
    }
}

impl Drop for PrefetchedDrawStream {
    fn drop(&mut self) {
        drop(self.receiver.take());
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

fn indexed_chacha8_uniform(seed: u64, address: &RandomDrawAddress) -> f64 {
    let index = address
        .stable_object_id
        .strip_prefix("indexed-u64-le:")
        .and_then(|value| value.split('/').next())
        .and_then(|value| u64::from_str_radix(value, 16).ok())
        .expect("indexed ChaCha8 draw object must start with indexed-u64-le:<hex>");
    let mut input = Vec::with_capacity(64);
    input.extend_from_slice(b"star_sim/");
    input.extend_from_slice(address.draw_namespace.as_bytes());
    input.extend_from_slice(&seed.to_le_bytes());
    input.extend_from_slice(&index.to_le_bytes());
    let mut rng = ChaCha8Rng::from_seed(*blake3::hash(&input).as_bytes());
    (0..=address.bounded_attempt_index)
        .map(|_| rng.gen_range(0.0..1.0))
        .last()
        .expect("bounded draw range contains its endpoint")
}

fn address_hasher(seed: u64, address: &RandomDrawAddress) -> blake3::Hasher {
    scoped_hasher(
        seed,
        &address.algorithm,
        &address.algorithm_version,
        &address.draw_namespace,
        &address.stable_object_id,
        &address.stream_key,
    )
}

fn scope_hasher(seed: u64, scope: &RandomDrawScope) -> blake3::Hasher {
    scoped_hasher(
        seed,
        ALGORITHM,
        ALGORITHM_VERSION,
        &scope.draw_namespace,
        &scope.stable_object_id,
        &scope.stream_key,
    )
}

fn scoped_hasher(
    seed: u64,
    algorithm: &str,
    algorithm_version: &str,
    draw_namespace: &str,
    stable_object_id: &str,
    stream_key: &str,
) -> blake3::Hasher {
    let mut hasher = blake3::Hasher::new_derive_key(DERIVE_KEY_CONTEXT);
    hasher.update(&seed.to_le_bytes());
    update_field(&mut hasher, algorithm);
    update_field(&mut hasher, algorithm_version);
    update_field(&mut hasher, draw_namespace);
    update_field(&mut hasher, stable_object_id);
    update_field(&mut hasher, stream_key);
    hasher
}

fn update_field(hasher: &mut blake3::Hasher, value: &str) {
    hasher.update(&(value.len() as u64).to_le_bytes());
    hasher.update(value.as_bytes());
}

fn validate_text(value: &str, field: &'static str) -> Result<(), RandomDrawError> {
    if value.is_empty() {
        Err(RandomDrawError::EmptyField { field })
    } else {
        Ok(())
    }
}

fn uniform_from_bytes(bytes: [u8; 8]) -> f64 {
    ((u64::from_le_bytes(bytes) >> 11) as f64) * F64_UNIT_SCALE
}
