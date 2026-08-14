use simulation::{DeterministicDraws, RandomDrawAddress, RandomDrawScope};

fn address(attempt: u32) -> RandomDrawAddress {
    RandomDrawAddress::new(
        "blake3-xof",
        "1",
        "stellar-multiplicity/v1",
        "system-42",
        "has-companion",
        attempt,
    )
    .unwrap()
}

#[test]
fn same_seed_and_random_draw_address_reproduce_the_uniform_draw() {
    let draws = DeterministicDraws::new(42);
    let expected = draws.uniform(&address(0));

    let _unrelated = draws.uniform(&address(1));

    assert_eq!(draws.uniform(&address(0)), expected);
}

#[test]
fn seed_and_random_draw_address_distinguish_uniform_draws() {
    let first = DeterministicDraws::new(42).uniform(&address(0));
    let different_seed = DeterministicDraws::new(43).uniform(&address(0));
    let different_address = DeterministicDraws::new(42).uniform(&address(1));

    let mut distinct = vec![
        first.to_bits(),
        different_seed.to_bits(),
        different_address.to_bits(),
    ];
    distinct.sort_unstable();
    distinct.dedup();

    assert_eq!(distinct.len(), 3);
}

#[test]
fn random_draw_scope_assigns_the_versioned_address_to_each_index() {
    let scope =
        RandomDrawScope::new("stellar-multiplicity/v1", "system-42", "has-companion").unwrap();

    let address = scope.at(7);

    assert_eq!(
        (
            address.algorithm.as_str(),
            address.algorithm_version.as_str(),
            address.prescription_namespace.as_str(),
            address.stable_object_id.as_str(),
            address.claim_key.as_str(),
            address.bounded_attempt_index,
        ),
        (
            "blake3-xof",
            "1",
            "stellar-multiplicity/v1",
            "system-42",
            "has-companion",
            7,
        )
    );
}

#[test]
#[should_panic(expected = "unsupported random draw algorithm")]
fn deterministic_draws_reject_an_address_for_another_algorithm() {
    let mut foreign_address = address(0);
    foreign_address.algorithm = "chacha8".into();

    DeterministicDraws::new(42).uniform(&foreign_address);
}

#[test]
fn blake3_xof_v1_has_a_stable_golden_draw() {
    let draw = DeterministicDraws::new(42).uniform(&address(0));

    assert_eq!(draw.to_bits(), 0x3fb9_19c7_4b01_f438);
}

#[test]
#[should_panic(expected = "draw chunk exceeds the supported size")]
fn prefetched_stream_rejects_an_oversized_chunk_before_starting() {
    let draws = DeterministicDraws::new(42);
    let scope =
        RandomDrawScope::new("stellar-multiplicity/v1", "system-42", "has-companion").unwrap();

    let _stream = draws.prefetched_stream(scope, 1_048_577);
}

#[test]
fn prefetched_stream_matches_direct_draws_across_chunk_boundaries() {
    let draws = DeterministicDraws::new(42);
    let scope =
        RandomDrawScope::new("stellar-multiplicity/v1", "system-42", "has-companion").unwrap();
    let mut stream = draws.prefetched_stream(scope.clone(), 3);

    let streamed: Vec<_> = (0..10)
        .map(|_| stream.next_uniform().expect("draw stream is not exhausted"))
        .collect();
    let direct: Vec<_> = (0..10)
        .map(|index| draws.uniform(&scope.at(index)))
        .collect();

    assert_eq!(streamed, direct);
}
