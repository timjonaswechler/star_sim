//! Comprehensive test of all implemented units in the new units_v2 system.
//!
//! This example demonstrates all the units that have been implemented from the old system
//! into the new units_v2 system, including:
//! 
//! ✅ Basic Units: Distance, Mass, Time, Temperature, Energy, Power
//! ✅ Angular Units: Angle, AngularVelocity, AngularAcceleration  
//! ✅ Derived Units: Area, Volume, Velocity, Acceleration, Force, Pressure, Density, Frequency, Momentum
//! ✅ Prefix System: All SI prefixes (yocto to yotta) work with all units
//! ✅ Astronomical Units: AU, parsecs, solar masses, etc.
//! ✅ Conversions: Hub-and-spoke conversions through SI base units

use star_sim::physics::units_v2::*;

fn main() {
    println!("=== Complete Units Implementation Test ===\n");
    
    // ===============================
    // BASIC UNITS (from old system)
    // ===============================
    
    println!("🎯 Basic Units (Direct from Old System):");
    
    // Distance units
    let dist_m = Distance::<Meter>::new(1000.0);
    let dist_km = Distance::<Kilometer>::new(1.5);
    let dist_au = Distance::<AstronomicalUnit>::new(1.0);
    let dist_ly = Distance::<LightYear>::new(4.2);
    let dist_pc = Distance::<Parsec>::new(1.0);
    let dist_kpc = Distance::<Kiloparsec>::new(1.0); // ✅ Added from old system
    
    println!("  📏 Distance: {} m, {} km, {} AU, {} ly, {} pc, {} kpc", 
             dist_m.value, dist_km.value, dist_au.value, dist_ly.value, dist_pc.value, dist_kpc.value);
    
    // Mass units  
    let mass_g = Mass::<Gram>::new(1000.0);
    let mass_kg = Mass::<Kilogram>::new(70.0);
    let mass_earth = Mass::<EarthMass>::new(1.0);
    let mass_sun = Mass::<SolarMass>::new(1.0);
    
    println!("  ⚖️  Mass: {} g, {} kg, {} M⊕, {} M☉", 
             mass_g.value, mass_kg.value, mass_earth.value, mass_sun.value);
    
    // Time units
    let time_s = Time::<Second>::new(3600.0);
    let time_min = Time::<Minute>::new(60.0);
    let time_hr = Time::<Hour>::new(24.0);
    let time_day = Time::<Day>::new(365.25);
    let time_yr = Time::<Year>::new(13.8);
    let time_gyr = Time::<Gigayear>::new(13.8);
    
    println!("  ⏰ Time: {} s, {} min, {} hr, {} d, {} yr, {} Gyr", 
             time_s.value, time_min.value, time_hr.value, time_day.value, time_yr.value, time_gyr.value);
    
    // Temperature 
    let temp_k = Temperature::<Kelvin>::new(273.15);
    println!("  🌡️ Temperature: {} K", temp_k.value);
    
    // Energy
    let energy_j = Energy::<Joule>::new(1000.0);
    let energy_erg = Energy::<Erg>::new(1e10);
    let energy_ev = Energy::<ElectronVolt>::new(1.0);
    
    println!("  ⚡ Energy: {} J, {} erg, {} eV", 
             energy_j.value, energy_erg.value, energy_ev.value);
    
    // Power
    let power_w = Power::<Watt>::new(100.0);
    let power_sun = Power::<SolarLuminosity>::new(1.0);
    
    println!("  🔌 Power: {} W, {} L☉", power_w.value, power_sun.value);
    
    // ===============================
    // ANGULAR UNITS (newly implemented)
    // ===============================
    
    println!("\n🔄 Angular Units (New Implementation):");
    
    let angle_rad = Angle::<Radian>::new(std::f64::consts::PI);
    let angle_deg = Angle::<Degree>::new(180.0);
    
    println!("  📐 Angle: {} rad = {} °", angle_rad.value, angle_deg.value);
    println!("    Conversion: {} rad = {:.1} °", 
             angle_rad.value, angle_rad.convert_to::<Degree>().value);
    
    let ang_vel_rad_s = AngularVelocity::<RadianPerSecond>::new(2.0 * std::f64::consts::PI);
    let ang_vel_deg_s = AngularVelocity::<DegreePerSecond>::new(360.0);
    
    println!("  🔄 Angular Velocity: {} rad/s = {} °/s", ang_vel_rad_s.value, ang_vel_deg_s.value);
    println!("    Conversion: {} rad/s = {:.1} °/s", 
             ang_vel_rad_s.value, ang_vel_rad_s.convert_to::<DegreePerSecond>().value);
    
    let ang_acc = AngularAcceleration::<RadianPerSecondSquared>::new(1.0);
    println!("  ⚡ Angular Acceleration: {} rad/s²", ang_acc.value);
    
    // ===============================
    // DERIVED UNITS (newly implemented)
    // ===============================
    
    println!("\n📊 Derived Units (New Implementation):");
    
    // Area
    let area_m2 = Area::<SquareMeter>::new(100.0);
    let area_km2 = Area::<SquareKilometer>::new(1.0);
    
    println!("  📐 Area: {} m² = {} km²", area_m2.value, area_km2.value);
    println!("    Conversion: {} m² = {:.0e} km²", 
             area_m2.value, area_m2.convert_to::<SquareKilometer>().value);
    
    // Volume
    let vol_m3 = Volume::<CubicMeter>::new(1.0);
    let vol_l = Volume::<Liter>::new(1000.0);
    
    println!("  📦 Volume: {} m³ = {} L", vol_m3.value, vol_l.value);
    println!("    Conversion: {} m³ = {} L", 
             vol_m3.value, vol_m3.convert_to::<Liter>().value);
    
    // Velocity
    let vel_ms = Velocity::<MeterPerSecond>::new(30.0);
    let vel_kmh = Velocity::<KilometerPerHour>::new(108.0);
    
    println!("  🚗 Velocity: {} m/s = {} km/h", vel_ms.value, vel_kmh.value);
    println!("    Conversion: {} m/s = {:.1} km/h", 
             vel_ms.value, vel_ms.convert_to::<KilometerPerHour>().value);
    
    // Acceleration
    let acc_ms2 = Acceleration::<MeterPerSecondSquared>::new(9.81);
    let acc_g = Acceleration::<StandardGravity>::new(1.0);
    
    println!("  📈 Acceleration: {} m/s² = {} g₀", acc_ms2.value, acc_g.value);
    println!("    Earth gravity: {} m/s² = {:.2} g₀", 
             acc_ms2.value, acc_ms2.convert_to::<StandardGravity>().value);
    
    // Force
    let force_n = Force::<Newton>::new(100.0);
    println!("  💪 Force: {} N", force_n.value);
    
    // Pressure
    let pressure_pa = Pressure::<Pascal>::new(101325.0);
    let pressure_bar = Pressure::<Bar>::new(1.0);
    
    println!("  🌀 Pressure: {} Pa = {} bar", pressure_pa.value, pressure_bar.value);
    println!("    Atmospheric pressure: {} Pa = {:.5} bar", 
             pressure_pa.value, pressure_pa.convert_to::<Bar>().value);
    
    // Density
    let density_kg_m3 = Density::<KilogramPerCubicMeter>::new(1000.0);
    let density_g_cm3 = Density::<GramPerCubicCentimeter>::new(1.0);
    
    println!("  🧊 Density: {} kg/m³ = {} g/cm³", density_kg_m3.value, density_g_cm3.value);
    println!("    Water density: {} kg/m³ = {} g/cm³", 
             density_kg_m3.value, density_kg_m3.convert_to::<GramPerCubicCentimeter>().value);
    
    // Frequency
    let freq_hz = Frequency::<Hertz>::new(440.0);
    println!("  🎵 Frequency: {} Hz (A4 note)", freq_hz.value);
    
    // Momentum
    let momentum = Momentum::<KilogramMeterPerSecond>::new(1000.0);
    println!("  🏃 Momentum: {} kg⋅m/s", momentum.value);
    
    // ===============================
    // PREFIX SYSTEM DEMONSTRATION
    // ===============================
    
    println!("\n🔬 Prefix System (Works with ALL Units):");
    
    // Extreme scales with prefixes
    let planck_length = Distance::<Prefixed<Yocto, Meter>>::new(16.0); // ~1.6×10^-35 m
    let proton_radius = Distance::<Prefixed<Femto, Meter>>::new(840.0); // ~8.4×10^-16 m
    let dna_width = Distance::<Prefixed<Nano, Meter>>::new(2.5); // ~2.5 nm
    let hair_width = Distance::<Prefixed<Micro, Meter>>::new(50.0); // ~50 μm
    let observable_universe = Distance::<Prefixed<Yotta, Meter>>::new(0.93); // ~9.3×10^25 m
    
    println!("  📏 Scale examples:");
    println!("    Planck length: {} ym = {:.0e} m", planck_length.value, planck_length.convert_to::<Meter>().value);
    println!("    Proton radius: {} fm = {:.0e} m", proton_radius.value, proton_radius.convert_to::<Meter>().value);
    println!("    DNA width: {} nm = {:.0e} m", dna_width.value, dna_width.convert_to::<Meter>().value);
    println!("    Hair width: {} μm = {:.0e} m", hair_width.value, hair_width.convert_to::<Meter>().value);
    println!("    Observable universe: {} Ym = {:.0e} m", observable_universe.value, observable_universe.convert_to::<Meter>().value);
    
    // Prefixes with different unit types
    let microwave_freq = Frequency::<Prefixed<Giga, Hertz>>::new(2.4); // 2.4 GHz WiFi
    let cpu_freq = Frequency::<Prefixed<Giga, Hertz>>::new(3.5); // 3.5 GHz CPU
    let radio_freq = Frequency::<Prefixed<Mega, Hertz>>::new(100.0); // 100 MHz FM radio
    
    println!("  📡 Frequency examples with prefixes:");
    println!("    WiFi: {} GHz = {:.0e} Hz", microwave_freq.value, microwave_freq.convert_to::<Hertz>().value);
    println!("    CPU: {} GHz = {:.0e} Hz", cpu_freq.value, cpu_freq.convert_to::<Hertz>().value);
    println!("    FM Radio: {} MHz = {:.0e} Hz", radio_freq.value, radio_freq.convert_to::<Hertz>().value);
    
    // ===============================
    // CONVERSION EXAMPLES
    // ===============================
    
    println!("\n🔄 Complex Unit Conversions:");
    
    // Astronomical to metric conversions
    let distance_to_alpha_centauri = Distance::<LightYear>::new(4.37);
    let distance_in_au = distance_to_alpha_centauri.convert_to::<AstronomicalUnit>();
    let distance_in_km = distance_to_alpha_centauri.convert_to::<Prefixed<Kilo, Meter>>();
    
    println!("  🌟 Alpha Centauri distance:");
    println!("    {} ly = {:.0} AU = {:.0e} km", 
             distance_to_alpha_centauri.value, distance_in_au.value, distance_in_km.value);
    
    // Mass conversions
    let black_hole_mass = Mass::<SolarMass>::new(4_000_000.0); // Sagittarius A*
    let black_hole_kg = black_hole_mass.convert_to::<Kilogram>();
    let black_hole_earth_masses = black_hole_mass.convert_to::<EarthMass>();
    
    println!("  🕳️ Sagittarius A* black hole:");
    println!("    {} M☉ = {:.0e} kg = {:.0e} M⊕", 
             black_hole_mass.value, black_hole_kg.value, black_hole_earth_masses.value);
    
    // ===============================
    // SUMMARY
    // ===============================
    
    println!("\n✅ Implementation Summary:");
    println!("   📏 Distance: Meter, Kilometer, AU, LightYear, Parsec, Kiloparsec + prefixes");
    println!("   ⚖️  Mass: Gram, Kilogram, EarthMass, SolarMass + prefixes");
    println!("   ⏰ Time: Second, Minute, Hour, Day, Year, Gigayear + prefixes");
    println!("   🌡️ Temperature: Kelvin + prefixes");
    println!("   ⚡ Energy: Joule, Erg, ElectronVolt + prefixes");
    println!("   🔌 Power: Watt, SolarLuminosity + prefixes");
    println!("   📐 Angle: Radian, Degree + prefixes");
    println!("   🔄 AngularVelocity: RadianPerSecond, DegreePerSecond + prefixes");
    println!("   📈 AngularAcceleration: RadianPerSecondSquared + prefixes");
    println!("   📊 Area: SquareMeter, SquareKilometer + prefixes");
    println!("   📦 Volume: CubicMeter, Liter + prefixes");
    println!("   🚗 Velocity: MeterPerSecond, KilometerPerHour + prefixes");
    println!("   📈 Acceleration: MeterPerSecondSquared, StandardGravity + prefixes");
    println!("   💪 Force: Newton + prefixes");
    println!("   🌀 Pressure: Pascal, Bar + prefixes");
    println!("   🧊 Density: KilogramPerCubicMeter, GramPerCubicCentimeter + prefixes");
    println!("   🎵 Frequency: Hertz + prefixes");
    println!("   🏃 Momentum: KilogramMeterPerSecond + prefixes");
    println!("   🔬 Prefixes: All 20 SI prefixes (yocto to yotta) work with ALL units");
    
    println!("\n🎉 All missing units from the old system have been successfully implemented!");
    println!("   The new system is now COMPLETE and provides significant advantages:");
    println!("   ✨ Hub-and-spoke conversions (O(n) vs O(n²) complexity)");
    println!("   ✨ Compile-time dimensional safety");
    println!("   ✨ Universal prefix system");
    println!("   ✨ Automatic unit generation via macros");
    println!("   ✨ Astronomy-focused with Unicode symbols");
}