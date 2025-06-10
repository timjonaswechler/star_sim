// Marker traits
pub trait TimeUnit {}

// Time unit types
pub struct Second;
pub struct Minute;
pub struct Hour;
pub struct Day;
pub struct Year;
pub type Megayear = Prefixed<Mega, Year>;
pub type Gigayear = Prefixed<Giga, Year>;

impl TimeUnit for Second {}
impl TimeUnit for Minute {}
impl TimeUnit for Hour {}
impl TimeUnit for Day {}
impl TimeUnit for Year {}

impl UnitSymbol for Second {
    fn symbol() -> &'static str {
        "s"
    }
}
impl UnitSymbol for Minute {
    fn symbol() -> &'static str {
        "min"
    }
}
impl UnitSymbol for Hour {
    fn symbol() -> &'static str {
        "h"
    }
}
impl UnitSymbol for Day {
    fn symbol() -> &'static str {
        "d"
    }
}
impl UnitSymbol for Year {
    fn symbol() -> &'static str {
        "yr"
    }
}

// Quantity structs
#[derive(Debug, Clone, Copy)]
pub struct Time<U: TimeUnit> {
    pub value: f64,
    _unit: PhantomData<U>,
}

// Basic constructors and accessors
impl<U: TimeUnit> Time<U> {
    pub fn new(value: f64) -> Self {
        Time {
            value,
            _unit: PhantomData,
        }
    }
    pub fn value(&self) -> f64 {
        self.value
    }
}

// Conversion traits
pub trait TimeConvertTo<V: TimeUnit> {
    fn convert(self) -> Time<V>;
}

// Generic get methods
impl<U: TimeUnit> Time<U> {
    pub fn get<V: TimeUnit>(self) -> Time<V>
    where
        Self: TimeConvertTo<V>,
    {
        self.convert()
    }
}
