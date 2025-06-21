use star_sim::physics::units::core::*;
use star_sim::physics::units::*;

#[test]
fn test_basic_unit_creation() {
    let distance = Distance::<Meter>::new(100.0);
    assert_eq!(distance.value(), 100.0);

    let mass = Mass::<Kilogram>::new(50.0);
    assert_eq!(mass.value(), 50.0);
}

#[test]
fn test_unit_conversions() {
    // Test Distance conversions
    let one_au = Distance::<AstronomicalUnit>::new(1.0);
    let in_meters = one_au.convert_to::<Meter>();
    assert!((in_meters.value() - 149_597_870_700.0).abs() < 1.0);

    let earth_radius = Distance::<EarthRadius>::new(1.0);
    let in_km = earth_radius.convert_to::<Kilometer>();
    assert!((in_km.value() - 6371.0).abs() < 1.0);

    // Test Mass conversions
    let earth_mass = Mass::<EarthMass>::new(1.0);
    let in_kg = earth_mass.convert_to::<Kilogram>();
    assert!((in_kg.value() - 5.972e24).abs() < 1e20);

    let solar_mass = Mass::<SolarMass>::new(1.0);
    let in_earth_masses = solar_mass.convert_to::<EarthMass>();
    assert!((in_earth_masses.value() - 333000.0).abs() < 1000.0);
}

#[test]
fn test_arithmetic_operations() {
    let d1 = Distance::<Meter>::new(100.0);
    let d2 = Distance::<Meter>::new(50.0);

    // Addition
    let sum = d1 + d2;
    assert_eq!(sum.value(), 150.0);

    // Subtraction
    let diff = d1 - d2;
    assert_eq!(diff.value(), 50.0);

    // Scalar multiplication
    let scaled = d1 * 2.0;
    assert_eq!(scaled.value(), 200.0);

    // Scalar division
    let divided = d1 / 2.0;
    assert_eq!(divided.value(), 50.0);

    // Negation
    let neg = -d1;
    assert_eq!(neg.value(), -100.0);
}

#[test]
fn test_dimensional_analysis() {
    let distance = Distance::<Meter>::new(100.0);
    let time = Time::<Second>::new(10.0);

    // Distance / Time = Velocity (returns SI value)
    let velocity_si = calculate_velocity(distance, time);
    assert_eq!(velocity_si, 10.0);

    let mass = Mass::<Kilogram>::new(5.0);
    let acceleration = Distance::<Meter>::new(2.0); // Simplified for now

    // Mass * Distance = simplified calculation (returns SI value)
    let result_si = multiply_quantities(mass, acceleration);
    assert_eq!(result_si, 10.0);
}

#[test]
fn test_astronomical_units() {
    // Test stellar properties
    let star_mass = Mass::<SolarMass>::new(0.7);
    let star_radius = Distance::<SunRadius>::new(0.66);
    let system_age = Time::<Gigayear>::new(6.0);

    // Convert to SI units
    let mass_kg = star_mass.convert_to::<Kilogram>();
    let radius_m = star_radius.convert_to::<Meter>();
    let age_s = system_age.convert_to::<Second>();

    assert!(mass_kg.value() > 1e30);
    assert!(radius_m.value() > 1e8);
    assert!(age_s.value() > 1e17);
}

#[test]
fn test_serialization() {
    let distance = Distance::<AstronomicalUnit>::new(1.5);
    let mass = Mass::<EarthMass>::new(0.8);

    // Test RON serialization
    let ron_distance = ron::to_string(&distance).unwrap();
    let ron_mass = ron::to_string(&mass).unwrap();

    // Test deserialization
    let deserialized_distance: Distance<AstronomicalUnit> = ron::from_str(&ron_distance).unwrap();
    let deserialized_mass: Mass<EarthMass> = ron::from_str(&ron_mass).unwrap();

    assert!((distance.value() - deserialized_distance.value()).abs() < f64::EPSILON);
    assert!((mass.value() - deserialized_mass.value()).abs() < f64::EPSILON);
}

#[test]
fn test_display_formatting() {
    let distance = Distance::<AstronomicalUnit>::new(1.5);
    let mass = Mass::<EarthMass>::new(0.8);
    let power = Power::<SolarLuminosity>::new(0.15);

    let distance_str = format!("{}", distance);
    let mass_str = format!("{}", mass);
    let power_str = format!("{}", power);

    assert!(distance_str.contains("AU"));
    assert!(distance_str.contains("1.5"));
    assert!(mass_str.contains("M⊕"));
    assert!(power_str.contains("L☉"));
}
