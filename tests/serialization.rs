use ron;
use star_sim::physics::units_v2::*;

macro_rules! unit_serialization_test {
    ($name:ident, $typ:ty, $value:expr) => {
        #[test]
        fn $name() {
            let original: $typ = <$typ>::new($value);
            let ron_string = ron::to_string(&original).unwrap();
            let deserialized: $typ = ron::from_str(&ron_string).unwrap();
            assert!((original.value() - deserialized.value()).abs() < f64::EPSILON);
        }
    };
}

unit_serialization_test!(distance_meter, Distance<Meter>, 1.0);
unit_serialization_test!(mass_gram, Mass<Gram>, 1.0);
unit_serialization_test!(time_second, Time<Second>, 1.0);
unit_serialization_test!(temperature_kelvin, Temperature<Kelvin>, 300.0);
unit_serialization_test!(velocity_mps, Velocity<MeterPerSecond>, 42.0);
unit_serialization_test!(acceleration_mps2, Acceleration<MeterPerSecondSquared>, 9.8);
unit_serialization_test!(angle_radian, Angle<Radian>, 1.57);
unit_serialization_test!(area_sqm, Area<SquareMeter>, 5.0);
unit_serialization_test!(volume_cbm, Volume<CubicMeter>, 3.0);
unit_serialization_test!(pressure_pascal, Pressure<Pascal>, 101325.0);
unit_serialization_test!(energy_joule, Energy<Joule>, 500.0);
unit_serialization_test!(power_watt, Power<Watt>, 1200.0);
unit_serialization_test!(force_newton, Force<Newton>, 10.0);
