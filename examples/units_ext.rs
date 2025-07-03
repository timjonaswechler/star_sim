//! Demonstration of the extended unit system with new physical quantities.

use star_sim::physics::units::*;

fn main() {
    println!("=== Extended Unit System Demo ===\n");

    // Gravitational Parameters
    println!("🪐 Gravitational Parameters:");
    let solar_gm = GravitationalParameter::<SolarGravitationalParameter>::new(1.0);
    let earth_gm = GravitationalParameter::<EarthGravitationalParameter>::new(1.0);

    println!("Solar GM: {} GM☉", solar_gm.value);
    println!("Earth GM: {} GM⊕", earth_gm.value);

    // Calculate orbital velocity around the Sun at 1 AU
    let au_distance = Distance::<AstronomicalUnit>::new(1.0);
    println!();

    // Spectral Units
    println!("🌈 Spectral Units:");
    let visible_light = Wavelength::<Prefixed<Nano, Meter>>::new(550.0);
    let infrared = Wavelength::<Prefixed<Micro, Meter>>::new(10.0);

    println!("Visible light: {} nm", visible_light.value);
    println!("Infrared: {} μm", infrared.value);

    let visible_freq = wavelength_to_frequency_si(visible_light.to_si());
    println!("Visible light frequency: {:.2e} Hz", visible_freq);
    println!();

    // Magnetic Fields
    println!("🧲 Magnetic Fields:");
    let earth_field = MagneticField::<Gauss>::new(0.5);
    let tesla_field = MagneticField::<Tesla>::new(1.0);

    println!("Earth's magnetic field: {} G", earth_field.value);
    println!("Laboratory field: {} T", tesla_field.value);
    println!("Earth's field in Tesla: {:.2e} T", earth_field.to_si());
    println!();

    // Specific Units
    println!("🌡️ Specific Units:");
    let specific_heat_water = SpecificHeatCapacity::<JoulePerKilogramKelvin>::new(4184.0);
    let binding_energy = SpecificEnergy::<JoulePerKilogram>::new(6.3e14);

    println!(
        "Water specific heat: {} J/(kg⋅K)",
        specific_heat_water.value
    );
    println!("Fusion binding energy: {:.1e} J/kg", binding_energy.value);
    println!();

    // Stellar Physics
    println!("⭐ Stellar Physics Examples:");
    let sun_radius = Distance::<SunRadius>::new(1.0);
    let sun_mass = Mass::<SolarMass>::new(1.0);
    let sun_temp = Temperature::<Kelvin>::new(5778.0);

    println!("Sun radius: {} R☉", sun_radius.value);
    println!("Sun mass: {} M☉", sun_mass.value);
    println!("Sun temperature: {} K", sun_temp.value);

    let sun_luminosity = stellar_luminosity_si(sun_radius.to_si(), sun_temp.to_si());
    println!("Calculated luminosity: {:.2e} W", sun_luminosity);
    println!();

    // Mathematical Operations
    println!("🧮 Physics Calculations:");
    let peak_wavelength = wien_peak_wavelength_si(5778.0);
    println!("Wien peak wavelength: {:.0} nm", peak_wavelength * 1e9);

    let thermal_energy = thermal_energy_per_particle_si(15_000_000.0);
    println!("Stellar core thermal energy: {:.2e} J", thermal_energy);

    println!();

    println!("🎯 Extended unit system successfully integrated!");
}
