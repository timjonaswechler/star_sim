use crate::physics::units::{constants::*, core::*};
use crate::{define_quantity, define_unit_dimension};

define_quantity!(Energy, 2, 1, -2, 0, 0, 0, 0); // Mass×Length²/Time²// Define Energy units

define_unit_dimension! {
    dimension Energy {
        base_unit: Joule = 1.0,
        units: {
            Joule = 1.0,
            Erg = JOULES_PER_ERG,
            ElectronVolt = JOULES_PER_EV,
        },
        symbols: {
            Joule = "J",
            Erg = "erg",
            ElectronVolt = "eV",
        }
    }
}
