use crate::physics::units::energy::{Energy, EnergyConvertTo, EnergyUnit, Joule, KilowattHour};
use crate::physics::units::UnitSymbol;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

const JOULES_PER_KWH: f64 = 3_600_000.0;
const KWH_PER_JOULE: f64 = 1.0 / JOULES_PER_KWH;

impl EnergyConvertTo<Joule> for Energy<KilowattHour> {
    fn convert(self) -> Energy<Joule> {
        Energy::<Joule>::new(self.value * JOULES_PER_KWH)
    }
}

impl EnergyConvertTo<KilowattHour> for Energy<Joule> {
    fn convert(self) -> Energy<KilowattHour> {
        Energy::<KilowattHour>::new(self.value * KWH_PER_JOULE)
    }
}

impl<U: EnergyUnit + UnitSymbol> fmt::Display for Energy<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}

impl<U: EnergyUnit> Add for Energy<U> {
    type Output = Energy<U>;
    fn add(self, other: Energy<U>) -> Energy<U> {
        Energy::new(self.value + other.value)
    }
}

impl<U: EnergyUnit> Sub for Energy<U> {
    type Output = Energy<U>;
    fn sub(self, other: Energy<U>) -> Energy<U> {
        Energy::new(self.value - other.value)
    }
}

impl<U: EnergyUnit> Mul<f64> for Energy<U> {
    type Output = Energy<U>;
    fn mul(self, scalar: f64) -> Energy<U> {
        Energy::new(self.value * scalar)
    }
}

impl<U: EnergyUnit> Div<f64> for Energy<U> {
    type Output = Energy<U>;
    fn div(self, scalar: f64) -> Energy<U> {
        Energy::new(self.value / scalar)
    }
}
