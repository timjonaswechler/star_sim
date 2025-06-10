const METER_PER_SECONDS_IN_KILOMETER_PER_HOUR: f64 = 1000.0 / 3600.0;

pub mod convert;
pub mod operations;
pub mod prefix;
pub mod traits;

pub use convert::*;
pub use operations::*;
pub use prefix::*;
pub use traits::*;
