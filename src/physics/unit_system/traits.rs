/// Trait implemented by all unit enums. Provides conversion
/// to and from the associated SI base unit.
pub trait Unit: Copy {
    fn to_base(self, value: f64) -> f64;
    fn from_base(self, value: f64) -> f64;
    fn symbol(self) -> &'static str;
}
