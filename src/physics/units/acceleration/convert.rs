use crate::physics::units::acceleration::{
    Acceleration, AccelerationConvertTo, AccelerationUnit, MeterPerSecondSquared, StandardGravity,
};
use crate::physics::units::UnitSymbol;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

const MPS2_PER_G: f64 = 9.80665;
const G_PER_MPS2: f64 = 1.0 / MPS2_PER_G;

impl AccelerationConvertTo<MeterPerSecondSquared> for Acceleration<StandardGravity> {
    fn convert(self) -> Acceleration<MeterPerSecondSquared> {
        Acceleration::<MeterPerSecondSquared>::new(self.value * MPS2_PER_G)
    }
}

impl AccelerationConvertTo<StandardGravity> for Acceleration<MeterPerSecondSquared> {
    fn convert(self) -> Acceleration<StandardGravity> {
        Acceleration::<StandardGravity>::new(self.value * G_PER_MPS2)
    }
}

impl<U: AccelerationUnit + UnitSymbol> fmt::Display for Acceleration<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}

impl<U: AccelerationUnit> Add for Acceleration<U> {
    type Output = Acceleration<U>;
    fn add(self, other: Acceleration<U>) -> Acceleration<U> {
        Acceleration::new(self.value + other.value)
    }
}

impl<U: AccelerationUnit> Sub for Acceleration<U> {
    type Output = Acceleration<U>;
    fn sub(self, other: Acceleration<U>) -> Acceleration<U> {
        Acceleration::new(self.value - other.value)
    }
}

impl<U: AccelerationUnit> Mul<f64> for Acceleration<U> {
    type Output = Acceleration<U>;
    fn mul(self, scalar: f64) -> Acceleration<U> {
        Acceleration::new(self.value * scalar)
    }
}

impl<U: AccelerationUnit> Div<f64> for Acceleration<U> {
    type Output = Acceleration<U>;
    fn div(self, scalar: f64) -> Acceleration<U> {
        Acceleration::new(self.value / scalar)
    }
}
