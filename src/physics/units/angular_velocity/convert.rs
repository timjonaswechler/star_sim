use crate::physics::units::UnitSymbol;
use crate::physics::units::angular_velocity::{
    AngularVelocity, AngularVelocityConvertTo, AngularVelocityUnit, DegreePerSecond,
    RadianPerSecond,
};
use std::fmt;

const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;

impl AngularVelocityConvertTo<RadianPerSecond> for AngularVelocity<DegreePerSecond> {
    fn convert(self) -> AngularVelocity<RadianPerSecond> {
        AngularVelocity::<RadianPerSecond>::new(self.value * DEG_TO_RAD)
    }
}

impl AngularVelocityConvertTo<DegreePerSecond> for AngularVelocity<RadianPerSecond> {
    fn convert(self) -> AngularVelocity<DegreePerSecond> {
        AngularVelocity::<DegreePerSecond>::new(self.value / DEG_TO_RAD)
    }
}

impl<U: AngularVelocityUnit + UnitSymbol> fmt::Display for AngularVelocity<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}
