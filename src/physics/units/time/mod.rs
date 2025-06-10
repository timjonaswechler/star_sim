const SECONDS_PER_MINUTE: f64 = 60.0;
const MINUTES_PER_HOUR: f64 = 60.0;
const HOURS_PER_DAY: f64 = 24.0;
const DAYS_PER_YEAR: f64 = 365.25;

pub mod convert;
pub mod operations;
pub mod prefix;
pub mod traits;

pub use convert::*;
pub use operations::*;
pub use prefix::*;
pub use traits::*;
