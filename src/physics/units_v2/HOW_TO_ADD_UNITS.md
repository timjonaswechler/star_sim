# How to Add New Units to the Units_v2 System

This guide explains how to extend the units_v2 system with new units and quantities. The system is designed to make adding new units as simple as possible while maintaining type safety and performance.

## Quick Start: Adding a New Unit to an Existing Dimension

If you want to add a new unit to an existing dimension (like Distance, Mass, Time), follow these steps:

### Example: Adding Light-Second to Distance

1. **Find the conversion factor** to the SI base unit (meters):
   ```rust
   // Light travels 299,792,458 meters per second
   const METERS_PER_LIGHT_SECOND: f64 = 299_792_458.0;
   ```

2. **Add the constant** to `constants.rs`:
   ```rust
   /// Light second to meters.
   /// 
   /// Distance light travels in one second in vacuum.
   pub const METERS_PER_LIGHT_SECOND: f64 = 299_792_458.0;
   ```

3. **Update the macro call** in `dimensions.rs`:
   ```rust
   define_unit_dimension! {
       dimension Distance {
           base_unit: Meter = 1.0,
           units: {
               Meter = 1.0,
               Kilometer = 1000.0,
               AstronomicalUnit = METERS_PER_AU,
               EarthRadius = METERS_PER_EARTH_RADIUS,
               SunRadius = METERS_PER_SUN_RADIUS,
               LightYear = METERS_PER_LIGHT_YEAR,
               Parsec = METERS_PER_PARSEC,
               LightSecond = METERS_PER_LIGHT_SECOND,  // ← Add this line
           },
           symbols: {
               Meter = "m",
               Kilometer = "km",
               AstronomicalUnit = "AU",
               EarthRadius = "R⊕",
               SunRadius = "R☉",
               LightYear = "ly",
               Parsec = "pc",
               LightSecond = "ls",  // ← Add this line
           }
       }
   }
   ```

4. **Use your new unit**:
   ```rust
   let distance = Distance::<LightSecond>::new(8.3); // 8.3 light-seconds to Sun
   let in_au = distance.convert_to::<AstronomicalUnit>();
   ```

That's it! The macro automatically generates all the necessary conversion code.

## Adding a Completely New Physical Dimension

For a new physical dimension (like Electric Resistance, Magnetic Field, etc.), you need more steps:

### Example: Adding Electric Resistance (Ohms)

1. **Add constants** to `constants.rs`:
   ```rust
   // ================================================================================================
   // RESISTANCE CONVERSIONS (to Ohms)
   // ================================================================================================

   /// Ohm to Ohms (base unit).
   pub const OHMS_PER_OHM: f64 = 1.0;

   /// Kiloohm to Ohms.
   pub const OHMS_PER_KILOOHM: f64 = 1000.0;

   /// Megaohm to Ohms.
   pub const OHMS_PER_MEGAOHM: f64 = 1_000_000.0;
   ```

2. **Define the quantity type** in `dimensions.rs`:
   ```rust
   // Electric resistance has dimensions [Length² Mass Time⁻³ Current⁻²]
   define_quantity!(Resistance, 2, 1, -3, 0, -2, 0, 0);
   ```

3. **Define the unit system**:
   ```rust
   define_unit_dimension! {
       dimension Resistance {
           base_unit: Ohm = 1.0,
           units: {
               Ohm = OHMS_PER_OHM,
               Kiloohm = OHMS_PER_KILOOHM,
               Megaohm = OHMS_PER_MEGAOHM,
           },
           symbols: {
               Ohm = "Ω",
               Kiloohm = "kΩ",
               Megaohm = "MΩ",
           }
       }
   }
   ```

4. **Export the new types** in the module's `pub use` statements:
   ```rust
   pub use dimensions::{
       // ... existing exports ...
       Resistance, Ohm, Kiloohm, Megaohm,
   };
   ```

5. **Use your new dimension**:
   ```rust
   let resistance = Resistance::<Kiloohm>::new(2.7);
   let in_ohms = resistance.convert_to::<Ohm>();
   ```

## Advanced: Dimensional Analysis

When you define a quantity with the correct dimensional exponents, the system can catch dimensional errors at compile time:

```rust
// Define derived quantities with proper dimensions
define_quantity!(Capacitance, -2, -1, 4, 0, 2, 0, 0);  // Length⁻² Mass⁻¹ Time⁴ Current²

// The compiler will prevent nonsensical operations
let resistance = Resistance::<Ohm>::new(100.0);
let capacitance = Capacitance::<Farad>::new(0.001);
// let invalid = resistance + capacitance;  // ← Compile error!
```

## Dimensional Exponents Reference

When defining new quantities, use these dimensional exponents:

| Dimension | Symbol | Index | Examples |
|-----------|--------|-------|----------|
| Length | L | 0 | Distance (1), Area (2), Volume (3) |
| Mass | M | 1 | Mass (1), Density (1) with Length (-3) |
| Time | T | 2 | Time (1), Frequency (-1), Acceleration (-2) |
| Temperature | K | 3 | Temperature (1) |
| Current | I | 4 | Current (1), Charge (1) with Time (1) |
| Luminous Intensity | J | 5 | Luminous Intensity (1) |
| Amount of Substance | N | 6 | Amount (1) |

### Common Physical Quantities

| Quantity | Dimensions | Exponents |
|----------|------------|-----------|
| Area | Length² | (2, 0, 0, 0, 0, 0, 0) |
| Volume | Length³ | (3, 0, 0, 0, 0, 0, 0) |
| Velocity | Length/Time | (1, 0, -1, 0, 0, 0, 0) |
| Acceleration | Length/Time² | (1, 0, -2, 0, 0, 0, 0) |
| Force | Mass×Length/Time² | (1, 1, -2, 0, 0, 0, 0) |
| Energy | Mass×Length²/Time² | (2, 1, -2, 0, 0, 0, 0) |
| Power | Mass×Length²/Time³ | (2, 1, -3, 0, 0, 0, 0) |
| Pressure | Mass/(Length×Time²) | (-1, 1, -2, 0, 0, 0, 0) |
| Frequency | 1/Time | (0, 0, -1, 0, 0, 0, 0) |
| Electric Charge | Current×Time | (0, 0, 1, 0, 1, 0, 0) |
| Voltage | Mass×Length²/(Time³×Current) | (2, 1, -3, 0, -1, 0, 0) |
| Resistance | Mass×Length²/(Time³×Current²) | (2, 1, -3, 0, -2, 0, 0) |
| Capacitance | Current²×Time⁴/(Mass×Length²) | (-2, -1, 4, 0, 2, 0, 0) |

## Testing Your New Units

After adding new units, create tests to verify they work correctly:

```rust
#[test]
fn test_light_second_conversion() {
    let distance = Distance::<LightSecond>::new(1.0);
    let in_meters = distance.convert_to::<Meter>();
    assert!((in_meters.value() - 299_792_458.0).abs() < 1.0);
}

#[test]
fn test_resistance_conversion() {
    let resistance = Resistance::<Kiloohm>::new(2.7);
    let in_ohms = resistance.convert_to::<Ohm>();
    assert!((in_ohms.value() - 2700.0).abs() < f64::EPSILON);
}
```

## Best Practices

1. **Use authoritative sources** for conversion factors (IAU, NIST, etc.)
2. **Document the source** and precision of constants
3. **Use Unicode symbols** when appropriate (⊕, ☉, Ω, etc.)
4. **Test conversions** thoroughly, especially for astronomy constants
5. **Group related units** together in the macro calls
6. **Follow naming conventions**: `TypeName` for types, `CONSTANT_NAME` for constants

## Performance Notes

The hub-and-spoke conversion system means:
- **Adding N units requires 2N conversions** (not N² conversions)
- **Runtime overhead is minimal** (just multiplication/division)
- **Compile-time safety is maximum** (dimensional analysis prevents errors)

This makes the system both fast and safe while remaining easy to extend!