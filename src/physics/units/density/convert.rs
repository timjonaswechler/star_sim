use crate::physics::units::UnitSymbol;
use crate::physics::units::density::{
    Density, DensityConvertTo, DensityUnit, GramPerCubicCentimeter, KilogramPerCubicMeter,
};
use std::fmt;

const KG_PER_M3_PER_G_PER_CM3: f64 = 1000.0;

impl DensityConvertTo<KilogramPerCubicMeter> for Density<GramPerCubicCentimeter> {
    fn convert(self) -> Density<KilogramPerCubicMeter> {
        Density::<KilogramPerCubicMeter>::new(self.value * KG_PER_M3_PER_G_PER_CM3)
    }
}

impl DensityConvertTo<GramPerCubicCentimeter> for Density<KilogramPerCubicMeter> {
    fn convert(self) -> Density<GramPerCubicCentimeter> {
        Density::<GramPerCubicCentimeter>::new(self.value / KG_PER_M3_PER_G_PER_CM3)
    }
}

impl<U: DensityUnit + UnitSymbol> fmt::Display for Density<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}
