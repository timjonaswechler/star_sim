use crate::physics::units::UnitSymbol;
use crate::physics::units::angular_acceleration::{
    AngularAcceleration, AngularAccelerationConvertTo, AngularAccelerationUnit,
    DegreePerSecondSquared, RadianPerSecondSquared,
};
use std::fmt;

const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;

impl AngularAccelerationConvertTo<RadianPerSecondSquared>
    for AngularAcceleration<DegreePerSecondSquared>
{
    fn convert(self) -> AngularAcceleration<RadianPerSecondSquared> {
        AngularAcceleration::<RadianPerSecondSquared>::new(self.value * DEG_TO_RAD)
    }
}

impl AngularAccelerationConvertTo<DegreePerSecondSquared>
    for AngularAcceleration<RadianPerSecondSquared>
{
    fn convert(self) -> AngularAcceleration<DegreePerSecondSquared> {
        AngularAcceleration::<DegreePerSecondSquared>::new(self.value / DEG_TO_RAD)
    }
}

impl<U: AngularAccelerationUnit + UnitSymbol> fmt::Display for AngularAcceleration<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}
