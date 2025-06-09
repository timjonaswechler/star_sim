use std::fmt;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};

// Marker traits
pub trait TimeUnit {}
pub trait LengthUnit {}
pub trait MassUnit {}
pub trait TemperatureUnit {}
pub trait VelocityUnit {}
pub trait ForceUnit {}

// Time unit types
pub struct Second;
pub struct Minute;
pub struct Hour;
pub struct Day;
pub struct Year;
pub struct Kiloyear;
pub struct Megayear;
pub struct Gigayear;

impl TimeUnit for Second {}
impl TimeUnit for Minute {}
impl TimeUnit for Hour {}
impl TimeUnit for Day {}
impl TimeUnit for Year {}
impl TimeUnit for Kiloyear {}
impl TimeUnit for Megayear {}
impl TimeUnit for Gigayear {}

// Length unit types
pub struct Meter;
pub struct Kilometer;
pub struct Centimeter;
pub struct Millimeter;
pub struct Inch;
pub struct Foot;
pub struct Mile;

impl LengthUnit for Meter {}
impl LengthUnit for Kilometer {}
impl LengthUnit for Centimeter {}
impl LengthUnit for Millimeter {}
impl LengthUnit for Inch {}
impl LengthUnit for Foot {}
impl LengthUnit for Mile {}

// Mass unit types
pub struct Kilogram;
pub struct Gram;
pub struct Pound;
pub struct Ounce;
pub struct Ton;

impl MassUnit for Kilogram {}
impl MassUnit for Gram {}
impl MassUnit for Pound {}
impl MassUnit for Ounce {}
impl MassUnit for Ton {}

// Temperature unit types
pub struct Celsius;
pub struct Fahrenheit;
pub struct Kelvin;

impl TemperatureUnit for Celsius {}
impl TemperatureUnit for Fahrenheit {}
impl TemperatureUnit for Kelvin {}

// Velocity unit types
pub struct MeterPerSecond;
pub struct KilometerPerHour;

impl VelocityUnit for MeterPerSecond {}
impl VelocityUnit for KilometerPerHour {}

// Quantity structs
#[derive(Debug, Clone, Copy)]
pub struct Time<U: TimeUnit> {
    value: f64,
    _unit: PhantomData<U>,
}

#[derive(Debug, Clone, Copy)]
pub struct Length<U: LengthUnit> {
    value: f64,
    _unit: PhantomData<U>,
}

#[derive(Debug, Clone, Copy)]
pub struct Mass<U: MassUnit> {
    value: f64,
    _unit: PhantomData<U>,
}

#[derive(Debug, Clone, Copy)]
pub struct Temperature<U: TemperatureUnit> {
    value: f64,
    _unit: PhantomData<U>,
}

#[derive(Debug, Clone, Copy)]
pub struct Velocity<U: VelocityUnit> {
    value: f64,
    _unit: PhantomData<U>,
}

// Basic constructors and accessors
impl<U: TimeUnit> Time<U> {
    pub fn new(value: f64) -> Self {
        Time { value, _unit: PhantomData }
    }
    pub fn value(&self) -> f64 { self.value }
}

impl<U: LengthUnit> Length<U> {
    pub fn new(value: f64) -> Self {
        Length { value, _unit: PhantomData }
    }
    pub fn value(&self) -> f64 { self.value }
}

impl<U: MassUnit> Mass<U> {
    pub fn new(value: f64) -> Self {
        Mass { value, _unit: PhantomData }
    }
    pub fn value(&self) -> f64 { self.value }
}

impl<U: TemperatureUnit> Temperature<U> {
    pub fn new(value: f64) -> Self {
        Temperature { value, _unit: PhantomData }
    }
    pub fn value(&self) -> f64 { self.value }
}

impl<U: VelocityUnit> Velocity<U> {
    pub fn new(value: f64) -> Self {
        Velocity { value, _unit: PhantomData }
    }
    pub fn value(&self) -> f64 { self.value }
}

// Conversion traits
pub trait TimeConvertTo<V: TimeUnit> {
    fn convert(self) -> Time<V>;
}

pub trait LengthConvertTo<V: LengthUnit> {
    fn convert(self) -> Length<V>;
}

pub trait MassConvertTo<V: MassUnit> {
    fn convert(self) -> Mass<V>;
}

pub trait TemperatureConvertTo<V: TemperatureUnit> {
    fn convert(self) -> Temperature<V>;
}

pub trait VelocityConvertTo<V: VelocityUnit> {
    fn convert(self) -> Velocity<V>;
}

// conversion constants
const SECONDS_PER_MINUTE: f64 = 60.0;
const SECONDS_PER_HOUR: f64 = 3600.0;
const SECONDS_PER_DAY: f64 = 86400.0;
const SECONDS_PER_YEAR: f64 = 31_536_000.0;
const SECONDS_PER_KILOYEAR: f64 = SECONDS_PER_YEAR * 1000.0;
const SECONDS_PER_MEGAYEAR: f64 = SECONDS_PER_YEAR * 1_000_000.0;
const SECONDS_PER_GIGAYEAR: f64 = SECONDS_PER_YEAR * 1_000_000_000.0;

const METERS_PER_KILOMETER: f64 = 1000.0;
const METERS_PER_CENTIMETER: f64 = 0.01;
const METERS_PER_MILLIMETER: f64 = 0.001;
const METERS_PER_INCH: f64 = 0.0254;
const METERS_PER_FOOT: f64 = 0.3048;
const METERS_PER_MILE: f64 = 1609.344;

const KILOGRAMS_PER_GRAM: f64 = 0.001;
const KILOGRAMS_PER_POUND: f64 = 0.453592;
const KILOGRAMS_PER_OUNCE: f64 = 0.0283495;
const KILOGRAMS_PER_TON: f64 = 1000.0;

fn celsius_to_kelvin(c: f64) -> f64 { c + 273.15 }
fn kelvin_to_celsius(k: f64) -> f64 { k - 273.15 }
fn celsius_to_fahrenheit(c: f64) -> f64 { c * 9.0/5.0 + 32.0 }
fn fahrenheit_to_celsius(f: f64) -> f64 { (f - 32.0) * 5.0/9.0 }

// Time conversions
impl TimeConvertTo<Second> for Time<Minute> {
    fn convert(self) -> Time<Second> {
        Time::<Second>::new(self.value * SECONDS_PER_MINUTE)
    }
}

impl TimeConvertTo<Minute> for Time<Second> {
    fn convert(self) -> Time<Minute> {
        Time::<Minute>::new(self.value / SECONDS_PER_MINUTE)
    }
}

impl TimeConvertTo<Second> for Time<Hour> {
    fn convert(self) -> Time<Second> {
        Time::<Second>::new(self.value * SECONDS_PER_HOUR)
    }
}

impl TimeConvertTo<Hour> for Time<Second> {
    fn convert(self) -> Time<Hour> {
        Time::<Hour>::new(self.value / SECONDS_PER_HOUR)
    }
}

// Length conversions
impl LengthConvertTo<Meter> for Length<Kilometer> {
    fn convert(self) -> Length<Meter> {
        Length::<Meter>::new(self.value * METERS_PER_KILOMETER)
    }
}

impl LengthConvertTo<Kilometer> for Length<Meter> {
    fn convert(self) -> Length<Kilometer> {
        Length::<Kilometer>::new(self.value / METERS_PER_KILOMETER)
    }
}

impl LengthConvertTo<Meter> for Length<Centimeter> {
    fn convert(self) -> Length<Meter> {
        Length::<Meter>::new(self.value * METERS_PER_CENTIMETER)
    }
}

impl LengthConvertTo<Centimeter> for Length<Meter> {
    fn convert(self) -> Length<Centimeter> {
        Length::<Centimeter>::new(self.value / METERS_PER_CENTIMETER)
    }
}

// Mass conversions
impl MassConvertTo<Kilogram> for Mass<Gram> {
    fn convert(self) -> Mass<Kilogram> {
        Mass::<Kilogram>::new(self.value * KILOGRAMS_PER_GRAM)
    }
}

impl MassConvertTo<Gram> for Mass<Kilogram> {
    fn convert(self) -> Mass<Gram> {
        Mass::<Gram>::new(self.value / KILOGRAMS_PER_GRAM)
    }
}

impl MassConvertTo<Kilogram> for Mass<Pound> {
    fn convert(self) -> Mass<Kilogram> {
        Mass::<Kilogram>::new(self.value * KILOGRAMS_PER_POUND)
    }
}

impl MassConvertTo<Pound> for Mass<Kilogram> {
    fn convert(self) -> Mass<Pound> {
        Mass::<Pound>::new(self.value / KILOGRAMS_PER_POUND)
    }
}

// Temperature conversions
impl TemperatureConvertTo<Kelvin> for Temperature<Celsius> {
    fn convert(self) -> Temperature<Kelvin> {
        Temperature::<Kelvin>::new(celsius_to_kelvin(self.value))
    }
}

impl TemperatureConvertTo<Celsius> for Temperature<Kelvin> {
    fn convert(self) -> Temperature<Celsius> {
        Temperature::<Celsius>::new(kelvin_to_celsius(self.value))
    }
}

impl TemperatureConvertTo<Fahrenheit> for Temperature<Celsius> {
    fn convert(self) -> Temperature<Fahrenheit> {
        Temperature::<Fahrenheit>::new(celsius_to_fahrenheit(self.value))
    }
}

impl TemperatureConvertTo<Celsius> for Temperature<Fahrenheit> {
    fn convert(self) -> Temperature<Celsius> {
        Temperature::<Celsius>::new(fahrenheit_to_celsius(self.value))
    }
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

impl<U: LengthUnit> Length<U> {
    pub fn get<V: LengthUnit>(self) -> Length<V>
    where
        Self: LengthConvertTo<V>,
    {
        self.convert()
    }
}

impl<U: MassUnit> Mass<U> {
    pub fn get<V: MassUnit>(self) -> Mass<V>
    where
        Self: MassConvertTo<V>,
    {
        self.convert()
    }
}

impl<U: TemperatureUnit> Temperature<U> {
    pub fn get<V: TemperatureUnit>(self) -> Temperature<V>
    where
        Self: TemperatureConvertTo<V>,
    {
        self.convert()
    }
}

impl<U: VelocityUnit> Velocity<U> {
    pub fn get<V: VelocityUnit>(self) -> Velocity<V>
    where
        Self: VelocityConvertTo<V>,
    {
        self.convert()
    }
}

// Unit symbols
pub trait UnitSymbol {
    fn symbol() -> &'static str;
}

impl UnitSymbol for Second { fn symbol() -> &'static str { "s" } }
impl UnitSymbol for Minute { fn symbol() -> &'static str { "min" } }
impl UnitSymbol for Hour { fn symbol() -> &'static str { "h" } }
impl UnitSymbol for Day { fn symbol() -> &'static str { "d" } }
impl UnitSymbol for Year { fn symbol() -> &'static str { "yr" } }

impl UnitSymbol for Meter { fn symbol() -> &'static str { "m" } }
impl UnitSymbol for Kilometer { fn symbol() -> &'static str { "km" } }
impl UnitSymbol for Centimeter { fn symbol() -> &'static str { "cm" } }

impl UnitSymbol for Kilogram { fn symbol() -> &'static str { "kg" } }
impl UnitSymbol for Gram { fn symbol() -> &'static str { "g" } }
impl UnitSymbol for Pound { fn symbol() -> &'static str { "lb" } }

impl UnitSymbol for Celsius { fn symbol() -> &'static str { "°C" } }
impl UnitSymbol for Fahrenheit { fn symbol() -> &'static str { "°F" } }
impl UnitSymbol for Kelvin { fn symbol() -> &'static str { "K" } }

// Display implementations
impl<U: TimeUnit + UnitSymbol> fmt::Display for Time<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}

impl<U: LengthUnit + UnitSymbol> fmt::Display for Length<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}

impl<U: MassUnit + UnitSymbol> fmt::Display for Mass<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}

impl<U: TemperatureUnit + UnitSymbol> fmt::Display for Temperature<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}

// Basic math operations
impl<U: TimeUnit> Add for Time<U> {
    type Output = Time<U>;
    fn add(self, other: Time<U>) -> Time<U> {
        Time::new(self.value + other.value)
    }
}

impl<U: LengthUnit> Add for Length<U> {
    type Output = Length<U>;
    fn add(self, other: Length<U>) -> Length<U> {
        Length::new(self.value + other.value)
    }
}

impl<U: TimeUnit> Mul<f64> for Time<U> {
    type Output = Time<U>;
    fn mul(self, scalar: f64) -> Time<U> {
        Time::new(self.value * scalar)
    }
}

impl<U: LengthUnit> Mul<f64> for Length<U> {
    type Output = Length<U>;
    fn mul(self, scalar: f64) -> Length<U> {
        Length::new(self.value * scalar)
    }
}
