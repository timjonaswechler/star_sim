use super::objects::TrojanObject;
use crate::physics::units::{Distance, Kilogram, Mass, Meter, Time, Year};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Builder für [`TrojanObject`]
pub struct TrojanBuilder {
    seed: u64,
    mass: Option<Mass<Kilogram>>,
    lagrange_point: Option<u8>,
    amplitude: Option<Distance<Meter>>,
    period: Option<Time<Year>>,
}

impl TrojanBuilder {
    pub fn new() -> Self {
        Self {
            seed: 0,
            mass: None,
            lagrange_point: None,
            amplitude: None,
            period: None,
        }
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    pub fn with_mass(mut self, mass: Mass<Kilogram>) -> Self {
        self.mass = Some(mass);
        self
    }

    pub fn with_lagrange_point(mut self, point: u8) -> Self {
        self.lagrange_point = Some(point);
        self
    }

    pub fn with_amplitude(mut self, amp: Distance<Meter>) -> Self {
        self.amplitude = Some(amp);
        self
    }

    pub fn with_period(mut self, period: Time<Year>) -> Self {
        self.period = Some(period);
        self
    }

    pub fn build(self) -> TrojanObject {
        let mut rng = ChaCha8Rng::seed_from_u64(self.seed);
        let mass = self
            .mass
            .unwrap_or_else(|| Mass::earth_masses(rng.gen_range(0.0001..0.01)));
        let lagrange_point = self
            .lagrange_point
            .unwrap_or(if rng.gen_bool(0.5) { 4 } else { 5 });
        let amplitude = self
            .amplitude
            .unwrap_or_else(|| Distance::au(rng.gen_range(0.001..0.1)));
        let period = self
            .period
            .unwrap_or_else(|| Time::years(rng.gen_range(1e0..1e3)));
        TrojanObject {
            mass,
            lagrange_point,
            oscillation_amplitude: amplitude,
            oscillation_period: period,
            stability: rng.gen_range(0.5..1.0),
        }
    }
}
