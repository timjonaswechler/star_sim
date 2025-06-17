//! Examples demonstrating the prefix system with all unit types.
//!
//! This example shows how the prefix system works with every unit type
//! in the units_v2 system, creating combinations like:
//! - Distance: nm, μm, mm, cm, km, Mm, Gm
//! - Mass: mg, g, kg, Mg, Gg  
//! - Time: ns, μs, ms, s, ks, Ms, Gs
//! - Energy: mJ, J, kJ, MJ, GJ
//! - Power: mW, W, kW, MW, GW
//! - And any other unit type with any prefix!

use star_sim::physics::units_v2::*;

fn main() {
    println!("=== Prefix System Examples ===\n");
    
    // ===============================
    // DISTANCE UNITS WITH PREFIXES
    // ===============================
    println!("📏 Distance Units with Prefixes:");
    
    let nanometer = Distance::<Prefixed<Nano, Meter>>::new(500.0);
    let micrometer = Distance::<Prefixed<Micro, Meter>>::new(25.0);
    let millimeter = Distance::<Prefixed<Milli, Meter>>::new(15.5);
    let centimeter = Distance::<Prefixed<Centi, Meter>>::new(12.3);
    let meter = Distance::<Meter>::new(1.0);
    let kilometer = Distance::<Prefixed<Kilo, Meter>>::new(5.2);
    let megameter = Distance::<Prefixed<Mega, Meter>>::new(150.0);
    let gigameter = Distance::<Prefixed<Giga, Meter>>::new(1.5);
    
    println!("  {} nm = {:.2e} m", nanometer.value, nanometer.convert_to::<Meter>().value);
    println!("  {} μm = {:.2e} m", micrometer.value, micrometer.convert_to::<Meter>().value);
    println!("  {} mm = {} m", millimeter.value, millimeter.convert_to::<Meter>().value);
    println!("  {} cm = {} m", centimeter.value, centimeter.convert_to::<Meter>().value);
    println!("  {} m = {} m", meter.value, meter.value);
    println!("  {} km = {} m", kilometer.value, kilometer.convert_to::<Meter>().value);
    println!("  {} Mm = {:.0e} m", megameter.value, megameter.convert_to::<Meter>().value);
    println!("  {} Gm = {:.0e} m", gigameter.value, gigameter.convert_to::<Meter>().value);
    
    // ===============================
    // MASS UNITS WITH PREFIXES
    // ===============================
    println!("\n⚖️  Mass Units with Prefixes:");
    
    let milligram = Mass::<Prefixed<Milli, Gram>>::new(250.0);
    let gram = Mass::<Gram>::new(1000.0);
    let kilogram = Mass::<Prefixed<Kilo, Gram>>::new(70.0);
    let megagram = Mass::<Prefixed<Mega, Gram>>::new(5.0);
    let gigagram = Mass::<Prefixed<Giga, Gram>>::new(2.0);
    
    println!("  {} mg = {} kg", milligram.value, milligram.convert_to::<Kilogram>().value);
    println!("  {} g = {} kg", gram.value, gram.convert_to::<Kilogram>().value);
    println!("  {} kg = {} kg", kilogram.value, kilogram.convert_to::<Kilogram>().value);
    println!("  {} Mg = {} kg", megagram.value, megagram.convert_to::<Kilogram>().value);
    println!("  {} Gg = {:.0e} kg", gigagram.value, gigagram.convert_to::<Kilogram>().value);
    
    // ===============================
    // TIME UNITS WITH PREFIXES
    // ===============================
    println!("\n⏰ Time Units with Prefixes:");
    
    let nanosecond = Time::<Prefixed<Nano, Second>>::new(100.0);
    let microsecond = Time::<Prefixed<Micro, Second>>::new(500.0);
    let millisecond = Time::<Prefixed<Milli, Second>>::new(250.0);
    let second = Time::<Second>::new(1.0);
    let kilosecond = Time::<Prefixed<Kilo, Second>>::new(1.0);
    let megasecond = Time::<Prefixed<Mega, Second>>::new(31.6); // ~1 year
    let gigasecond = Time::<Prefixed<Giga, Second>>::new(31.7); // ~31.7 years
    
    println!("  {} ns = {:.0e} s", nanosecond.value, nanosecond.convert_to::<Second>().value);
    println!("  {} μs = {:.0e} s", microsecond.value, microsecond.convert_to::<Second>().value);
    println!("  {} ms = {} s", millisecond.value, millisecond.convert_to::<Second>().value);
    println!("  {} s = {} s", second.value, second.value);
    println!("  {} ks = {} s", kilosecond.value, kilosecond.convert_to::<Second>().value);
    println!("  {} Ms = {:.0e} s (~{:.1} years)", megasecond.value, megasecond.convert_to::<Second>().value, megasecond.convert_to::<Year>().value);
    println!("  {} Gs = {:.0e} s (~{:.1} years)", gigasecond.value, gigasecond.convert_to::<Second>().value, gigasecond.convert_to::<Year>().value);
    
    // ===============================
    // ENERGY UNITS WITH PREFIXES
    // ===============================
    println!("\n⚡ Energy Units with Prefixes:");
    
    let millijoule = Energy::<Prefixed<Milli, Joule>>::new(500.0);
    let joule = Energy::<Joule>::new(1000.0);
    let kilojoule = Energy::<Prefixed<Kilo, Joule>>::new(4.2); // ~1 food calorie
    let megajoule = Energy::<Prefixed<Mega, Joule>>::new(3.6); // 1 kWh
    let gigajoule = Energy::<Prefixed<Giga, Joule>>::new(1.0);
    let terajoule = Energy::<Prefixed<Tera, Joule>>::new(1.0);
    
    println!("  {} mJ = {} J", millijoule.value, millijoule.convert_to::<Joule>().value);
    println!("  {} J = {} J", joule.value, joule.value);
    println!("  {} kJ = {} J", kilojoule.value, kilojoule.convert_to::<Joule>().value);
    println!("  {} MJ = {:.0e} J", megajoule.value, megajoule.convert_to::<Joule>().value);
    println!("  {} GJ = {:.0e} J", gigajoule.value, gigajoule.convert_to::<Joule>().value);
    println!("  {} TJ = {:.0e} J", terajoule.value, terajoule.convert_to::<Joule>().value);
    
    // ===============================
    // POWER UNITS WITH PREFIXES  
    // ===============================
    println!("\n🔌 Power Units with Prefixes:");
    
    let milliwatt = Power::<Prefixed<Milli, Watt>>::new(100.0);
    let watt = Power::<Watt>::new(60.0); // Light bulb
    let kilowatt = Power::<Prefixed<Kilo, Watt>>::new(2.0); // Household appliance
    let megawatt = Power::<Prefixed<Mega, Watt>>::new(1000.0); // Large power plant
    let gigawatt = Power::<Prefixed<Giga, Watt>>::new(1.21); // Back to the Future reference!
    let terawatt = Power::<Prefixed<Tera, Watt>>::new(15.0); // Global power consumption
    
    println!("  {} mW = {} W", milliwatt.value, milliwatt.convert_to::<Watt>().value);
    println!("  {} W = {} W", watt.value, watt.value);
    println!("  {} kW = {} W", kilowatt.value, kilowatt.convert_to::<Watt>().value);
    println!("  {} MW = {:.0e} W", megawatt.value, megawatt.convert_to::<Watt>().value);
    println!("  {} GW = {:.0e} W", gigawatt.value, gigawatt.convert_to::<Watt>().value);
    println!("  {} TW = {:.0e} W", terawatt.value, terawatt.convert_to::<Watt>().value);
    
    // ===============================
    // MIXING PREFIXED AND REGULAR UNITS
    // ===============================
    println!("\n🔄 Unit Conversions Between Prefixed and Regular Units:");
    
    // Convert astronomical distance to smaller units
    let au = Distance::<AstronomicalUnit>::new(1.0);
    let au_in_km = au.convert_to::<Prefixed<Kilo, Meter>>();
    let au_in_mm = au.convert_to::<Prefixed<Milli, Meter>>();
    
    println!("  1 AU = {:.0e} km", au_in_km.value);
    println!("  1 AU = {:.0e} mm", au_in_mm.value);
    
    // Convert solar mass to smaller units
    let solar_mass = Mass::<SolarMass>::new(1.0);
    let solar_mass_in_kg = solar_mass.convert_to::<Prefixed<Kilo, Gram>>();
    let solar_mass_in_mg = solar_mass.convert_to::<Prefixed<Mega, Gram>>();
    
    println!("  1 M☉ = {:.0e} kg", solar_mass_in_kg.value);
    println!("  1 M☉ = {:.0e} Mg", solar_mass_in_mg.value);
    
    // ===============================
    // EXTREME PREFIXES
    // ===============================
    println!("\n🚀 Extreme Prefix Examples:");
    
    let yoctometer = Distance::<Prefixed<Yocto, Meter>>::new(1.0); // Subatomic scale
    let yottameter = Distance::<Prefixed<Yotta, Meter>>::new(1.0); // Observable universe scale
    
    println!("  {} ym = {:.0e} m (subatomic scale)", yoctometer.value, yoctometer.convert_to::<Meter>().value);
    println!("  {} Ym = {:.0e} m (observable universe scale)", yottameter.value, yottameter.convert_to::<Meter>().value);
    
    // Compare to known scales
    let planck_length_approx = Distance::<Prefixed<Yocto, Meter>>::new(16.0); // ~1.6×10^-35 m
    let observable_universe = Distance::<Prefixed<Yotta, Meter>>::new(0.93); // ~9.3×10^25 m
    
    println!("  Planck length ≈ {} ym = {:.0e} m", planck_length_approx.value, planck_length_approx.convert_to::<Meter>().value);
    println!("  Observable universe ≈ {} Ym = {:.0e} m", observable_universe.value, observable_universe.convert_to::<Meter>().value);
    
    println!("\n✨ The prefix system works with ANY unit type automatically!");
    println!("   You can create combinations like:");
    println!("   - Temperature: Distance::<Prefixed<Milli, Kelvin>>::new(273.15)  // 273.15 mK");
    println!("   - Frequency: Distance::<Prefixed<Mega, Hertz>>::new(100.0)       // 100 MHz (when implemented)");
    println!("   - And any future unit types you add!");
}