#[path = "../../../src/physics/thermodynamics/color_temperature.rs"]
mod color_temperature_impl;

#[path = "../../../src/rendering/star_material.rs"]
mod star_material_impl;

pub mod physics {
    pub mod thermodynamics {
        pub mod color_temperature {
            pub use crate::color_temperature_impl::*;
        }
    }
}

pub mod rendering {
    pub mod star_material {
        pub use crate::star_material_impl::*;
    }
}
