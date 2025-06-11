use crate::physics::constants::DEG_TO_RAD;
use crate::physics::units::UnitSymbol;
use crate::physics::units::angle::{Angle, AngleConvertTo, AngleUnit, Degree, Radian};
use std::fmt;

impl AngleConvertTo<Radian> for Angle<Degree> {
    fn convert(self) -> Angle<Radian> {
        Angle::<Radian>::new(self.value * DEG_TO_RAD)
    }
}

impl AngleConvertTo<Degree> for Angle<Radian> {
    fn convert(self) -> Angle<Degree> {
        Angle::<Degree>::new(self.value / DEG_TO_RAD)
    }
}

impl<U: AngleUnit + UnitSymbol> fmt::Display for Angle<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}
