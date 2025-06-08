pub type CosmicTime = f64;
pub type Metallicity = f64;

/// Vollständiges Sternsystem mit kosmischer Umgebung
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarSystem {
    pub name: &str,
    /// Eindeutige Seed für Reproduzierbarkeit
    pub seed: u64,
    /// Kosmische Parameter
    pub cosmic_epoch: CosmicEpoch,
    pub galactic_distance: Distance, // Wird jetzt aus GalacticRegion abgeleitet
    pub galactic_region: GalacticRegion,
    pub radiation_environment: CosmicRadiationEnvironment,
    /// Sternsystem Konfiguration
    pub system_type: SystemType,
    /// Elementhäufigkeiten
    pub elemental_abundance: ElementalAbundance,
    /// Gesamte Bewohnbarkeitsbewertung
    pub habitability_assessment: HabitabilityAssessment,
    /// Einheitensystem für Berechnungen
    pub unit_system: UnitSystem,
    pub galactic_dynamics: GalacticDynamics,
}

impl StarSystem {
    pub fn generate_from_seed(seed: u64) -> Self {
        Self::generate_from_seed_with_units(seed, UnitSystem::Astronomical)
    }

    pub fn generate_from_seed_with_units(seed: u64, unit_system: UnitSystem) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        let age_universe_gyr = rng.gen_range(3.0..13.8);
        let cosmic_epoch = CosmicEpoch::from_age(age_universe_gyr);

        // System Type wird vor galaktischer Umgebung generiert,
        // da manche Sterneigenschaften (z.B. Alter) von der kosmischen Epoche abhängen können.
        let system_type = Self::generate_system_type(&mut rng, &cosmic_epoch, unit_system);

        let galactic_region = GalacticRegion::generate_random(&mut rng, unit_system);
        let galactic_distance = galactic_region.distance_from_center().clone();

        let radiation_environment = CosmicRadiationEnvironment::from_region_and_epoch(
            &galactic_region,
            &cosmic_epoch,
            &mut rng,
        );

        let elemental_abundance = ElementalAbundance::from_metallicity_and_epoch(
            cosmic_epoch.epoch_metallicity,
            &cosmic_epoch,
        );

        // HIER WIRD GalacticDynamics BERECHNET UND GESPEICHERT
        let galactic_dynamics = GalacticDynamics::calculate_for_position(
            &galactic_region,
            cosmic_epoch.age_universe, // Alter des Universums in Gyr, wie von der Funktion erwartet
            &mut rng,
        );
        // Das Ergebnis von calculate_for_position muss dem Feld in StarSystem zugewiesen werden.

        let target_distances: Vec<Distance> = Vec::new();
        let habitability_assessment = HabitabilityAssessment::comprehensive_analysis(
            &system_type,
            &radiation_environment,
            &target_distances,
            // GalacticDynamics wird hier nicht direkt übergeben,
            // aber es ist Teil des StarSystem-Kontexts, der indirekt relevant sein könnte.
        );

        StarSystem {
            seed,
            cosmic_epoch,
            galactic_distance,
            galactic_region,
            radiation_environment,
            system_type,
            elemental_abundance,
            habitability_assessment,
            unit_system,
            galactic_dynamics, // <<-- HIER ZUWEISEN
        }
    }

    fn generate_system_type(
        rng: &mut ChaCha8Rng,
        cosmic_epoch: &CosmicEpoch,
        unit_system: UnitSystem,
    ) -> SystemType {
        let primary_mass_solar = Self::generate_stellar_mass(rng);
        let primary_mass = Mass::solar_masses(primary_mass_solar).to_system(unit_system);

        let multiplicity_probability = match primary_mass_solar {
            m if m > 15.0 => 0.8,
            m if m > 1.5 => 0.6,
            m if m > 0.5 => 0.4,
            _ => 0.25,
        };

        // Alter des Sterns sollte geringer oder gleich dem Alter des Universums sein
        // Und nicht zu jung, um Hauptreihensterne zu haben (oder Pre-MS, wenn gewünscht)
        // Wir nehmen hier einen Bruchteil des Universumsalters, oder die Logik in StellarProperties::new kümmert sich darum.
        // cosmic_epoch.age_universe ist in Gyr. StellarProperties erwartet Time.
        let star_age_gyr = rng.gen_range(0.1..cosmic_epoch.age_universe.min(10.0)); // Sterne können jünger sein als das Universum
        let age = Time::years(star_age_gyr * 1e9).to_system(unit_system);

        if rng.r#gen::<f64>() < multiplicity_probability {
            let secondary_mass_solar = Self::generate_secondary_mass(rng, primary_mass_solar);
            let secondary_mass = Mass::solar_masses(secondary_mass_solar).to_system(unit_system);

            let primary = StellarProperties::new(
                primary_mass.clone(),
                age.clone(),
                cosmic_epoch.epoch_metallicity,
            ); // unit_system hinzugefügt
            let secondary = StellarProperties::new(
                secondary_mass.clone(),
                age.clone(),
                cosmic_epoch.epoch_metallicity,
            ); // unit_system hinzugefügt

            let separation_au =
                Self::generate_binary_separation(rng, primary_mass_solar, secondary_mass_solar);
            let separation = Distance::au(separation_au).to_system(unit_system);
            let eccentricity = rng.gen_range(0.0..0.8);
            let inclination = rng.gen_range(0.0..180.0);
            let longitude_of_ascending_node = rng.gen_range(0.0..360.0);
            let argument_of_periapsis = rng.gen_range(0.0..360.0);

            let orbital_properties = BinaryOrbit::new(
                &primary,
                &secondary,
                separation,
                eccentricity,
                inclination,
                longitude_of_ascending_node,
                argument_of_periapsis,
            );

            if rng.r#gen::<f64>() < 0.1 && primary_mass_solar > 2.0 {
                let tertiary_mass_solar = Self::generate_secondary_mass(rng, secondary_mass_solar);
                let tertiary_mass = Mass::solar_masses(tertiary_mass_solar) // .clone() nicht nötig für f64
                    .to_system(unit_system);
                let tertiary = StellarProperties::new(
                    tertiary_mass,
                    age.clone(),
                    cosmic_epoch.epoch_metallicity,
                );

                let components = vec![primary, secondary, tertiary];
                let hierarchy = SystemHierarchy::new(&components);

                SystemType::Multiple {
                    components,
                    hierarchy,
                }
            } else {
                SystemType::Binary {
                    primary,
                    secondary,
                    orbital_properties,
                }
            }
        } else {
            let star =
                StellarProperties::new(primary_mass, age.clone(), cosmic_epoch.epoch_metallicity); // unit_system hinzugefügt
            SystemType::Single(star)
        }
    }

    fn generate_stellar_mass(rng: &mut ChaCha8Rng) -> f64 {
        let r: f64 = rng.r#gen();
        match r {
            x if x < 0.6 => rng.gen_range(0.1..0.5),
            x if x < 0.85 => rng.gen_range(0.5..1.0),
            x if x < 0.95 => rng.gen_range(1.0..2.0),
            x if x < 0.99 => rng.gen_range(2.0..10.0),
            _ => rng.gen_range(10.0..50.0),
        }
    }

    fn generate_secondary_mass(rng: &mut ChaCha8Rng, primary_mass: f64) -> f64 {
        let mass_ratio = rng.gen_range(0.1..1.0);
        (primary_mass * mass_ratio).max(0.08)
    }

    fn generate_binary_separation(
        rng: &mut ChaCha8Rng,
        primary_mass: f64,
        secondary_mass: f64,
    ) -> f64 {
        let log_separation = rng.gen_range(0.0..4.0);
        let separation_au = 10.0_f64.powf(log_separation);
        let mass_factor = (primary_mass + secondary_mass) / 2.0;
        separation_au * mass_factor.sqrt()
    }

    pub fn to_ron_string(&self) -> Result<String, ron::Error> {
        ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())
    }

    pub fn from_ron_string(s: &str) -> Result<Self, ron::error::SpannedError> {
        ron::from_str(s)
    }
    /// Fügt Stabilitätsanalyse zum Sternsystem hinzu
    pub fn calculate_system_stability(&self) -> SystemStability {
        SystemStability::analyze_system(&self.system_type)
    }

    /// Erweiterte System-Generierung mit Stabilitätsprüfung
    pub fn generate_stable_system(seed: u64, max_attempts: u32) -> Option<Self> {
        for _attempt in 0..max_attempts {
            let system = Self::generate_from_seed(seed + _attempt as u64);
            let stability = system.calculate_system_stability();

            if stability.is_million_year_stable() {
                return Some(system);
            }
        }
        None // Kein stabiles System nach max_attempts gefunden
    }
}
