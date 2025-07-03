use crate::physics::units::{
    self, Angle, Degree, Fraction, Kelvin, Kilo, Kilogram, Mega, Meter, Prefixed, Ratio, Second,
    Year,
};
use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct Root;

#[derive(Component, Debug, Clone, Copy)]
pub struct Star;

#[derive(Component, Debug, Clone, Copy)]
pub struct RockyBody;

#[derive(Component, Debug, Clone, Copy)]
pub struct GaseousBody;

#[derive(Component, Debug, Clone, Copy)]
pub struct IcyBody;

#[derive(Component, Debug, Clone, Copy)]
pub struct Habitable;

#[derive(Component, Debug, Clone, Copy)]
pub struct Mass {
    pub value: units::Mass<Kilogram>,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Age {
    pub value: units::Time<Prefixed<Mega, Year>>,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct MeanRadius {
    pub value: units::Distance<Prefixed<Kilo, Meter>>,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct SurfaceGravity {
    pub value: units::Acceleration<Prefixed<Kilo, Meter>>,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct RotationPeriod {
    pub value: units::Time<Prefixed<Kilo, Second>>,
}
#[derive(Component, Debug, Clone, Copy)]
pub struct AxialTilt {
    pub value: Angle<Degree>,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct BlackBodyTemperature {
    pub value: units::Temperature<Kelvin>,
}

pub struct CelestialBody {
    pub mass: Mass,
    pub mean_radius: MeanRadius,
}

// Define the orbital parameters for celestial bodies
#[derive(Component, Debug, Clone, Copy)]
pub struct SemiMajorAxis {
    pub value: units::Distance<Prefixed<Kilo, Meter>>,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Eccentricity {
    pub value: Ratio<Fraction>, // 0.0 to 1.0
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Inclination {
    pub value: Angle<Degree>,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct ArgumentOfPeriapsis {
    pub value: Angle<Degree>,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct TimeOfPeriapsis {
    pub value: units::Time<Year>,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct LongitudeOfAscendingNode {
    pub value: Angle<Degree>,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct AscendingNode {
    pub value: units::Distance<Prefixed<Kilo, Meter>>,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct DescendingNode {
    pub value: units::Distance<Prefixed<Kilo, Meter>>,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Orbit {
    pub semi_major_axis: SemiMajorAxis,
    pub eccentricity: Eccentricity,
    pub inclination: Inclination,
    pub argument_of_periapsis: ArgumentOfPeriapsis,
    pub time_of_periapsis: TimeOfPeriapsis,
    pub longitude_of_ascending_node: LongitudeOfAscendingNode,
}

impl Default for Orbit {
    fn default() -> Self {
        Self {
            semi_major_axis: SemiMajorAxis {
                value: units::Distance::<Prefixed<Kilo, Meter>>::new(1.0),
            },
            eccentricity: Eccentricity {
                value: Ratio::new(0.0),
            },
            inclination: Inclination {
                value: Angle::<Degree>::new(0.0),
            },
            argument_of_periapsis: ArgumentOfPeriapsis {
                value: Angle::<Degree>::new(0.0),
            },
            time_of_periapsis: TimeOfPeriapsis {
                value: units::Time::<Year>::new(1.0),
            },
            longitude_of_ascending_node: LongitudeOfAscendingNode {
                value: Angle::<Degree>::new(0.0),
            },
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Barycenter {
    pub mass: units::Mass<Kilogram>,
}

impl Default for Barycenter {
    fn default() -> Self {
        Self {
            mass: units::Mass::<Kilogram>::new(0.0),
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct SatelliteOf(pub Entity);

pub trait SatelliteCommands {
    fn with_satellites<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut SatelliteBuilder);
}

pub struct SatelliteBuilder<'w, 's> {
    parent: Entity,
    commands: &'s mut Commands<'w, 's>,
}

impl<'w, 's> SatelliteBuilder<'w, 's> {
    pub fn spawn_satellite(&mut self, bundle: impl Bundle) -> Entity {
        let child_entity = self.commands.spawn(bundle).id();
        self.commands
            .entity(child_entity)
            .insert(SatelliteOf(self.parent));
        self.commands.entity(self.parent).add_child(child_entity);
        child_entity
    }
}

impl SatelliteCommands for EntityCommands<'_> {
    fn with_satellites<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut SatelliteBuilder),
    {
        let parent = self.id();
        let commands_ptr = &mut self.commands() as *mut Commands;
        unsafe {
            let mut builder = SatelliteBuilder {
                parent,
                commands: &mut *commands_ptr,
            };
            f(&mut builder);
        }
        self
    }
}

// Beispiel Query-Systeme für SatelliteOf
pub fn query_all_satellites(query: Query<(&Name, &SatelliteOf)>) {
    for (name, satellite_of) in &query {
        println!("Satellit '{}' umkreist Entity {:?}", name, satellite_of.0);
    }
}

pub fn query_satellites_of_specific_parent(
    query: Query<(&Name, &SatelliteOf)>,
    parent_query: Query<&Name, With<Root>>,
) {
    if let Ok(parent_name) = parent_query.get_single() {
        for (sat_name, satellite_of) in &query {
            if let Ok(parent_name) = parent_query.get(satellite_of.0) {
                println!("Satellit '{}' umkreist '{}'", sat_name, parent_name);
            }
        }
    }
}

pub fn query_rocky_satellites(query: Query<(&Name, &SatelliteOf), With<RockyBody>>) {
    for (name, satellite_of) in &query {
        println!(
            "Gesteinsplanet '{}' ist Satellit von {:?}",
            name, satellite_of.0
        );
    }
}

pub fn query_satellites_with_orbit(query: Query<(&Name, &SatelliteOf, &Orbit)>) {
    for (name, satellite_of, orbit) in &query {
        println!(
            "Satellit '{}' umkreist {:?} mit Halbachse: {:?}",
            name, satellite_of.0, orbit.semi_major_axis
        );
    }
}

pub fn query_nested_hierarchy(query: Query<(&Name, &SatelliteOf)>, name_query: Query<&Name>) {
    println!("=== Hierarchie der Satelliten ===");
    for (sat_name, satellite_of) in &query {
        if let Ok(parent_name) = name_query.get(satellite_of.0) {
            println!("  {} → {}", parent_name, sat_name);
        }
    }
}

pub fn find_satellites_by_parent_name(
    satellite_query: Query<(&Name, &SatelliteOf)>,
    name_query: Query<&Name>,
    parent_name: &str,
) -> Vec<String> {
    let mut satellites = Vec::new();

    for (sat_name, satellite_of) in &satellite_query {
        if let Ok(parent_name_comp) = name_query.get(satellite_of.0) {
            if parent_name_comp.as_str() == parent_name {
                satellites.push(sat_name.to_string());
            }
        }
    }

    satellites
}

pub fn setup_system(mut commands: Commands) {
    // Haupt-Baryzentrum (System-Ebene)
    let main_barycenter = commands
        .spawn((
            Name::new("System Barycenter"),
            Barycenter::default(),
            Orbit::default(),
            Root,
        ))
        .id();

    // Himmelskörper als Satelliten des Haupt-Baryzentrums
    commands
        .entity(main_barycenter)
        .with_children(|system_bary| {
            system_bary.spawn((
                Name::new("Star A"),
                Star,
                Orbit::default(),
                SatelliteOf(main_barycenter),
            ));
            system_bary.spawn((
                Name::new("Star B"),
                Star,
                Orbit::default(),
                SatelliteOf(main_barycenter),
            ));
        });

    // Planet B Baryzentrum
    let planet_b_barycenter = commands
        .spawn((
            Name::new("Planet B Barycenter"),
            Barycenter::default(),
            Orbit::default(),
            SatelliteOf(main_barycenter),
        ))
        .id();
    commands
        .entity(main_barycenter)
        .add_child(planet_b_barycenter);

    // Moon B1 Baryzentrum (hat eigene Satelliten)
    let moon_b1_barycenter = commands
        .spawn((
            Name::new("Moon B1 Barycenter"),
            Barycenter::default(),
            Orbit::default(),
            SatelliteOf(planet_b_barycenter),
        ))
        .id();
    commands
        .entity(planet_b_barycenter)
        .add_child(moon_b1_barycenter);

    // Planet B Himmelskörper
    commands
        .entity(planet_b_barycenter)
        .with_children(|planet_bary| {
            planet_bary.spawn((
                Name::new("Planet B"),
                RockyBody,
                Orbit::default(),
                SatelliteOf(planet_b_barycenter),
            ));

            planet_bary.spawn((
                Name::new("Moon B2"),
                RockyBody,
                Orbit::default(),
                SatelliteOf(planet_b_barycenter),
            ));
            planet_bary.spawn((
                Name::new("Moon B1"),
                IcyBody,
                Orbit::default(),
                SatelliteOf(planet_b_barycenter),
            ));
        });

    // Planet C Baryzentrum
    let planet_c_barycenter = commands
        .spawn((
            Name::new("Planet C Barycenter"),
            Barycenter::default(),
            Orbit::default(),
            SatelliteOf(main_barycenter),
        ))
        .id();
    commands
        .entity(main_barycenter)
        .add_child(planet_c_barycenter);

    // Planet C Himmelskörper
    commands
        .entity(planet_c_barycenter)
        .with_children(|planet_bary| {
            planet_bary.spawn((
                Name::new("Planet C"),
                GaseousBody,
                Orbit::default(),
                SatelliteOf(planet_c_barycenter),
            ));
            planet_bary.spawn((
                Name::new("Moon C1"),
                IcyBody,
                Orbit::default(),
                SatelliteOf(planet_c_barycenter),
            ));
            planet_bary.spawn((
                Name::new("Moon C2"),
                IcyBody,
                Orbit::default(),
                SatelliteOf(planet_c_barycenter),
            ));
            planet_bary.spawn((
                Name::new("Moon C3"),
                RockyBody,
                Orbit::default(),
                SatelliteOf(planet_c_barycenter),
            ));
        });
}
