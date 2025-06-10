use crate::physics::units::UnitSymbol;
use crate::physics::units::energy::{Energy, EnergyConvertTo, EnergyUnit, Joule};
use std::fmt;

// const JOULES_PER_KWH: f64 = 3_600_000.0;
// const KWH_PER_JOULE: f64 = 1.0 / JOULES_PER_KWH;

// impl EnergyConvertTo<Joule> for Energy<KilowattHour> {
//     fn convert(self) -> Energy<Joule> {
//         Energy::<Joule>::new(self.value * JOULES_PER_KWH)
//     }
// }

// impl EnergyConvertTo<KilowattHour> for Energy<Joule> {
//     fn convert(self) -> Energy<KilowattHour> {
//         Energy::<KilowattHour>::new(self.value * KWH_PER_JOULE)
//     }
// }

impl<U: EnergyUnit + UnitSymbol> fmt::Display for Energy<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}
