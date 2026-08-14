//! Claim-level provenance adapter for generated stellar catalogs.

use super::super::*;
use super::{GeneratedStellarCatalog, StellarClaimValue};

pub(super) fn generate(
    seed: u64,
    catalog: &GeneratedStellarCatalog,
    birth_mass_model: &StellarBirthMassModel,
    population_history_model: &PopulationHistoryModel,
    evolution_model_fingerprint: &str,
) -> Result<ProvenanceDocument<StellarClaimValue>, ProvenanceError> {
    const SOURCE_ID: &str = "source.kroupa-2001-canonical-imf";
    const COMPANION_SOURCE_ID: &str = "source.reggiani-meyer-2013-mass-ratios";
    const EVOLUTION_SOURCE_ID: &str = "source.choi-2016-mist-v1-2";
    const COOLING_SOURCE_ID: &str = "source.bedard-2020-montreal-white-dwarf-cooling";
    const PRESCRIPTION_ID: &str = "prescription.stellar-birth-primary-mass-proxy-v1";
    const COMPANION_RATIO_PRESCRIPTION_ID: &str =
        "prescription.stellar-birth-companion-mass-ratio-proxy-v1";
    const COMPANION_MASS_PRESCRIPTION_ID: &str =
        "prescription.stellar-birth-companion-mass-derivation-v1";
    const MEMBER_COUNT_PRESCRIPTION_ID: &str = "prescription.stellar-birth-member-count-proxy-v1";
    const MEMBER_ROLE_PRESCRIPTION_ID: &str =
        "prescription.stellar-birth-member-role-derivation-v1";
    const POPULATION_PRESCRIPTION_ID: &str = "prescription.stellar-population-assignment-v1";
    const AGE_PRESCRIPTION_ID: &str = "prescription.stellar-population-age-proxy-v1";
    const IRON_PRESCRIPTION_ID: &str = "prescription.stellar-population-iron-proxy-v1";
    const ALPHA_PRESCRIPTION_ID: &str = "prescription.stellar-chemistry-alpha-proxy-v1";
    const CHEMISTRY_PRESCRIPTION_ID: &str = "prescription.stellar-chemistry-derivation-v1";
    const EVOLUTION_PRESCRIPTION_ID: &str = "prescription.stellar-evolution-mist-v1-2-v1";
    const COOLING_PRESCRIPTION_ID: &str =
        "prescription.white-dwarf-cooling-montreal-bedard-2020-thick-h-v1";

    let source_reference = ScientificSourceReference {
        source_id: SourceId::from(SOURCE_ID),
        locator: Some("canonical stellar IMF, equations 1-2".into()),
    };
    let mut source =
        ScientificSource::new(SOURCE_ID, "On the variation of the initial mass function")?;
    source.authors = vec!["Pavel Kroupa".into()];
    source.publication = Some("Monthly Notices of the Royal Astronomical Society".into());
    source.publication_year = Some(2001);
    source.doi = Some("10.1046/j.1365-8711.2001.04022.x".into());
    source.url = Some("https://arxiv.org/abs/astro-ph/0009005".into());
    source.validate()?;

    let companion_source_reference = ScientificSourceReference {
        source_id: SourceId::from(COMPANION_SOURCE_ID),
        locator: Some("period-marginal mass-ratio exponent".into()),
    };
    let mut companion_source = ScientificSource::new(
        COMPANION_SOURCE_ID,
        "The universality of the companion mass-ratio distribution",
    )?;
    companion_source.authors = vec!["Maddalena M. Reggiani".into(), "Michael R. Meyer".into()];
    companion_source.publication = Some("Astronomy & Astrophysics".into());
    companion_source.publication_year = Some(2013);
    companion_source.doi = Some("10.1051/0004-6361/201321631".into());
    companion_source.url = Some("https://arxiv.org/abs/1304.3459".into());
    companion_source.validate()?;

    let evolution_source_reference = ScientificSourceReference {
        source_id: SourceId::from(EVOLUTION_SOURCE_ID),
        locator: Some("MIST v1.2 non-rotating solar-scaled stellar tracks".into()),
    };
    let mut evolution_source = ScientificSource::new(
        EVOLUTION_SOURCE_ID,
        "MESA Isochrones and Stellar Tracks (MIST). I: Solar-Scaled Models",
    )?;
    evolution_source.authors = vec!["Jieun Choi et al.".into()];
    evolution_source.publication = Some("The Astrophysical Journal".into());
    evolution_source.publication_year = Some(2016);
    evolution_source.doi = Some("10.3847/0004-637X/823/2/102".into());
    evolution_source.url = Some("https://arxiv.org/abs/1604.08592".into());
    evolution_source.validate()?;

    let cooling_source_reference = ScientificSourceReference {
        source_id: SourceId::from(COOLING_SOURCE_ID),
        locator: Some("thick-hydrogen C/O-core cooling sequences".into()),
    };
    let mut cooling_source = ScientificSource::new(
        COOLING_SOURCE_ID,
        "New Cooling Sequences for Old White Dwarfs",
    )?;
    cooling_source.authors = vec!["Antoine Bédard et al.".into()];
    cooling_source.publication = Some("The Astrophysical Journal".into());
    cooling_source.publication_year = Some(2020);
    cooling_source.doi = Some("10.3847/1538-4357/abafbe".into());
    cooling_source.url = Some("https://www.astro.umontreal.ca/~bergeron/CoolingModels/".into());
    cooling_source.validate()?;

    let prescription = GeneratingPrescription::new(
        PRESCRIPTION_ID,
        PRIMARY_MASS_PRESCRIPTION_NAMESPACE,
        "1",
        EvidenceLevel::PhysicalProxy,
        "Samples the configured Kroupa-form IMF as a system-primary mass proxy",
        vec![source_reference.clone()],
    )?;
    let companion_ratio_prescription = GeneratingPrescription::new(
        COMPANION_RATIO_PRESCRIPTION_ID,
        COMPANION_MASS_RATIO_PRESCRIPTION_NAMESPACE,
        "1",
        EvidenceLevel::PhysicalProxy,
        "Samples the configured period-marginal companion mass-ratio power law",
        vec![companion_source_reference.clone()],
    )?;
    let companion_mass_prescription = GeneratingPrescription::new(
        COMPANION_MASS_PRESCRIPTION_ID,
        "stellar_birth/companion_mass_derivation/v1",
        "1",
        EvidenceLevel::PhysicalProxy,
        "Derives companion initial mass as primary initial mass times mass ratio",
        vec![companion_source_reference.clone()],
    )?;
    let member_count_prescription = GeneratingPrescription::new(
        MEMBER_COUNT_PRESCRIPTION_ID,
        "stellar_birth/system_multiplicity/v1",
        "1",
        EvidenceLevel::PhysicalProxy,
        "Samples a mass-conditioned categorical stellar member count",
        vec![],
    )?;
    let member_role_prescription = GeneratingPrescription::new(
        MEMBER_ROLE_PRESCRIPTION_ID,
        "stellar_birth/member_role/v1",
        "1",
        EvidenceLevel::PhysicalProxy,
        "Classifies the initially most massive member as primary and all others as companions",
        vec![],
    )?;
    let population_prescription = GeneratingPrescription::new(
        POPULATION_PRESCRIPTION_ID,
        "stellar_region/system_population/v1",
        "1",
        EvidenceLevel::PhysicalProxy,
        "Assigns a geometrical Stellar Population from local density-component weights",
        vec![],
    )?;
    let age_prescription = GeneratingPrescription::new(
        AGE_PRESCRIPTION_ID,
        "population_history/age/v1",
        "1",
        EvidenceLevel::PhysicalProxy,
        "Samples the configured population-conditioned truncated-normal Stellar Age proxy",
        vec![],
    )?;
    let iron_prescription = GeneratingPrescription::new(
        IRON_PRESCRIPTION_ID,
        "population_history/metallicity/v1",
        "1",
        EvidenceLevel::PhysicalProxy,
        "Samples a radial-gradient-adjusted iron-abundance proxy for the assigned population",
        vec![],
    )?;
    let alpha_prescription = GeneratingPrescription::new(
        ALPHA_PRESCRIPTION_ID,
        "stellar_chemistry/alpha/v1",
        "1",
        EvidenceLevel::PhysicalProxy,
        "Samples alpha enhancement conditionally on population and iron abundance",
        vec![],
    )?;
    let chemistry_prescription = GeneratingPrescription::new(
        CHEMISTRY_PRESCRIPTION_ID,
        "stellar_chemistry/composition_derivation/v1",
        "1",
        EvidenceLevel::PhysicalProxy,
        "Derives coherent global metallicity and initial mass fractions from iron and alpha abundance",
        vec![],
    )?;
    let evolution_prescription = GeneratingPrescription::new(
        EVOLUTION_PRESCRIPTION_ID,
        "stellar_evolution/mist_v1_2_nonrotating_solar_scaled/v1",
        "1",
        EvidenceLevel::PhysicalProxy,
        "Evaluates the bundled reduced MIST track grid without clamping unsupported inputs",
        vec![evolution_source_reference.clone()],
    )?;
    let cooling_prescription = GeneratingPrescription::new(
        COOLING_PRESCRIPTION_ID,
        "white_dwarf_cooling/montreal_bedard_2020_thick_h/v1",
        "1",
        EvidenceLevel::PhysicalProxy,
        "Interpolates the locally supplied Montréal thick-hydrogen C/O-core cooling grid",
        vec![cooling_source_reference.clone()],
    )?;
    let imf = birth_mass_model.initial_mass_function;
    let model_fingerprint = catalog_model_fingerprint(
        birth_mass_model,
        population_history_model,
        evolution_model_fingerprint,
    );
    let model_realization_id = ModelRealizationId::from(format!(
        "model-realization.stellar-catalog-v1.{model_fingerprint}.seed-{seed}"
    ));
    let model_realization = ModelRealization {
        id: model_realization_id.clone(),
        version: "1".into(),
        seed,
        description: format!(
            "Coherent birth, population-history, chemistry, and evolution configuration; primary-mass bounds [{}, {}, {}] M_sun; evolution fingerprint {}",
            imf.minimum_mass_msun,
            imf.break_mass_msun,
            imf.maximum_mass_msun,
            evolution_model_fingerprint,
        ),
    };
    let source_catalog = ScientificSourceCatalog::new(
        seed,
        vec![source, companion_source, evolution_source, cooling_source],
        vec![
            prescription,
            companion_ratio_prescription,
            companion_mass_prescription,
            member_count_prescription,
            member_role_prescription,
            population_prescription,
            age_prescription,
            iron_prescription,
            alpha_prescription,
            chemistry_prescription,
            evolution_prescription,
            cooling_prescription,
        ],
        vec![model_realization],
        vec![],
    )?;

    let mut outcomes = Vec::with_capacity(catalog.systems.len());
    for system in &catalog.systems {
        let system_object_id =
            ObjectId::from(format!("indexed-u64-le:{:016x}/stellar-system", system.id));
        let population_claim_id = ClaimId::from(format!("{system_object_id}/stellar_population"));
        let population_draw_address = RandomDrawAddress::new(
            "blake3-xof",
            "1",
            "stellar_region/system_population/v1",
            system_object_id.clone(),
            "stellar_population",
            0,
        )?;
        let population_provenance = ClaimProvenance::new(
            system_object_id.clone(),
            "stellar_population",
            EvidenceLevel::PhysicalProxy,
            POPULATION_PRESCRIPTION_ID,
            vec![],
            ClaimApplicability::inside_domain(
                "configured local Galactic density components",
                std::collections::BTreeMap::new(),
            )?,
            ClaimUncertainty::new(
                Some(AleatoryVariation::new(
                    UncertaintyRepresentation::not_quantified(
                        "categorical probabilities are determined by local density-component weights",
                    )?,
                )?),
                Some(EpistemicUncertainty::new(
                    UncertaintyRepresentation::not_quantified(
                        "the smooth density components are a geometrical population proxy",
                    )?,
                    Some(model_realization_id.clone()),
                    None,
                )?),
            )?,
            None,
            Some(population_draw_address),
        )?;
        let population_claim = ScientificClaim::new(
            population_claim_id.clone(),
            StellarClaimValue::Population(system.population),
            population_provenance,
        )?;
        outcomes.push(ClaimOutcome::Accepted(
            population_claim,
            ValidationReceipt::new(
                "stellar-population-assignment-support",
                "1",
                vec![],
                vec![ConstraintEvaluation::passed(
                    "known-stellar-population",
                    None,
                    None,
                    None,
                    Some("the sampled category is one of the configured density components"),
                )?],
            )?,
        ));

        let distribution = match system.population {
            StellarPopulation::ThinDisk => population_history_model.thin_disk,
            StellarPopulation::ThickDisk => population_history_model.thick_disk,
            StellarPopulation::Halo => population_history_model.halo,
        };
        let age_claim_id = ClaimId::from(format!("{system_object_id}/stellar_age_gyr"));
        let age_draw_address = RandomDrawAddress::new(
            "blake3-seeded-chacha8-indexed",
            "1",
            "population_history/age/v1",
            system_object_id.clone(),
            "stellar_age_gyr",
            0,
        )?;
        let age_provenance = ClaimProvenance::new(
            system_object_id.clone(),
            "stellar_age_gyr",
            EvidenceLevel::PhysicalProxy,
            AGE_PRESCRIPTION_ID,
            vec![],
            ClaimApplicability::inside_domain(
                "configured Stellar Age support",
                std::collections::BTreeMap::from([(
                    "stellar_age_gyr".into(),
                    system.history.age_gyr,
                )]),
            )?,
            ClaimUncertainty::new(
                Some(AleatoryVariation::new(
                    UncertaintyRepresentation::parametric_distribution(
                        "truncated normal",
                        std::collections::BTreeMap::from([
                            ("mean_gyr".into(), distribution.age_gyr.mean),
                            (
                                "standard_deviation_gyr".into(),
                                distribution.age_gyr.standard_deviation,
                            ),
                            ("minimum_gyr".into(), distribution.age_gyr.minimum),
                            ("maximum_gyr".into(), distribution.age_gyr.maximum),
                        ]),
                    )?,
                )?),
                Some(EpistemicUncertainty::new(
                    UncertaintyRepresentation::not_quantified(
                        "the population age distribution is an engineering proxy synthesized from heterogeneous surveys",
                    )?,
                    Some(model_realization_id.clone()),
                    None,
                )?),
            )?,
            Some(ClaimDerivation::new(vec![population_claim_id.clone()])?),
            Some(age_draw_address),
        )?;
        outcomes.push(ClaimOutcome::Accepted(
            ScientificClaim::new(
                age_claim_id.clone(),
                StellarClaimValue::StellarAgeGyr(system.history.age_gyr),
                age_provenance,
            )?,
            ValidationReceipt::new(
                "stellar-age-support",
                "1",
                vec![population_claim_id.clone()],
                vec![
                    ConstraintEvaluation::passed(
                        "at-or-above-minimum-age",
                        Some(system.history.age_gyr),
                        Some(distribution.age_gyr.minimum),
                        Some(system.history.age_gyr - distribution.age_gyr.minimum),
                        None::<String>,
                    )?,
                    ConstraintEvaluation::passed(
                        "at-or-below-maximum-age",
                        Some(system.history.age_gyr),
                        Some(distribution.age_gyr.maximum),
                        Some(distribution.age_gyr.maximum - system.history.age_gyr),
                        None::<String>,
                    )?,
                ],
            )?,
        ));

        let iron_claim_id = ClaimId::from(format!("{system_object_id}/iron_abundance_feh"));
        let iron_draw_address = RandomDrawAddress::new(
            "blake3-seeded-chacha8-indexed",
            "1",
            "population_history/metallicity/v1",
            system_object_id.clone(),
            "iron_abundance_feh",
            0,
        )?;
        let iron_provenance = ClaimProvenance::new(
            system_object_id.clone(),
            "iron_abundance_feh",
            EvidenceLevel::PhysicalProxy,
            IRON_PRESCRIPTION_ID,
            vec![],
            ClaimApplicability::inside_domain(
                "configured population and radial iron-abundance support",
                std::collections::BTreeMap::from([(
                    "iron_abundance_feh".into(),
                    system.history.chemistry.iron_abundance_feh,
                )]),
            )?,
            ClaimUncertainty::new(
                Some(AleatoryVariation::new(
                    UncertaintyRepresentation::parametric_distribution(
                        "radially adjusted truncated normal",
                        std::collections::BTreeMap::from([
                            ("base_mean_dex".into(), distribution.iron_abundance_feh.mean),
                            (
                                "standard_deviation_dex".into(),
                                distribution.iron_abundance_feh.standard_deviation,
                            ),
                            (
                                "minimum_dex".into(),
                                distribution.iron_abundance_feh.minimum,
                            ),
                            (
                                "maximum_dex".into(),
                                distribution.iron_abundance_feh.maximum,
                            ),
                            (
                                "radial_gradient_dex_per_kpc".into(),
                                distribution.iron_abundance_radial_gradient.dex_per_kpc,
                            ),
                        ]),
                    )?,
                )?),
                Some(EpistemicUncertainty::new(
                    UncertaintyRepresentation::not_quantified(
                        "the geometrical population proxy omits formation radius, migration, and age-metallicity covariance",
                    )?,
                    Some(model_realization_id.clone()),
                    None,
                )?),
            )?,
            Some(ClaimDerivation::new(vec![population_claim_id.clone()])?),
            Some(iron_draw_address),
        )?;
        outcomes.push(ClaimOutcome::Accepted(
            ScientificClaim::new(
                iron_claim_id.clone(),
                StellarClaimValue::IronAbundanceFeH(system.history.chemistry.iron_abundance_feh),
                iron_provenance,
            )?,
            ValidationReceipt::new(
                "stellar-iron-abundance-support",
                "1",
                vec![population_claim_id.clone()],
                vec![
                    ConstraintEvaluation::passed(
                        "at-or-above-configured-iron-minimum",
                        Some(system.history.chemistry.iron_abundance_feh),
                        Some(distribution.iron_abundance_feh.minimum),
                        Some(
                            system.history.chemistry.iron_abundance_feh
                                - distribution.iron_abundance_feh.minimum,
                        ),
                        None::<String>,
                    )?,
                    ConstraintEvaluation::passed(
                        "at-or-below-configured-iron-maximum",
                        Some(system.history.chemistry.iron_abundance_feh),
                        Some(distribution.iron_abundance_feh.maximum),
                        Some(
                            distribution.iron_abundance_feh.maximum
                                - system.history.chemistry.iron_abundance_feh,
                        ),
                        None::<String>,
                    )?,
                ],
            )?,
        ));

        let alpha_claim_id =
            ClaimId::from(format!("{system_object_id}/alpha_enhancement_alpha_fe"));
        let alpha_draw_address = RandomDrawAddress::new(
            "blake3-seeded-chacha8-indexed",
            "1",
            "stellar_chemistry/alpha/v1",
            system_object_id.clone(),
            "alpha_enhancement_alpha_fe",
            0,
        )?;
        let alpha = distribution.alpha_enhancement;
        let alpha_provenance = ClaimProvenance::new(
            system_object_id.clone(),
            "alpha_enhancement_alpha_fe",
            EvidenceLevel::PhysicalProxy,
            ALPHA_PRESCRIPTION_ID,
            vec![],
            ClaimApplicability::inside_domain(
                "configured conditional alpha-enhancement support",
                std::collections::BTreeMap::from([
                    (
                        "iron_abundance_feh".into(),
                        system.history.chemistry.iron_abundance_feh,
                    ),
                    (
                        "alpha_enhancement_alpha_fe".into(),
                        system.history.chemistry.alpha_enhancement_alpha_fe,
                    ),
                ]),
            )?,
            ClaimUncertainty::new(
                Some(AleatoryVariation::new(
                    UncertaintyRepresentation::parametric_distribution(
                        "conditional truncated normal",
                        std::collections::BTreeMap::from([
                            ("standard_deviation_dex".into(), alpha.standard_deviation),
                            ("minimum_dex".into(), alpha.minimum),
                            ("maximum_dex".into(), alpha.maximum),
                        ]),
                    )?,
                )?),
                Some(EpistemicUncertainty::new(
                    UncertaintyRepresentation::not_quantified(
                        "one alpha coordinate cannot represent element-by-element abundance sequences",
                    )?,
                    Some(model_realization_id.clone()),
                    None,
                )?),
            )?,
            Some(ClaimDerivation::new(vec![
                population_claim_id.clone(),
                iron_claim_id.clone(),
            ])?),
            Some(alpha_draw_address),
        )?;
        outcomes.push(ClaimOutcome::Accepted(
            ScientificClaim::new(
                alpha_claim_id.clone(),
                StellarClaimValue::AlphaEnhancementAlphaFe(
                    system.history.chemistry.alpha_enhancement_alpha_fe,
                ),
                alpha_provenance,
            )?,
            ValidationReceipt::new(
                "stellar-alpha-enhancement-support",
                "1",
                vec![population_claim_id.clone(), iron_claim_id.clone()],
                vec![
                    ConstraintEvaluation::passed(
                        "at-or-above-configured-alpha-minimum",
                        Some(system.history.chemistry.alpha_enhancement_alpha_fe),
                        Some(alpha.minimum),
                        Some(system.history.chemistry.alpha_enhancement_alpha_fe - alpha.minimum),
                        None::<String>,
                    )?,
                    ConstraintEvaluation::passed(
                        "at-or-below-configured-alpha-maximum",
                        Some(system.history.chemistry.alpha_enhancement_alpha_fe),
                        Some(alpha.maximum),
                        Some(alpha.maximum - system.history.chemistry.alpha_enhancement_alpha_fe),
                        None::<String>,
                    )?,
                ],
            )?,
        ));

        let chemistry_claim_id = ClaimId::from(format!("{system_object_id}/stellar_chemistry"));
        let chemistry = system.history.chemistry;
        let chemistry_provenance = ClaimProvenance::new(
            system_object_id.clone(),
            "stellar_chemistry",
            EvidenceLevel::PhysicalProxy,
            CHEMISTRY_PRESCRIPTION_ID,
            vec![],
            ClaimApplicability::inside_domain(
                "configured alpha-corrected initial-composition derivation",
                std::collections::BTreeMap::from([
                    (
                        "global_metallicity_mh".into(),
                        chemistry.global_metallicity_mh,
                    ),
                    (
                        "hydrogen_mass_fraction_x".into(),
                        chemistry.hydrogen_mass_fraction_x,
                    ),
                    (
                        "helium_mass_fraction_y".into(),
                        chemistry.helium_mass_fraction_y,
                    ),
                    (
                        "metal_mass_fraction_z".into(),
                        chemistry.metal_mass_fraction_z,
                    ),
                ]),
            )?,
            ClaimUncertainty::not_quantified(
                "the deterministic composition conversion inherits the iron and alpha proxy limitations",
            )?,
            Some(ClaimDerivation::new(vec![
                iron_claim_id.clone(),
                alpha_claim_id.clone(),
            ])?),
            None,
        )?;
        let composition_sum = chemistry.hydrogen_mass_fraction_x
            + chemistry.helium_mass_fraction_y
            + chemistry.metal_mass_fraction_z;
        outcomes.push(ClaimOutcome::Accepted(
            ScientificClaim::new(
                chemistry_claim_id.clone(),
                StellarClaimValue::StellarChemistry(chemistry),
                chemistry_provenance,
            )?,
            ValidationReceipt::new(
                "stellar-chemistry-composition-consistency",
                "1",
                vec![iron_claim_id, alpha_claim_id],
                vec![ConstraintEvaluation::passed(
                    "mass-fractions-sum-to-unity",
                    Some(composition_sum),
                    Some(1.0),
                    Some(1.0e-12 - (composition_sum - 1.0).abs()),
                    None::<String>,
                )?],
            )?,
        ));

        let Some(primary) = system.members.first() else {
            continue;
        };
        let object_id = stellar_member_object_id(system.id, primary.birth.id);
        let claim_id = ClaimId::from(format!("{object_id}/{INITIAL_STELLAR_MASS_CLAIM_KEY}"));
        let draw_address = RandomDrawAddress::new(
            "blake3-seeded-chacha8-indexed",
            "1",
            PRIMARY_MASS_PRESCRIPTION_NAMESPACE,
            object_id.clone(),
            INITIAL_STELLAR_MASS_CLAIM_KEY,
            0,
        )?;
        let aleatory = AleatoryVariation::new(UncertaintyRepresentation::parametric_distribution(
            "configured two-segment power law",
            std::collections::BTreeMap::from([
                ("minimum_mass_msolar".into(), imf.minimum_mass_msun),
                ("break_mass_msolar".into(), imf.break_mass_msun),
                ("maximum_mass_msolar".into(), imf.maximum_mass_msun),
                ("low_mass_exponent".into(), imf.low_mass_exponent),
                ("high_mass_exponent".into(), imf.high_mass_exponent),
            ]),
        )?)?;
        let epistemic = EpistemicUncertainty::new(
            UncertaintyRepresentation::not_quantified(
                "the primary-mass proxy is not a closed system-primary IMF",
            )?,
            Some(model_realization_id.clone()),
            None,
        )?;
        let provenance = ClaimProvenance::new(
            object_id.clone(),
            INITIAL_STELLAR_MASS_CLAIM_KEY,
            EvidenceLevel::PhysicalProxy,
            PRESCRIPTION_ID,
            vec![source_reference.clone()],
            ClaimApplicability::inside_domain(
                "configured stellar primary-mass proxy support",
                std::collections::BTreeMap::from([(
                    "initial_stellar_mass_msolar".into(),
                    primary.birth.initial_mass_msun,
                )]),
            )?,
            ClaimUncertainty::new(Some(aleatory), Some(epistemic))?,
            None,
            Some(draw_address),
        )?;
        let claim = ScientificClaim::new(
            claim_id,
            StellarClaimValue::InitialStellarMassMsolar(primary.birth.initial_mass_msun),
            provenance,
        )?;
        let receipt = ValidationReceipt::new(
            "stellar-birth-primary-mass-support",
            "1",
            vec![],
            vec![
                ConstraintEvaluation::passed(
                    "at-or-above-configured-primary-mass-minimum",
                    Some(primary.birth.initial_mass_msun),
                    Some(imf.minimum_mass_msun),
                    Some(primary.birth.initial_mass_msun - imf.minimum_mass_msun),
                    None::<String>,
                )?,
                ConstraintEvaluation::passed(
                    "at-or-below-configured-primary-mass-maximum",
                    Some(primary.birth.initial_mass_msun),
                    Some(imf.maximum_mass_msun),
                    Some(imf.maximum_mass_msun - primary.birth.initial_mass_msun),
                    None::<String>,
                )?,
            ],
        )?;
        outcomes.push(ClaimOutcome::Accepted(claim, receipt));

        let primary_mass_claim_id =
            ClaimId::from(format!("{object_id}/{INITIAL_STELLAR_MASS_CLAIM_KEY}"));
        let multiplicity = birth_mass_model
            .multiplicity_bins
            .iter()
            .find(|bin| primary.birth.initial_mass_msun <= bin.maximum_primary_mass_msun)
            .expect("validated multiplicity coverage");
        let member_count_claim_id =
            ClaimId::from(format!("{system_object_id}/stellar_member_count"));
        let member_count_draw_address = RandomDrawAddress::new(
            "blake3-seeded-chacha8-indexed",
            "1",
            "stellar_birth/system_multiplicity/v1",
            system_object_id.clone(),
            "stellar_member_count",
            0,
        )?;
        let member_count = u8::try_from(system.members.len())
            .expect("validated birth-system member count fits u8");
        let member_count_provenance = ClaimProvenance::new(
            system_object_id.clone(),
            "stellar_member_count",
            EvidenceLevel::PhysicalProxy,
            MEMBER_COUNT_PRESCRIPTION_ID,
            vec![],
            ClaimApplicability::inside_domain(
                "configured primary-mass-conditioned multiplicity table",
                std::collections::BTreeMap::from([
                    (
                        "primary_initial_mass_msolar".into(),
                        primary.birth.initial_mass_msun,
                    ),
                    ("stellar_member_count".into(), f64::from(member_count)),
                ]),
            )?,
            ClaimUncertainty::new(
                Some(AleatoryVariation::new(
                    UncertaintyRepresentation::not_quantified(
                        "categorical single, binary, triple, and higher-order probabilities",
                    )?,
                )?),
                Some(EpistemicUncertainty::new(
                    UncertaintyRepresentation::not_quantified(
                        "the mass-conditioned multiplicity table combines surveys and engineered higher-order splits",
                    )?,
                    Some(model_realization_id.clone()),
                    None,
                )?),
            )?,
            Some(ClaimDerivation::new(vec![primary_mass_claim_id.clone()])?),
            Some(member_count_draw_address),
        )?;
        outcomes.push(ClaimOutcome::Accepted(
            ScientificClaim::new(
                member_count_claim_id.clone(),
                StellarClaimValue::StellarMemberCount(member_count),
                member_count_provenance,
            )?,
            ValidationReceipt::new(
                "stellar-birth-member-count-support",
                "1",
                vec![primary_mass_claim_id.clone()],
                vec![
                    ConstraintEvaluation::passed(
                        "at-least-one-stellar-member",
                        Some(f64::from(member_count)),
                        Some(1.0),
                        Some(f64::from(member_count) - 1.0),
                        None::<String>,
                    )?,
                    ConstraintEvaluation::passed(
                        "inside-configured-higher-order-cap",
                        Some(f64::from(member_count)),
                        Some(f64::from(multiplicity.representative_higher_order_members)),
                        Some(
                            f64::from(multiplicity.representative_higher_order_members)
                                - f64::from(member_count),
                        ),
                        None::<String>,
                    )?,
                ],
            )?,
        ));
        let mut member_mass_claim_ids = vec![primary_mass_claim_id.clone()];
        for companion in system.members.iter().skip(1) {
            let companion_object_id = stellar_member_object_id(system.id, companion.birth.id);
            let ratio = companion
                .birth
                .mass_ratio_to_primary
                .expect("companion has a mass ratio");
            let ratio_claim_id = ClaimId::from(format!(
                "{companion_object_id}/{COMPANION_MASS_RATIO_CLAIM_KEY}"
            ));
            let ratio_draw_address = RandomDrawAddress::new(
                "blake3-xof",
                "1",
                COMPANION_MASS_RATIO_PRESCRIPTION_NAMESPACE,
                companion_object_id.clone(),
                COMPANION_MASS_RATIO_CLAIM_KEY,
                0,
            )?;
            let minimum_ratio = multiplicity.minimum_mass_ratio.max(
                birth_mass_model.minimum_companion_mass_msun / primary.birth.initial_mass_msun,
            );
            let ratio_provenance = ClaimProvenance::new(
                companion_object_id.clone(),
                COMPANION_MASS_RATIO_CLAIM_KEY,
                EvidenceLevel::PhysicalProxy,
                COMPANION_RATIO_PRESCRIPTION_ID,
                vec![companion_source_reference.clone()],
                ClaimApplicability::inside_domain(
                    "configured companion mass-ratio proxy support",
                    std::collections::BTreeMap::from([
                        (
                            "primary_initial_mass_msolar".into(),
                            primary.birth.initial_mass_msun,
                        ),
                        ("companion_mass_ratio".into(), ratio),
                    ]),
                )?,
                ClaimUncertainty::new(
                    Some(AleatoryVariation::new(
                        UncertaintyRepresentation::parametric_distribution(
                            "configured bounded power law",
                            std::collections::BTreeMap::from([
                                ("minimum_mass_ratio".into(), minimum_ratio),
                                ("maximum_mass_ratio".into(), 1.0),
                                ("power".into(), multiplicity.mass_ratio_power),
                            ]),
                        )?,
                    )?),
                    Some(EpistemicUncertainty::new(
                        UncertaintyRepresentation::not_quantified(
                            "the period-marginal mass-ratio proxy omits orbital-period covariance",
                        )?,
                        Some(model_realization_id.clone()),
                        None,
                    )?),
                )?,
                Some(ClaimDerivation::new(vec![primary_mass_claim_id.clone()])?),
                Some(ratio_draw_address),
            )?;
            let ratio_claim = ScientificClaim::new(
                ratio_claim_id.clone(),
                StellarClaimValue::CompanionMassRatio(ratio),
                ratio_provenance,
            )?;
            let ratio_receipt = ValidationReceipt::new(
                "stellar-birth-companion-mass-ratio-support",
                "1",
                vec![primary_mass_claim_id.clone()],
                vec![
                    ConstraintEvaluation::passed(
                        "at-or-above-minimum-mass-ratio",
                        Some(ratio),
                        Some(minimum_ratio),
                        Some(ratio - minimum_ratio),
                        None::<String>,
                    )?,
                    ConstraintEvaluation::passed(
                        "at-or-below-unity-mass-ratio",
                        Some(ratio),
                        Some(1.0),
                        Some(1.0 - ratio),
                        None::<String>,
                    )?,
                ],
            )?;
            outcomes.push(ClaimOutcome::Accepted(ratio_claim, ratio_receipt));

            let mass_claim_id = ClaimId::from(format!(
                "{companion_object_id}/{INITIAL_STELLAR_MASS_CLAIM_KEY}"
            ));
            let mass_provenance = ClaimProvenance::new(
                companion_object_id.clone(),
                INITIAL_STELLAR_MASS_CLAIM_KEY,
                EvidenceLevel::PhysicalProxy,
                COMPANION_MASS_PRESCRIPTION_ID,
                vec![companion_source_reference.clone()],
                ClaimApplicability::inside_domain(
                    "configured primary-constrained companion pairing support",
                    std::collections::BTreeMap::from([
                        (
                            "initial_stellar_mass_msolar".into(),
                            companion.birth.initial_mass_msun,
                        ),
                        ("companion_mass_ratio".into(), ratio),
                    ]),
                )?,
                ClaimUncertainty::not_quantified(
                    "the value is deterministically derived from the sampled primary mass and mass ratio",
                )?,
                Some(ClaimDerivation::new(vec![
                    primary_mass_claim_id.clone(),
                    ratio_claim_id.clone(),
                ])?),
                None,
            )?;
            let mass_claim = ScientificClaim::new(
                mass_claim_id.clone(),
                StellarClaimValue::InitialStellarMassMsolar(companion.birth.initial_mass_msun),
                mass_provenance,
            )?;
            let mass_receipt = ValidationReceipt::new(
                "stellar-birth-companion-mass-support",
                "1",
                vec![primary_mass_claim_id.clone(), ratio_claim_id],
                vec![
                    ConstraintEvaluation::passed(
                        "at-or-above-stellar-mass-floor",
                        Some(companion.birth.initial_mass_msun),
                        Some(birth_mass_model.minimum_companion_mass_msun),
                        Some(
                            companion.birth.initial_mass_msun
                                - birth_mass_model.minimum_companion_mass_msun,
                        ),
                        None::<String>,
                    )?,
                    ConstraintEvaluation::passed(
                        "not-more-massive-than-primary",
                        Some(companion.birth.initial_mass_msun),
                        Some(primary.birth.initial_mass_msun),
                        Some(primary.birth.initial_mass_msun - companion.birth.initial_mass_msun),
                        None::<String>,
                    )?,
                ],
            )?;
            outcomes.push(ClaimOutcome::Accepted(mass_claim, mass_receipt));
            member_mass_claim_ids.push(mass_claim_id);
        }

        let mut role_input_claims = member_mass_claim_ids.clone();
        role_input_claims.push(member_count_claim_id);
        for member in &system.members {
            let member_object_id = stellar_member_object_id(system.id, member.birth.id);
            let role_claim_id = ClaimId::from(format!(
                "{member_object_id}/{STELLAR_MEMBER_ROLE_CLAIM_KEY}"
            ));
            let role_provenance = ClaimProvenance::new(
                member_object_id,
                STELLAR_MEMBER_ROLE_CLAIM_KEY,
                EvidenceLevel::PhysicalProxy,
                MEMBER_ROLE_PRESCRIPTION_ID,
                vec![],
                ClaimApplicability::inside_domain(
                    "generated stellar birth-system member ordering",
                    std::collections::BTreeMap::from([(
                        "member_count".into(),
                        system.members.len() as f64,
                    )]),
                )?,
                ClaimUncertainty::not_quantified(
                    "the role is an exact structural classification of generated initial masses",
                )?,
                Some(ClaimDerivation::new(role_input_claims.clone())?),
                None,
            )?;
            let role_claim = ScientificClaim::new(
                role_claim_id,
                StellarClaimValue::MemberRole(member.birth.role),
                role_provenance,
            )?;
            let role_receipt = ValidationReceipt::new(
                "stellar-birth-member-role-consistency",
                "1",
                role_input_claims.clone(),
                vec![
                    ConstraintEvaluation::passed(
                        "primary-is-initially-most-massive",
                        Some(primary.birth.initial_mass_msun),
                        Some(member.birth.initial_mass_msun),
                        Some(primary.birth.initial_mass_msun - member.birth.initial_mass_msun),
                        Some("the primary mass is no smaller than every classified member"),
                    )?,
                    ConstraintEvaluation::passed(
                        "role-matches-generated-member-order",
                        None,
                        None,
                        None,
                        Some(match member.birth.role {
                            StellarMemberRole::Primary => {
                                "the first generated member is the unique primary"
                            }
                            StellarMemberRole::Companion => {
                                "every non-primary generated member is a companion"
                            }
                        }),
                    )?,
                ],
            )?;
            outcomes.push(ClaimOutcome::Accepted(role_claim, role_receipt));
        }

        for member in &system.members {
            let member_object_id = stellar_member_object_id(system.id, member.birth.id);
            let initial_mass_claim_id = ClaimId::from(format!(
                "{member_object_id}/{INITIAL_STELLAR_MASS_CLAIM_KEY}"
            ));
            let evolution_inputs = vec![
                initial_mass_claim_id,
                age_claim_id.clone(),
                chemistry_claim_id.clone(),
            ];
            let evolution_provenance = |claim_key: &str| {
                ClaimProvenance::new(
                    member_object_id.clone(),
                    claim_key,
                    EvidenceLevel::PhysicalProxy,
                    EVOLUTION_PRESCRIPTION_ID,
                    vec![evolution_source_reference.clone()],
                    ClaimApplicability::inside_domain(
                        "attempted bundled MIST track evaluation",
                        std::collections::BTreeMap::from([
                            (
                                "initial_stellar_mass_msolar".into(),
                                member.birth.initial_mass_msun,
                            ),
                            ("stellar_age_gyr".into(), system.history.age_gyr),
                            (
                                "global_metallicity_mh".into(),
                                system.history.chemistry.global_metallicity_mh,
                            ),
                        ]),
                    )?,
                    ClaimUncertainty::new(
                        None,
                        Some(EpistemicUncertainty::new(
                            UncertaintyRepresentation::not_quantified(
                                "the coarse reduced-grid interpolation and omitted binary interaction are not quantified",
                            )?,
                            Some(model_realization_id.clone()),
                            None,
                        )?),
                    )?,
                    Some(ClaimDerivation::new(evolution_inputs.clone())?),
                    None,
                )
            };

            match &member.evolution {
                Ok(snapshot) => {
                    let state_key = "evolutionary_state";
                    let state_claim = ScientificClaim::new(
                        format!("{member_object_id}/{state_key}"),
                        StellarClaimValue::EvolutionaryState(snapshot.state),
                        evolution_provenance(state_key)?,
                    )?;
                    outcomes.push(ClaimOutcome::Accepted(
                        state_claim,
                        ValidationReceipt::new(
                            "stellar-evolution-snapshot-consistency",
                            "1",
                            evolution_inputs.clone(),
                            vec![ConstraintEvaluation::passed(
                                "state-classified-from-track-phase",
                                Some(snapshot.raw_eep),
                                None,
                                None,
                                Some(snapshot.state.label()),
                            )?],
                        )?,
                    ));

                    let mut scalars = vec![
                        (
                            "current_stellar_mass_msolar",
                            StellarClaimValue::CurrentStellarMassMsolar(snapshot.current_mass_msun),
                            snapshot.current_mass_msun,
                        ),
                        (
                            "source_metallicity_coordinate_mh",
                            StellarClaimValue::SourceMetallicityCoordinateMh(
                                snapshot.source_metallicity_coordinate_mh,
                            ),
                            snapshot.source_metallicity_coordinate_mh,
                        ),
                        (
                            "zero_age_main_sequence_age_gyr",
                            StellarClaimValue::ZeroAgeMainSequenceAgeGyr(snapshot.zams_age_gyr),
                            snapshot.zams_age_gyr,
                        ),
                        (
                            "terminal_age_main_sequence_age_gyr",
                            StellarClaimValue::TerminalAgeMainSequenceAgeGyr(snapshot.tams_age_gyr),
                            snapshot.tams_age_gyr,
                        ),
                        (
                            "main_sequence_lifetime_gyr",
                            StellarClaimValue::MainSequenceLifetimeGyr(
                                snapshot.main_sequence_lifetime_gyr,
                            ),
                            snapshot.main_sequence_lifetime_gyr,
                        ),
                    ];
                    if let Some(value) = snapshot.fractional_main_sequence_age {
                        scalars.push((
                            "fractional_main_sequence_age",
                            StellarClaimValue::FractionalMainSequenceAge(value),
                            value,
                        ));
                    }
                    if let Some(value) = snapshot.white_dwarf_handoff_age_gyr {
                        scalars.push((
                            "white_dwarf_handoff_age_gyr",
                            StellarClaimValue::WhiteDwarfHandoffAgeGyr(value),
                            value,
                        ));
                    }
                    if let Some(value) = snapshot.cooling_age_gyr {
                        scalars.push((
                            "white_dwarf_cooling_age_gyr",
                            StellarClaimValue::WhiteDwarfCoolingAgeGyr(value),
                            value,
                        ));
                    }
                    if let Some(value) = snapshot.remnant_mass_msun {
                        scalars.push((
                            "remnant_mass_msolar",
                            StellarClaimValue::RemnantMassMsolar(value),
                            value,
                        ));
                    }
                    if let Some(value) = snapshot.luminosity_lsun {
                        scalars.push((
                            "luminosity_lsolar",
                            StellarClaimValue::LuminosityLsolar(value),
                            value,
                        ));
                    }
                    if let Some(value) = snapshot.radius_rsun {
                        scalars.push((
                            "radius_rsolar",
                            StellarClaimValue::RadiusRsolar(value),
                            value,
                        ));
                    }
                    if let Some(value) = snapshot.effective_temperature_k {
                        scalars.push((
                            "effective_temperature_k",
                            StellarClaimValue::EffectiveTemperatureK(value),
                            value,
                        ));
                    }
                    if let Some(value) = snapshot.surface_gravity_log10_cgs {
                        scalars.push((
                            "surface_gravity_log10_cgs",
                            StellarClaimValue::SurfaceGravityLog10Cgs(value),
                            value,
                        ));
                    }

                    for (claim_key, claim_value, value) in scalars {
                        let mut constraints = vec![ConstraintEvaluation::passed(
                            "finite-track-output",
                            Some(value),
                            None,
                            None,
                            None::<String>,
                        )?];
                        if claim_key == "current_stellar_mass_msolar" {
                            constraints.push(ConstraintEvaluation::passed(
                                "current-mass-not-above-initial-mass",
                                Some(value),
                                Some(member.birth.initial_mass_msun),
                                Some(member.birth.initial_mass_msun - value),
                                None::<String>,
                            )?);
                        }
                        if claim_key == "white_dwarf_cooling_age_gyr" {
                            constraints.push(ConstraintEvaluation::passed(
                                "nonnegative-white-dwarf-cooling-age",
                                Some(value),
                                Some(0.0),
                                Some(value),
                                None::<String>,
                            )?);
                        }
                        let uses_montreal_cooling =
                            snapshot.white_dwarf_cooling_model_version.is_some()
                                && matches!(
                                    claim_key,
                                    "luminosity_lsolar"
                                        | "radius_rsolar"
                                        | "effective_temperature_k"
                                        | "surface_gravity_log10_cgs"
                                );
                        let scalar_provenance = if uses_montreal_cooling {
                            ClaimProvenance::new(
                                member_object_id.clone(),
                                claim_key,
                                EvidenceLevel::PhysicalProxy,
                                COOLING_PRESCRIPTION_ID,
                                vec![cooling_source_reference.clone()],
                                ClaimApplicability::inside_domain(
                                    "locally supplied Montréal thick-hydrogen C/O-core cooling grid",
                                    std::collections::BTreeMap::from([
                                        (
                                            "remnant_mass_msolar".into(),
                                            snapshot.remnant_mass_msun.expect(
                                                "Montreal-backed snapshot has a remnant mass",
                                            ),
                                        ),
                                        (
                                            "white_dwarf_cooling_age_gyr".into(),
                                            snapshot.cooling_age_gyr.expect(
                                                "Montreal-backed snapshot has a cooling age",
                                            ),
                                        ),
                                    ]),
                                )?,
                                ClaimUncertainty::new(
                                    None,
                                    Some(EpistemicUncertainty::new(
                                        UncertaintyRepresentation::not_quantified(
                                            "cooling-grid interpolation and envelope-model systematics are not quantified",
                                        )?,
                                        Some(model_realization_id.clone()),
                                        None,
                                    )?),
                                )?,
                                Some(ClaimDerivation::new(evolution_inputs.clone())?),
                                None,
                            )?
                        } else {
                            evolution_provenance(claim_key)?
                        };
                        outcomes.push(ClaimOutcome::Accepted(
                            ScientificClaim::new(
                                format!("{member_object_id}/{claim_key}"),
                                claim_value,
                                scalar_provenance,
                            )?,
                            ValidationReceipt::new(
                                "stellar-evolution-snapshot-consistency",
                                "1",
                                evolution_inputs.clone(),
                                constraints,
                            )?,
                        ));
                    }

                    for (index, quality_flag) in snapshot.quality_flags.iter().enumerate() {
                        let claim_key = format!("evolution_quality_flag_{index}");
                        let cooling_flag = matches!(
                            quality_flag,
                            StellarEvolutionQualityFlag::WhiteDwarfCoolingOutsideModelCoverage
                                | StellarEvolutionQualityFlag::MontrealCoolingHybridModel
                                | StellarEvolutionQualityFlag::YoungWhiteDwarfCoolingZeroPointUncertain
                        );
                        let quality_provenance = if cooling_flag {
                            ClaimProvenance::new(
                                member_object_id.clone(),
                                claim_key.clone(),
                                EvidenceLevel::PhysicalProxy,
                                COOLING_PRESCRIPTION_ID,
                                vec![cooling_source_reference.clone()],
                                ClaimApplicability::inside_domain(
                                    "white-dwarf cooling backend diagnostics",
                                    std::collections::BTreeMap::new(),
                                )?,
                                ClaimUncertainty::new(
                                    None,
                                    Some(EpistemicUncertainty::new(
                                        UncertaintyRepresentation::not_quantified(
                                            "the cooling-backend limitation is recorded but not quantified",
                                        )?,
                                        Some(model_realization_id.clone()),
                                        None,
                                    )?),
                                )?,
                                Some(ClaimDerivation::new(evolution_inputs.clone())?),
                                None,
                            )?
                        } else {
                            evolution_provenance(&claim_key)?
                        };
                        outcomes.push(ClaimOutcome::Accepted(
                            ScientificClaim::new(
                                format!("{member_object_id}/{claim_key}"),
                                StellarClaimValue::EvolutionQualityFlag(*quality_flag),
                                quality_provenance,
                            )?,
                            ValidationReceipt::new(
                                "stellar-evolution-quality-limitation-recording",
                                "1",
                                evolution_inputs.clone(),
                                vec![ConstraintEvaluation::passed(
                                    "quality-limitation-recorded",
                                    None,
                                    None,
                                    None,
                                    Some(format!("{quality_flag:?}")),
                                )?],
                            )?,
                        ));
                    }

                    let cooling_reason = if snapshot.quality_flags.contains(
                        &StellarEvolutionQualityFlag::WhiteDwarfCoolingOutsideModelCoverage,
                    ) {
                        Some((
                            "white_dwarf_cooling_outside_model_coverage",
                            "the white dwarf lies outside the supplied cooling-grid coverage",
                            COOLING_PRESCRIPTION_ID,
                            vec![cooling_source_reference.clone()],
                            "supplied Montréal thick-hydrogen C/O-core cooling-grid coverage",
                        ))
                    } else if snapshot
                        .quality_flags
                        .contains(&StellarEvolutionQualityFlag::WhiteDwarfCoolingNotBundled)
                    {
                        Some((
                            "white_dwarf_cooling_not_bundled",
                            "no local white-dwarf cooling grid was supplied",
                            EVOLUTION_PRESCRIPTION_ID,
                            vec![evolution_source_reference.clone()],
                            "bundled MIST model without a long-term white-dwarf cooling grid",
                        ))
                    } else {
                        None
                    };
                    if let Some((
                        reason_code,
                        reason_detail,
                        prescription_id,
                        source_references,
                        calibrated_domain,
                    )) = cooling_reason
                    {
                        for (claim_key, missing) in [
                            ("luminosity_lsolar", snapshot.luminosity_lsun.is_none()),
                            ("radius_rsolar", snapshot.radius_rsun.is_none()),
                            (
                                "effective_temperature_k",
                                snapshot.effective_temperature_k.is_none(),
                            ),
                            (
                                "surface_gravity_log10_cgs",
                                snapshot.surface_gravity_log10_cgs.is_none(),
                            ),
                        ] {
                            if !missing {
                                continue;
                            }
                            let provenance = ClaimProvenance::new(
                                member_object_id.clone(),
                                claim_key,
                                EvidenceLevel::PhysicalProxy,
                                prescription_id,
                                source_references.clone(),
                                ClaimApplicability::outside_domain(
                                    calibrated_domain,
                                    std::collections::BTreeMap::from([
                                        (
                                            "remnant_mass_msolar".into(),
                                            snapshot
                                                .remnant_mass_msun
                                                .expect("white dwarf has a remnant mass"),
                                        ),
                                        (
                                            "white_dwarf_cooling_age_gyr".into(),
                                            snapshot
                                                .cooling_age_gyr
                                                .expect("white dwarf has a cooling age"),
                                        ),
                                    ]),
                                )?,
                                ClaimUncertainty::not_quantified(reason_detail)?,
                                Some(ClaimDerivation::new(evolution_inputs.clone())?),
                                None,
                            )?;
                            outcomes.push(ClaimOutcome::Unsupported(
                                provenance,
                                vec![UnsupportedReason::new(reason_code, reason_detail)?],
                            ));
                        }
                    }
                }
                Err(error) => {
                    let unsupported_provenance = ClaimProvenance::new(
                        member_object_id.clone(),
                        "evolutionary_state",
                        EvidenceLevel::PhysicalProxy,
                        EVOLUTION_PRESCRIPTION_ID,
                        vec![evolution_source_reference.clone()],
                        ClaimApplicability::outside_domain(
                            "bundled MIST track mass, metallicity, age, and terminal-phase coverage",
                            std::collections::BTreeMap::from([
                                (
                                    "initial_stellar_mass_msolar".into(),
                                    member.birth.initial_mass_msun,
                                ),
                                ("stellar_age_gyr".into(), system.history.age_gyr),
                                (
                                    "global_metallicity_mh".into(),
                                    system.history.chemistry.global_metallicity_mh,
                                ),
                            ]),
                        )?,
                        ClaimUncertainty::new(
                            None,
                            Some(EpistemicUncertainty::new(
                                UncertaintyRepresentation::not_quantified(
                                    "no value is generated outside the bundled track coverage",
                                )?,
                                Some(model_realization_id.clone()),
                                None,
                            )?),
                        )?,
                        Some(ClaimDerivation::new(evolution_inputs.clone())?),
                        None,
                    )?;
                    outcomes.push(ClaimOutcome::Unsupported(
                        unsupported_provenance,
                        vec![stellar_evolution_unsupported_reason(error)?],
                    ));
                }
            }
        }
    }

    let object_ids = outcomes
        .iter()
        .map(|outcome| outcome.provenance().object_id.clone())
        .collect::<std::collections::BTreeSet<_>>();
    let object_summaries = object_ids
        .into_iter()
        .map(|object_id| ObjectEvidenceSummary::from_outcomes(object_id, &outcomes))
        .collect::<Result<Vec<_>, _>>()?;
    ProvenanceDocument::new(source_catalog, outcomes, object_summaries)
}

fn stellar_evolution_unsupported_reason(
    error: &StellarEvolutionError,
) -> Result<UnsupportedReason, ProvenanceError> {
    let code = match error {
        StellarEvolutionError::InvalidModel => "stellar_evolution_invalid_model",
        StellarEvolutionError::InvalidInput { .. } => "stellar_evolution_invalid_input",
        StellarEvolutionError::OutsideMassGrid { .. } => "stellar_evolution_outside_mass_grid",
        StellarEvolutionError::OutsideMetallicityGrid { .. } => {
            "stellar_evolution_outside_metallicity_grid"
        }
        StellarEvolutionError::AgeBeforeTrack { .. } => "stellar_evolution_age_before_track",
        StellarEvolutionError::PostMainSequenceNotBundled { .. } => {
            "stellar_evolution_post_main_sequence_not_bundled"
        }
        StellarEvolutionError::UnsupportedCoreCollapse { .. } => {
            "stellar_evolution_unsupported_core_collapse"
        }
        StellarEvolutionError::PostAgbTrackIncomplete { .. } => {
            "stellar_evolution_post_agb_track_incomplete"
        }
        StellarEvolutionError::TrackEndedBeforeExpectedEndpoint { .. } => {
            "stellar_evolution_track_ended_before_expected_endpoint"
        }
    };
    UnsupportedReason::new(code, error.to_string())
}

fn catalog_model_fingerprint(
    birth_mass_model: &StellarBirthMassModel,
    population_history_model: &PopulationHistoryModel,
    evolution_model_fingerprint: &str,
) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"star_sim/stellar_catalog_model/v1");
    hasher.update(format!("{birth_mass_model:?}").as_bytes());
    hasher.update(format!("{population_history_model:?}").as_bytes());
    hasher.update(evolution_model_fingerprint.as_bytes());
    hasher.finalize().to_hex()[..16].to_owned()
}
