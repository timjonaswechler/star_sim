use star_sim::physics::units::*;

fn main() {
    // Erstelle eine Distance in AU
    let distance_au = Distance::<AstronomicalUnit>::new(1.5);
    println!("Original distance: {}", distance_au);
    
    // Konvertierung mit convert_to()
    let distance_meters = distance_au.convert_to::<Meter>();
    println!("Converted with convert_to(): {}", distance_meters);
    
    // Konvertierung mit AutoConvert trait
    let distance_meters_auto: Distance<Meter> = distance_au.convert();
    println!("Converted with convert(): {}", distance_meters_auto);
    
    // Test mit anderen Units
    let earth_radius = Distance::<EarthRadius>::new(1.0);
    let earth_radius_meters: Distance<Meter> = earth_radius.convert();
    println!("Earth radius in meters: {}", earth_radius_meters);
    
    // Test mit Mass units
    let solar_mass = Mass::<SolarMass>::new(0.8);
    let solar_mass_kg: Mass<Kilogram> = solar_mass.convert();
    println!("0.8 solar masses in kg: {}", solar_mass_kg);
}