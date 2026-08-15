//! Domain payloads published through the catalog provenance seam.

use super::super::*;

/// Kind of structural node referenced by an orbital-hierarchy claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum OrbitalNodeClaimKind {
    StellarMember,
    Barycentre,
}

/// Stable reference to either a stellar member or a structural Barycentre.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OrbitalNodeClaim {
    pub stable_id: u64,
    pub kind: OrbitalNodeClaimKind,
}

/// Compact, serializable description of a generated Stellar Orbital Hierarchy.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StellarOrbitalHierarchyClaim {
    pub root: OrbitalNodeClaim,
    pub relative_orbit_count: u8,
    pub quadruple_topology: Option<QuadrupleTopology>,
}

/// One immutable bounded hierarchy candidate that failed its evaluated constraints.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StellarOrbitalHierarchyAttemptClaim {
    pub attempt: u16,
    pub candidate_root: Option<OrbitalNodeClaim>,
    pub candidate_relationships: Vec<RelativeStellarOrbitClaim>,
    pub constraints: Vec<StellarOrbitalHierarchyAttemptConstraint>,
}

/// One structural Barycentre and its two orbital child nodes.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RelativeStellarOrbitClaim {
    pub barycentre: OrbitalNodeClaim,
    pub left_child: OrbitalNodeClaim,
    pub right_child: OrbitalNodeClaim,
    pub orbit: RelativeStellarOrbit,
    pub sampling_attempt: u16,
    pub sampling_slot: u8,
}

/// Replay metadata and scale results for one Relative Stellar Orbit.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RelativeStellarOrbitScaleClaim {
    pub semimajor_axis_au: f64,
    pub period_days: f64,
    pub combined_mass_msun: f64,
    pub sampling_attempt: u16,
    pub sampling_slot: u8,
}

/// Per-member result of evaluating a Circumstellar S-Type Stability Zone.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CircumstellarStabilityClaim {
    UnboundedByStellarCompanion,
    CompanionLimited {
        model: HolmanWiegertSTypeModel,
        nominal_outer_critical_semimajor_axis_au: f64,
        fit_residual_lower_semimajor_axis_au: f64,
        limiting_companion_mass_msun: f64,
        companion_mass_fraction: f64,
        limiting_barycentre_id: u64,
    },
}

/// One explicitly calibrated Planet Occurrence channel result.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PlanetOccurrenceClaim {
    PlanetCount {
        count: u32,
        multiplicity_suppression_extrapolated: bool,
    },
    HasAtLeastOneGiant {
        multiplicity_suppression_extrapolated: bool,
    },
}

/// Observable properties and present-day orbit of one Explicit Planet Candidate.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ExplicitPlanetClaim {
    /// Stellar-member Host Scope containing the candidate's stable orbital domain.
    pub host_member_id: u64,
    /// Present-day Orbital Parent; currently the same S-type stellar member as the Host Scope.
    pub orbital_parent_member_id: u64,
    pub source_channel: ExplicitPlanetSourceChannel,
    pub source_cell_index: Option<u32>,
    pub source_cell_count: Option<u32>,
    pub transit_radius_rearth: Option<f64>,
    pub doppler_minimum_mass_mjup: Option<f64>,
    pub period_days: f64,
    pub semimajor_axis_au: f64,
}

/// Heterogeneous stellar values published through the catalog provenance seam.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StellarClaimValue {
    /// The geometrical Stellar Population assigned to a generated system.
    Population(StellarPopulation),
    /// Elapsed time since the system formed.
    StellarAgeGyr(f64),
    /// Sampled logarithmic iron abundance relative to the Sun.
    IronAbundanceFeH(f64),
    /// Sampled alpha-element enhancement relative to iron.
    AlphaEnhancementAlphaFe(f64),
    /// Coherent initial Stellar Chemistry derived for a system.
    StellarChemistry(StellarChemistry),
    /// Initial stellar mass relative to the Sun.
    InitialStellarMassMsolar(f64),
    /// Companion-to-primary initial-mass ratio.
    CompanionMassRatio(f64),
    /// Number of stellar members selected for a birth system.
    StellarMemberCount(u8),
    /// A member's role in the stellar birth system.
    MemberRole(StellarMemberRole),
    /// Present Evolutionary State from the bundled track evaluation.
    EvolutionaryState(EvolutionaryState),
    EvolutionQualityFlag(StellarEvolutionQualityFlag),
    CurrentStellarMassMsolar(f64),
    SourceMetallicityCoordinateMh(f64),
    ZeroAgeMainSequenceAgeGyr(f64),
    TerminalAgeMainSequenceAgeGyr(f64),
    MainSequenceLifetimeGyr(f64),
    FractionalMainSequenceAge(f64),
    WhiteDwarfHandoffAgeGyr(f64),
    WhiteDwarfCoolingAgeGyr(f64),
    RemnantMassMsolar(f64),
    LuminosityLsolar(f64),
    RadiusRsolar(f64),
    EffectiveTemperatureK(f64),
    SurfaceGravityLog10Cgs(f64),
    StellarOrbitalHierarchy(StellarOrbitalHierarchyClaim),
    OrbitalHierarchySamplingAttempt(StellarOrbitalHierarchyAttemptClaim),
    OrbitalHierarchySamplingExhaustion {
        attempted_candidates: u16,
    },
    QuadrupleTopology(QuadrupleTopology),
    RelativeStellarOrbit(RelativeStellarOrbitClaim),
    RelativeStellarOrbitScale(RelativeStellarOrbitScaleClaim),
    RelativeStellarOrbitEccentricity(f64),
    CircumstellarStability(CircumstellarStabilityClaim),
    PlanetOccurrence(PlanetOccurrenceClaim),
    CloseBinaryOccurrenceFactor(f64),
    StellarOrbitalQualityFlag(StellarOrbitalHierarchyQualityFlag),
    CircumstellarStabilityQualityFlag(CircumstellarStabilityQualityFlag),
    PlanetOccurrenceQualityFlag(PlanetOccurrenceQualityFlag),
    ExplicitPlanetQualityFlag(ExplicitPlanetQualityFlag),
    MDwarfOccurrenceCellSelection {
        index: u32,
        cell_count: u32,
    },
    ExplicitPlanetSourceChannel(ExplicitPlanetSourceChannel),
    PlanetTransitRadiusRearth(f64),
    PlanetDopplerMinimumMassMjup(f64),
    PlanetOrbitalPeriodDays(f64),
    PlanetSemimajorAxisAu(f64),
    ExplicitPlanet(ExplicitPlanetClaim),
}

impl ProvenanceValue for StellarClaimValue {
    fn validate_claim_value(
        &self,
        claim: &ScientificClaim<Self>,
        realized_claims: &std::collections::BTreeMap<ClaimId, &ScientificClaim<Self>>,
        known_objects: &std::collections::BTreeSet<ObjectId>,
    ) -> Result<(), ProvenanceError> {
        let invalid = |detail: &str| ProvenanceError::InvalidClaimValue {
            claim: claim.id.to_string(),
            detail: detail.into(),
        };
        let finite = |value: f64| value.is_finite();
        let positive = |value: f64| value.is_finite() && value > 0.0;
        let non_negative = |value: f64| value.is_finite() && value >= 0.0;
        let object_id = claim.provenance.object_id.as_str();
        let claim_key = claim.provenance.claim_key.as_str();
        let key_matches_value = match self {
            Self::StellarOrbitalHierarchy(_) | Self::OrbitalHierarchySamplingExhaustion { .. } => {
                claim_key == "stellar_orbital_hierarchy"
            }
            Self::OrbitalHierarchySamplingAttempt(_) => {
                claim_key.starts_with("stellar_orbital_hierarchy_attempt_")
            }
            Self::QuadrupleTopology(_) => claim_key == "quadruple_topology",
            Self::RelativeStellarOrbit(_) => claim_key == "relative_stellar_orbit",
            Self::RelativeStellarOrbitScale(_) => claim_key == "relative_stellar_orbit_scale",
            Self::RelativeStellarOrbitEccentricity(_) => {
                claim_key == "relative_stellar_orbit_eccentricity"
            }
            Self::CircumstellarStability(_) => claim_key == "circumstellar_s_type_stability_zone",
            Self::PlanetOccurrence(_) => claim_key.ends_with("occurrence"),
            Self::CloseBinaryOccurrenceFactor(_) => claim_key == "close_binary_occurrence_factor",
            Self::StellarOrbitalQualityFlag(_) => {
                claim_key.starts_with("stellar_orbital_hierarchy_quality_")
            }
            Self::CircumstellarStabilityQualityFlag(_) => {
                claim_key.starts_with("circumstellar_stability_quality_")
            }
            Self::PlanetOccurrenceQualityFlag(_) => {
                claim_key.starts_with("planet_occurrence_quality_")
            }
            Self::ExplicitPlanetQualityFlag(_) => claim_key.starts_with("explicit_planet_quality_"),
            Self::MDwarfOccurrenceCellSelection { .. } => {
                claim_key == "m_dwarf_occurrence_cell_index"
            }
            Self::ExplicitPlanetSourceChannel(_) => claim_key == "occurrence_source_channel",
            Self::PlanetTransitRadiusRearth(_) => claim_key == "planet_transit_radius_rearth",
            Self::PlanetDopplerMinimumMassMjup(_) => {
                claim_key == "planet_doppler_minimum_mass_mjup"
            }
            Self::PlanetOrbitalPeriodDays(_) => claim_key == "planet_orbital_period_days",
            Self::PlanetSemimajorAxisAu(_) => claim_key == "planet_semimajor_axis_au",
            Self::ExplicitPlanet(_) => claim_key == "explicit_planet_candidate",
            _ => true,
        };
        if !key_matches_value {
            return Err(invalid("claim key does not match its domain payload"));
        }
        let owner_system_id = claim_owner_system_id(object_id);
        let node_resolves = |node: OrbitalNodeClaim| match (owner_system_id, node.kind) {
            (Some(system_id), OrbitalNodeClaimKind::StellarMember) => {
                known_objects.iter().any(|id| {
                    id.as_str()
                        == format!(
                            "indexed-u64-le:{system_id:016x}/stellar-member:{:016x}",
                            node.stable_id
                        )
                })
            }
            (Some(system_id), OrbitalNodeClaimKind::Barycentre) => known_objects.iter().any(|id| {
                let value = id.as_str();
                value.contains(&format!("stellar-system-owner:{system_id:016x}"))
                    && value.ends_with(&format!("barycentre:{:016x}", node.stable_id))
            }),
            _ => false,
        };

        match self {
            Self::Population(_)
            | Self::MemberRole(_)
            | Self::EvolutionaryState(_)
            | Self::EvolutionQualityFlag(_)
            | Self::QuadrupleTopology(_)
            | Self::StellarOrbitalQualityFlag(_)
            | Self::CircumstellarStabilityQualityFlag(_)
            | Self::PlanetOccurrenceQualityFlag(_)
            | Self::ExplicitPlanetQualityFlag(_) => {}
            Self::StellarChemistry(chemistry) => {
                let values = [
                    chemistry.iron_abundance_feh,
                    chemistry.alpha_enhancement_alpha_fe,
                    chemistry.global_metallicity_mh,
                    chemistry.metal_mass_fraction_z,
                    chemistry.helium_mass_fraction_y,
                ];
                if !values.into_iter().all(finite)
                    || !(0.0..1.0).contains(&chemistry.metal_mass_fraction_z)
                    || !(0.0..1.0).contains(&chemistry.helium_mass_fraction_y)
                    || chemistry.metal_mass_fraction_z + chemistry.helium_mass_fraction_y >= 1.0
                {
                    return Err(invalid("Stellar Chemistry contains invalid mass fractions"));
                }
            }
            Self::IronAbundanceFeH(value)
            | Self::AlphaEnhancementAlphaFe(value)
            | Self::SourceMetallicityCoordinateMh(value)
            | Self::SurfaceGravityLog10Cgs(value) => {
                if !finite(*value) {
                    return Err(invalid("numeric payload must be finite"));
                }
            }
            Self::StellarAgeGyr(value)
            | Self::ZeroAgeMainSequenceAgeGyr(value)
            | Self::TerminalAgeMainSequenceAgeGyr(value)
            | Self::MainSequenceLifetimeGyr(value)
            | Self::WhiteDwarfHandoffAgeGyr(value)
            | Self::WhiteDwarfCoolingAgeGyr(value) => {
                if !non_negative(*value) {
                    return Err(invalid("age payload must be finite and non-negative"));
                }
            }
            Self::InitialStellarMassMsolar(value)
            | Self::CurrentStellarMassMsolar(value)
            | Self::RemnantMassMsolar(value)
            | Self::LuminosityLsolar(value)
            | Self::RadiusRsolar(value)
            | Self::EffectiveTemperatureK(value)
            | Self::PlanetTransitRadiusRearth(value)
            | Self::PlanetDopplerMinimumMassMjup(value)
            | Self::PlanetOrbitalPeriodDays(value)
            | Self::PlanetSemimajorAxisAu(value) => {
                if !positive(*value) {
                    return Err(invalid("physical scalar must be finite and positive"));
                }
                if matches!(
                    self,
                    Self::PlanetTransitRadiusRearth(_)
                        | Self::PlanetDopplerMinimumMassMjup(_)
                        | Self::PlanetOrbitalPeriodDays(_)
                ) && !planet_property_source_matches(claim, realized_claims)
                {
                    return Err(invalid("explicit planet property input is inconsistent"));
                }
            }
            Self::CompanionMassRatio(value)
            | Self::FractionalMainSequenceAge(value)
            | Self::CloseBinaryOccurrenceFactor(value) => {
                if !finite(*value) || !(0.0..=1.0).contains(value) {
                    return Err(invalid("fractional payload must lie between zero and one"));
                }
            }
            Self::StellarMemberCount(count) => {
                if *count == 0 || *count > 4 {
                    return Err(invalid("stellar member count must be in 1..=4"));
                }
            }
            Self::OrbitalHierarchySamplingAttempt(attempt) => {
                if !valid_hierarchy_attempt_claim(claim, attempt, realized_claims) {
                    return Err(invalid(
                        "bounded hierarchy attempt diagnostic is inconsistent",
                    ));
                }
            }
            Self::OrbitalHierarchySamplingExhaustion {
                attempted_candidates,
            } => {
                if *attempted_candidates == 0 {
                    return Err(invalid(
                        "sampling exhaustion must retain a positive attempt count",
                    ));
                }
            }
            Self::StellarOrbitalHierarchy(value) => {
                validate_hierarchy_claim(claim, value, realized_claims, known_objects)
                    .map_err(|detail| invalid(&detail))?;
            }
            Self::RelativeStellarOrbit(value) => {
                if value.barycentre.kind != OrbitalNodeClaimKind::Barycentre
                    || value.left_child == value.right_child
                    || value.left_child == value.barycentre
                    || value.right_child == value.barycentre
                    || !node_resolves(value.barycentre)
                    || !node_resolves(value.left_child)
                    || !node_resolves(value.right_child)
                    || !valid_relative_orbit(value.orbit)
                    || !relative_orbit_inputs_match(claim, value, realized_claims)
                {
                    return Err(invalid(
                        "orbital relationship contains invalid or unresolved nodes",
                    ));
                }
            }
            Self::RelativeStellarOrbitScale(value) => {
                if !positive(value.semimajor_axis_au)
                    || !positive(value.period_days)
                    || !positive(value.combined_mass_msun)
                {
                    return Err(invalid("relative-orbit scale payload is invalid"));
                }
            }
            Self::RelativeStellarOrbitEccentricity(value) => {
                if !finite(*value) || !(0.0..1.0).contains(value) {
                    return Err(invalid("eccentricity must lie in [0, 1)"));
                }
            }
            Self::CircumstellarStability(
                CircumstellarStabilityClaim::UnboundedByStellarCompanion,
            ) => {}
            Self::CircumstellarStability(CircumstellarStabilityClaim::CompanionLimited {
                model,
                nominal_outer_critical_semimajor_axis_au,
                fit_residual_lower_semimajor_axis_au,
                limiting_companion_mass_msun,
                companion_mass_fraction,
                limiting_barycentre_id,
            }) => {
                let limiting_node = OrbitalNodeClaim {
                    stable_id: *limiting_barycentre_id,
                    kind: OrbitalNodeClaimKind::Barycentre,
                };
                if !positive(*nominal_outer_critical_semimajor_axis_au)
                    || !non_negative(*fit_residual_lower_semimajor_axis_au)
                    || fit_residual_lower_semimajor_axis_au
                        > nominal_outer_critical_semimajor_axis_au
                    || !positive(*limiting_companion_mass_msun)
                    || !finite(*companion_mass_fraction)
                    || !(0.0..1.0).contains(companion_mass_fraction)
                    || !node_resolves(limiting_node)
                    || !stability_inputs_match(
                        claim,
                        StabilityInputValues {
                            model: *model,
                            nominal: *nominal_outer_critical_semimajor_axis_au,
                            lower: *fit_residual_lower_semimajor_axis_au,
                            companion_mass: *limiting_companion_mass_msun,
                            mass_fraction: *companion_mass_fraction,
                            barycentre_id: *limiting_barycentre_id,
                        },
                        realized_claims,
                    )
                {
                    return Err(invalid("companion-limited stability payload is invalid"));
                }
            }
            Self::PlanetOccurrence(PlanetOccurrenceClaim::PlanetCount { count: 0, .. }) => {
                return Err(invalid(
                    "zero occurrence counts must be represented by NotSelected",
                ));
            }
            Self::PlanetOccurrence(PlanetOccurrenceClaim::PlanetCount {
                multiplicity_suppression_extrapolated,
                ..
            })
            | Self::PlanetOccurrence(PlanetOccurrenceClaim::HasAtLeastOneGiant {
                multiplicity_suppression_extrapolated,
            }) => {
                if *multiplicity_suppression_extrapolated
                    != derivation_has_suffix(claim, "close_binary_occurrence_factor")
                {
                    return Err(invalid(
                        "multiplicity suppression metadata disagrees with claim derivation",
                    ));
                }
            }
            Self::MDwarfOccurrenceCellSelection { index, cell_count } => {
                if *cell_count == 0
                    || *index >= *cell_count
                    || !planet_property_source_matches(claim, realized_claims)
                {
                    return Err(invalid("M-dwarf occurrence cell selection is out of range"));
                }
            }
            Self::ExplicitPlanetSourceChannel(channel) => {
                if !explicit_source_input_matches(claim, *channel, realized_claims) {
                    return Err(invalid(
                        "explicit planet source channel input is inconsistent",
                    ));
                }
            }
            Self::ExplicitPlanet(value) => {
                let Some(system_id) = owner_system_id else {
                    return Err(invalid("explicit planet lacks System Ownership identity"));
                };
                let host_object = format!(
                    "indexed-u64-le:{system_id:016x}/stellar-member:{:016x}",
                    value.host_member_id
                );
                let expected_host_path =
                    format!("stellar-member-host:{:016x}", value.host_member_id);
                let has_radius = value.transit_radius_rearth.is_some();
                let has_mass = value.doppler_minimum_mass_mjup.is_some();
                let properties_match_channel = match value.source_channel {
                    ExplicitPlanetSourceChannel::FgkDopplerGiant => has_mass && !has_radius,
                    _ => has_radius && !has_mass,
                };
                if value.host_member_id != value.orbital_parent_member_id
                    || !known_objects.iter().any(|id| id.as_str() == host_object)
                    || !object_id.contains(&expected_host_path)
                    || !properties_match_channel
                    || !explicit_planet_inputs_match(claim, value, realized_claims)
                    || value
                        .transit_radius_rearth
                        .is_some_and(|value| !positive(value))
                    || value
                        .doppler_minimum_mass_mjup
                        .is_some_and(|value| !positive(value))
                    || !positive(value.period_days)
                    || !positive(value.semimajor_axis_au)
                    || matches!(
                        value.source_channel,
                        ExplicitPlanetSourceChannel::MDwarfSmallPlanet
                            | ExplicitPlanetSourceChannel::MDwarfSubEarth
                    ) != value.source_cell_index.is_some()
                    || value.source_cell_index.is_some() != value.source_cell_count.is_some()
                    || value
                        .source_cell_index
                        .zip(value.source_cell_count)
                        .is_some_and(|(index, count)| count == 0 || index >= count)
                {
                    return Err(invalid(
                        "explicit planet ownership, host, parent, or properties are inconsistent",
                    ));
                }
            }
        }
        Ok(())
    }
}

type StellarClaimMap<'a> =
    std::collections::BTreeMap<ClaimId, &'a ScientificClaim<StellarClaimValue>>;

fn claim_inputs<'a>(
    claim: &ScientificClaim<StellarClaimValue>,
    realized_claims: &'a StellarClaimMap<'_>,
) -> Option<Vec<&'a ScientificClaim<StellarClaimValue>>> {
    claim
        .provenance
        .derivation
        .as_ref()?
        .input_claims
        .iter()
        .map(|id| realized_claims.get(id).copied())
        .collect()
}

fn valid_hierarchy_attempt_claim(
    claim: &ScientificClaim<StellarClaimValue>,
    attempt: &StellarOrbitalHierarchyAttemptClaim,
    realized_claims: &StellarClaimMap<'_>,
) -> bool {
    let address_matches = claim
        .provenance
        .random_draw_address
        .as_ref()
        .is_some_and(|address| {
            address.bounded_attempt_index == 0
                && address.stable_object_id == claim.provenance.object_id
        });
    let constraints_valid = !attempt.constraints.is_empty()
        && attempt
            .constraints
            .iter()
            .any(|constraint| !constraint.passed())
        && attempt
            .constraints
            .iter()
            .all(|constraint| match constraint {
                StellarOrbitalHierarchyAttemptConstraint::OrbitalScaleCalibration {
                    sampling_slot,
                    ..
                } => *sampling_slot < 3,
                StellarOrbitalHierarchyAttemptConstraint::StellarContact {
                    sampling_slot,
                    periastron_au,
                    minimum_separation_au,
                    passed,
                } => {
                    *sampling_slot < 3
                        && periastron_au.is_finite()
                        && minimum_separation_au.is_finite()
                        && *passed == (*periastron_au > *minimum_separation_au)
                }
                StellarOrbitalHierarchyAttemptConstraint::HierarchicalStability {
                    outer_sampling_slot,
                    semimajor_axis_ratio,
                    critical_ratio,
                    passed,
                } => {
                    *outer_sampling_slot < 3
                        && semimajor_axis_ratio.is_finite()
                        && critical_ratio.is_finite()
                        && *passed == (*semimajor_axis_ratio > *critical_ratio)
                }
            });
    if !address_matches || !constraints_valid {
        return false;
    }
    let Some(root) = attempt.candidate_root else {
        return attempt.candidate_relationships.is_empty()
            && attempt.constraints.iter().all(|constraint| {
                matches!(
                    constraint,
                    StellarOrbitalHierarchyAttemptConstraint::OrbitalScaleCalibration { .. }
                )
            });
    };
    if attempt.candidate_relationships.is_empty()
        || attempt.candidate_relationships.iter().any(|relationship| {
            relationship.sampling_attempt != attempt.attempt
                || !valid_relative_orbit(relationship.orbit)
        })
    {
        return false;
    }
    let Some(system_id) = claim_owner_system_id(claim.provenance.object_id.as_str()) else {
        return false;
    };
    let Some(member_masses) = claim_member_masses(claim, realized_claims, system_id) else {
        return false;
    };
    validate_orbital_tree(
        system_id,
        root,
        &attempt.candidate_relationships,
        &member_masses,
    )
    .is_ok()
}

fn claim_member_masses(
    claim: &ScientificClaim<StellarClaimValue>,
    realized_claims: &StellarClaimMap<'_>,
    system_id: u64,
) -> Option<std::collections::BTreeMap<u64, f64>> {
    let marker = format!("indexed-u64-le:{system_id:016x}/stellar-member:");
    let mut masses = std::collections::BTreeMap::new();
    for input in claim_inputs(claim, realized_claims)? {
        let mass = match input.value {
            StellarClaimValue::InitialStellarMassMsolar(mass)
            | StellarClaimValue::CurrentStellarMassMsolar(mass) => mass,
            _ => continue,
        };
        let member_id = input
            .provenance
            .object_id
            .as_str()
            .strip_prefix(&marker)
            .and_then(|id| u64::from_str_radix(id, 16).ok())?;
        if masses.insert(member_id, mass).is_some() {
            return None;
        }
    }
    (!masses.is_empty()).then_some(masses)
}

fn validate_orbital_tree(
    system_id: u64,
    root: OrbitalNodeClaim,
    relationships: &[RelativeStellarOrbitClaim],
    member_masses: &std::collections::BTreeMap<u64, f64>,
) -> Result<(), String> {
    let mut by_barycentre = std::collections::BTreeMap::new();
    let mut slots = std::collections::BTreeSet::new();
    let sampling_attempt = relationships
        .first()
        .map(|relationship| relationship.sampling_attempt);
    for relationship in relationships {
        if relationship.barycentre.kind != OrbitalNodeClaimKind::Barycentre
            || by_barycentre
                .insert(relationship.barycentre, relationship)
                .is_some()
            || usize::from(relationship.sampling_slot) >= relationships.len()
            || !slots.insert(relationship.sampling_slot)
            || Some(relationship.sampling_attempt) != sampling_attempt
            || !valid_relative_orbit(relationship.orbit)
        {
            return Err("orbital nodes, slots, or physical values are inconsistent".into());
        }
    }
    if slots.len() != relationships.len() {
        return Err("orbital sampling slots are not unique and complete".into());
    }

    fn visit(
        system_id: u64,
        node: OrbitalNodeClaim,
        relationships: &std::collections::BTreeMap<OrbitalNodeClaim, &RelativeStellarOrbitClaim>,
        member_masses: &std::collections::BTreeMap<u64, f64>,
        visited_barycentres: &mut std::collections::BTreeSet<OrbitalNodeClaim>,
    ) -> Result<(std::collections::BTreeSet<u64>, f64), String> {
        match node.kind {
            OrbitalNodeClaimKind::StellarMember => {
                let mass = member_masses.get(&node.stable_id).copied().ok_or_else(|| {
                    "orbital member does not belong to the owning system".to_owned()
                })?;
                Ok((std::collections::BTreeSet::from([node.stable_id]), mass))
            }
            OrbitalNodeClaimKind::Barycentre => {
                if !visited_barycentres.insert(node) {
                    return Err("orbital hierarchy contains a cycle or repeated barycentre".into());
                }
                let relationship = relationships
                    .get(&node)
                    .ok_or_else(|| "orbital barycentre relationship is missing".to_owned())?;
                let (left_members, left_mass) = visit(
                    system_id,
                    relationship.left_child,
                    relationships,
                    member_masses,
                    visited_barycentres,
                )?;
                let (right_members, right_mass) = visit(
                    system_id,
                    relationship.right_child,
                    relationships,
                    member_masses,
                    visited_barycentres,
                )?;
                if !left_members.is_disjoint(&right_members) {
                    return Err("orbital child subtrees overlap".into());
                }
                let members = left_members
                    .union(&right_members)
                    .copied()
                    .collect::<std::collections::BTreeSet<_>>();
                let ordered_members = members.iter().copied().collect::<Vec<_>>();
                if node.stable_id != stable_barycentre_id(system_id, &ordered_members) {
                    return Err("orbital barycentre identity does not match its descendants".into());
                }
                let combined_mass = left_mass + right_mass;
                if relationship.orbit.combined_mass_msun != combined_mass
                    || relationship.orbit.period_days
                        != period_days_from_semimajor_axis(
                            relationship.orbit.semimajor_axis_au,
                            combined_mass,
                        )
                {
                    return Err("relative orbit mass or Kepler period is inconsistent".into());
                }
                Ok((members, combined_mass))
            }
        }
    }

    if relationships.is_empty() {
        return (root.kind == OrbitalNodeClaimKind::StellarMember
            && member_masses.len() == 1
            && member_masses.contains_key(&root.stable_id))
        .then_some(())
        .ok_or_else(|| "single-member orbital root is inconsistent".into());
    }
    let mut visited = std::collections::BTreeSet::new();
    let (members, _) = visit(system_id, root, &by_barycentre, member_masses, &mut visited)?;
    if visited.len() != relationships.len()
        || members.len() != member_masses.len()
        || !members
            .iter()
            .all(|member| member_masses.contains_key(member))
    {
        return Err("orbital hierarchy is disconnected or omits owning members".into());
    }
    Ok(())
}

fn validate_hierarchy_claim(
    claim: &ScientificClaim<StellarClaimValue>,
    value: &StellarOrbitalHierarchyClaim,
    realized_claims: &StellarClaimMap<'_>,
    known_objects: &std::collections::BTreeSet<ObjectId>,
) -> Result<(), String> {
    let system_id = claim_owner_system_id(claim.provenance.object_id.as_str())
        .ok_or_else(|| "hierarchy lacks a system identity".to_owned())?;
    let inputs = claim_inputs(claim, realized_claims)
        .ok_or_else(|| "hierarchy lacks realized derivation inputs".to_owned())?;
    let relationships = inputs
        .iter()
        .filter_map(|input| match &input.value {
            StellarClaimValue::RelativeStellarOrbit(relationship) => Some(*relationship),
            _ => None,
        })
        .collect::<Vec<_>>();
    let topology_inputs = inputs
        .iter()
        .filter_map(|input| match input.value {
            StellarClaimValue::QuadrupleTopology(topology) => Some(topology),
            _ => None,
        })
        .collect::<Vec<_>>();
    let mass_inputs = inputs
        .iter()
        .filter(|input| {
            matches!(
                input.value,
                StellarClaimValue::InitialStellarMassMsolar(_)
                    | StellarClaimValue::CurrentStellarMassMsolar(_)
            )
        })
        .copied()
        .collect::<Vec<_>>();
    let mass_input_count = mass_inputs.len();
    if inputs.len() != relationships.len() + topology_inputs.len() + mass_input_count
        || relationships.len() != usize::from(value.relative_orbit_count)
        || topology_inputs.as_slice() != value.quadruple_topology.as_slice()
    {
        return Err("hierarchy derivation inputs, count, or topology disagree".into());
    }

    let member_marker = format!("indexed-u64-le:{system_id:016x}/stellar-member:");
    let expected_members = known_objects
        .iter()
        .filter_map(|object| {
            object
                .as_str()
                .strip_prefix(&member_marker)
                .and_then(|id| u64::from_str_radix(id, 16).ok())
        })
        .collect::<std::collections::BTreeSet<_>>();
    let mass_input_members = mass_inputs
        .iter()
        .filter_map(|input| {
            input
                .provenance
                .object_id
                .as_str()
                .strip_prefix(&member_marker)
                .and_then(|id| u64::from_str_radix(id, 16).ok())
        })
        .collect::<std::collections::BTreeSet<_>>();
    if expected_members.is_empty()
        || mass_input_count != expected_members.len()
        || mass_input_members != expected_members
    {
        return Err("hierarchy mass inputs do not cover the system members exactly".into());
    }
    let member_masses = claim_member_masses(claim, realized_claims, system_id)
        .ok_or_else(|| "hierarchy mass inputs are ambiguous".to_owned())?;
    validate_orbital_tree(system_id, value.root, &relationships, &member_masses)?;

    let mut barycentres = std::collections::BTreeMap::new();
    let mut parent_counts = std::collections::BTreeMap::<OrbitalNodeClaim, usize>::new();
    let mut used_members = std::collections::BTreeSet::new();
    for relationship in &relationships {
        if relationship.barycentre.kind != OrbitalNodeClaimKind::Barycentre
            || barycentres
                .insert(relationship.barycentre, *relationship)
                .is_some()
        {
            return Err("hierarchy barycentres are not unique".into());
        }
        for child in [relationship.left_child, relationship.right_child] {
            *parent_counts.entry(child).or_default() += 1;
            if child.kind == OrbitalNodeClaimKind::StellarMember
                && !used_members.insert(child.stable_id)
            {
                return Err("a stellar member occurs more than once in the hierarchy".into());
            }
        }
    }
    if value.relative_orbit_count == 0 {
        return (value.root.kind == OrbitalNodeClaimKind::StellarMember
            && value.quadruple_topology.is_none()
            && expected_members == std::collections::BTreeSet::from([value.root.stable_id]))
        .then_some(())
        .ok_or_else(|| "single-member hierarchy root is inconsistent".into());
    }
    if value.root.kind != OrbitalNodeClaimKind::Barycentre
        || !barycentres.contains_key(&value.root)
        || parent_counts.contains_key(&value.root)
        || used_members != expected_members
        || barycentres
            .keys()
            .any(|node| *node != value.root && parent_counts.get(node).copied() != Some(1))
        || parent_counts.values().any(|count| *count != 1)
    {
        return Err("hierarchy is not one rooted tree covering every member once".into());
    }
    let mut pending = vec![value.root];
    let mut visited = std::collections::BTreeSet::new();
    while let Some(node) = pending.pop() {
        if !visited.insert(node) {
            return Err("hierarchy contains a cycle".into());
        }
        if let Some(relationship) = barycentres.get(&node) {
            pending.extend([relationship.left_child, relationship.right_child]);
        }
    }
    if visited.len() != relationships.len() + expected_members.len() {
        return Err("hierarchy contains disconnected nodes".into());
    }
    let root = barycentres[&value.root];
    let topology_matches = match value.quadruple_topology {
        None => value.relative_orbit_count < 3,
        Some(QuadrupleTopology::TwoPlusTwo) => {
            value.relative_orbit_count == 3
                && root.left_child.kind == OrbitalNodeClaimKind::Barycentre
                && root.right_child.kind == OrbitalNodeClaimKind::Barycentre
        }
        Some(QuadrupleTopology::ThreePlusOne) => {
            value.relative_orbit_count == 3
                && (root.left_child.kind == OrbitalNodeClaimKind::Barycentre)
                    != (root.right_child.kind == OrbitalNodeClaimKind::Barycentre)
        }
    };
    topology_matches
        .then_some(())
        .ok_or_else(|| "quadruple topology disagrees with the realized tree".into())
}

fn relative_orbit_inputs_match(
    claim: &ScientificClaim<StellarClaimValue>,
    relationship: &RelativeStellarOrbitClaim,
    realized_claims: &StellarClaimMap<'_>,
) -> bool {
    let Some(inputs) = claim_inputs(claim, realized_claims) else {
        return false;
    };
    let scales = inputs
        .iter()
        .filter_map(|input| match input.value {
            StellarClaimValue::RelativeStellarOrbitScale(value) => Some((input, value)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let eccentricities = inputs
        .iter()
        .filter_map(|input| match input.value {
            StellarClaimValue::RelativeStellarOrbitEccentricity(value) => Some((input, value)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let (scale_claim, scale) = match scales.as_slice() {
        [(claim, value)] => (*claim, *value),
        _ => return false,
    };
    let (eccentricity_claim, eccentricity) = match eccentricities.as_slice() {
        [(claim, value)] => (*claim, *value),
        _ => return false,
    };
    if scale_claim.id.as_str()
        != format!(
            "{}/relative_stellar_orbit_scale",
            claim.provenance.object_id
        )
        || eccentricity_claim.id.as_str()
            != format!(
                "{}/relative_stellar_orbit_eccentricity",
                claim.provenance.object_id
            )
    {
        return false;
    }
    let expected_draw_id =
        claim_owner_system_id(claim.provenance.object_id.as_str()).map(|system_id| {
            stable_orbit_draw_id(
                system_id,
                relationship.sampling_attempt,
                relationship.sampling_slot,
            )
        });
    let address_matches = |input: &ScientificClaim<StellarClaimValue>| {
        input
            .provenance
            .random_draw_address
            .as_ref()
            .is_some_and(|address| {
                address.bounded_attempt_index == 0
                    && expected_draw_id.is_some_and(|id| {
                        address
                            .stable_object_id
                            .as_str()
                            .starts_with(&format!("indexed-u64-le:{id:016x}/"))
                    })
            })
    };
    scale.semimajor_axis_au == relationship.orbit.semimajor_axis_au
        && scale.period_days == relationship.orbit.period_days
        && scale.combined_mass_msun == relationship.orbit.combined_mass_msun
        && scale.sampling_attempt == relationship.sampling_attempt
        && scale.sampling_slot == relationship.sampling_slot
        && eccentricity == relationship.orbit.eccentricity
        && address_matches(scale_claim)
        && (relationship.orbit.eccentricity == 0.0 || address_matches(eccentricity_claim))
}

#[derive(Clone, Copy)]
struct StabilityInputValues {
    model: HolmanWiegertSTypeModel,
    nominal: f64,
    lower: f64,
    companion_mass: f64,
    mass_fraction: f64,
    barycentre_id: u64,
}

fn stability_inputs_match(
    claim: &ScientificClaim<StellarClaimValue>,
    values: StabilityInputValues,
    realized_claims: &StellarClaimMap<'_>,
) -> bool {
    let StabilityInputValues {
        model,
        nominal,
        lower,
        companion_mass,
        mass_fraction,
        barycentre_id,
    } = values;
    let Some(inputs) = claim_inputs(claim, realized_claims) else {
        return false;
    };
    let relationships = inputs
        .iter()
        .filter_map(|input| match input.value {
            StellarClaimValue::RelativeStellarOrbit(value) => Some(value),
            _ => None,
        })
        .collect::<Vec<_>>();
    let [relationship] = relationships.as_slice() else {
        return false;
    };
    let Some(host_id) = claim
        .provenance
        .object_id
        .as_str()
        .split("stellar-member:")
        .nth(1)
        .and_then(|id| u64::from_str_radix(id, 16).ok())
    else {
        return false;
    };
    if relationship.barycentre.stable_id != barycentre_id
        || ![relationship.left_child, relationship.right_child].contains(&OrbitalNodeClaim {
            stable_id: host_id,
            kind: OrbitalNodeClaimKind::StellarMember,
        })
    {
        return false;
    }
    let host_mass = inputs.iter().find_map(|input| {
        (input
            .provenance
            .object_id
            .as_str()
            .ends_with(&format!("stellar-member:{host_id:016x}")))
        .then_some(match input.value {
            StellarClaimValue::CurrentStellarMassMsolar(value)
            | StellarClaimValue::InitialStellarMassMsolar(value) => Some(value),
            _ => None,
        })
        .flatten()
    });
    let Some(host_mass) = host_mass else {
        return false;
    };
    let derived_companion = relationship.orbit.combined_mass_msun - host_mass;
    let derived_fraction = derived_companion / relationship.orbit.combined_mass_msun;
    let e = relationship.orbit.eccentricity;
    let critical_fraction = model.constant
        + model.mass_ratio_coefficient * derived_fraction
        + model.eccentricity_coefficient * e
        + model.mass_ratio_eccentricity_coefficient * derived_fraction * e
        + model.eccentricity_squared_coefficient * e * e
        + model.mass_ratio_eccentricity_squared_coefficient * derived_fraction * e * e;
    floats_match(companion_mass, derived_companion)
        && floats_match(mass_fraction, derived_fraction)
        && floats_match(
            nominal,
            critical_fraction * relationship.orbit.semimajor_axis_au,
        )
        && floats_match(lower, nominal * model.fit_residual_lower_factor)
}

fn planet_property_source_matches(
    claim: &ScientificClaim<StellarClaimValue>,
    realized_claims: &StellarClaimMap<'_>,
) -> bool {
    let expected = ClaimId::from(format!(
        "{}/occurrence_source_channel",
        claim.provenance.object_id
    ));
    claim
        .provenance
        .derivation
        .as_ref()
        .is_some_and(|derivation| {
            derivation.input_claims.as_slice() == [expected.clone()]
                && realized_claims.get(&expected).is_some_and(|source| {
                    matches!(
                        source.value,
                        StellarClaimValue::ExplicitPlanetSourceChannel(_)
                    )
                })
        })
}

fn explicit_source_input_matches(
    claim: &ScientificClaim<StellarClaimValue>,
    channel: ExplicitPlanetSourceChannel,
    realized_claims: &StellarClaimMap<'_>,
) -> bool {
    let Some(inputs) = claim_inputs(claim, realized_claims) else {
        return false;
    };
    let [occurrence] = inputs.as_slice() else {
        return false;
    };
    let expected_key = match channel {
        ExplicitPlanetSourceChannel::FgkWarmSuperEarth => "fgk_warm_super_earth_occurrence",
        ExplicitPlanetSourceChannel::FgkWarmSubNeptune => "fgk_warm_sub_neptune_occurrence",
        ExplicitPlanetSourceChannel::MDwarfSmallPlanet => "m_dwarf_small_planet_occurrence",
        ExplicitPlanetSourceChannel::MDwarfSubEarth => "m_dwarf_sub_earth_occurrence",
        ExplicitPlanetSourceChannel::FgkDopplerGiant => "giant_planet_occurrence",
    };
    let object = claim.provenance.object_id.as_str();
    let owner = object
        .split("stellar-system-owner:")
        .nth(1)
        .and_then(|rest| rest.split('/').next());
    let host = object
        .split("stellar-member-host:")
        .nth(1)
        .and_then(|rest| rest.split('/').next());
    occurrence.provenance.claim_key == expected_key
        && owner.is_some_and(|owner| {
            occurrence
                .provenance
                .object_id
                .as_str()
                .contains(&format!("stellar-system-owner:{owner}"))
        })
        && host.is_some_and(|host| {
            occurrence
                .provenance
                .object_id
                .as_str()
                .contains(&format!("stellar-member-host:{host}"))
        })
        && matches!(occurrence.value, StellarClaimValue::PlanetOccurrence(_))
}

fn explicit_planet_inputs_match(
    claim: &ScientificClaim<StellarClaimValue>,
    value: &ExplicitPlanetClaim,
    realized_claims: &StellarClaimMap<'_>,
) -> bool {
    let object = claim.provenance.object_id.as_str();
    let property_key = if value.transit_radius_rearth.is_some() {
        "planet_transit_radius_rearth"
    } else {
        "planet_doppler_minimum_mass_mjup"
    };
    let mut expected_ids = vec![
        ClaimId::from(format!("{object}/occurrence_source_channel")),
        ClaimId::from(format!("{object}/{property_key}")),
        ClaimId::from(format!("{object}/planet_orbital_period_days")),
        ClaimId::from(format!("{object}/planet_semimajor_axis_au")),
    ];
    if value.source_cell_index.is_some() {
        expected_ids.push(ClaimId::from(format!(
            "{object}/m_dwarf_occurrence_cell_index"
        )));
    }
    let Some(derivation) = &claim.provenance.derivation else {
        return false;
    };
    if derivation.input_claims.len() != expected_ids.len()
        || !expected_ids
            .iter()
            .all(|expected| derivation.input_claims.contains(expected))
    {
        return false;
    }
    let input = |key: &str| {
        realized_claims
            .get(&ClaimId::from(format!("{object}/{key}")))
            .copied()
    };
    let source_matches = input("occurrence_source_channel").is_some_and(|input| {
        input.provenance.object_id == claim.provenance.object_id
            && input.value == StellarClaimValue::ExplicitPlanetSourceChannel(value.source_channel)
    });
    let property_matches = input(property_key).is_some_and(|input| {
        input.provenance.object_id == claim.provenance.object_id
            && match input.value {
                StellarClaimValue::PlanetTransitRadiusRearth(radius) => {
                    Some(radius) == value.transit_radius_rearth
                }
                StellarClaimValue::PlanetDopplerMinimumMassMjup(mass) => {
                    Some(mass) == value.doppler_minimum_mass_mjup
                }
                _ => false,
            }
    });
    let period_matches = input("planet_orbital_period_days").is_some_and(|input| {
        input.value == StellarClaimValue::PlanetOrbitalPeriodDays(value.period_days)
    });
    let axis_matches = input("planet_semimajor_axis_au").is_some_and(|input| {
        input.value == StellarClaimValue::PlanetSemimajorAxisAu(value.semimajor_axis_au)
            && explicit_axis_inputs_match(input, value, realized_claims)
    });
    let cell_matches = match (value.source_cell_index, value.source_cell_count) {
        (Some(index), Some(cell_count)) => {
            input("m_dwarf_occurrence_cell_index").is_some_and(|input| {
                input.value
                    == StellarClaimValue::MDwarfOccurrenceCellSelection { index, cell_count }
            })
        }
        (None, None) => true,
        _ => false,
    };
    source_matches && property_matches && period_matches && axis_matches && cell_matches
}

fn explicit_axis_inputs_match(
    axis_claim: &ScientificClaim<StellarClaimValue>,
    value: &ExplicitPlanetClaim,
    realized_claims: &StellarClaimMap<'_>,
) -> bool {
    let Some(inputs) = claim_inputs(axis_claim, realized_claims) else {
        return false;
    };
    if inputs.len() != 2 {
        return false;
    }
    let period_id = ClaimId::from(format!(
        "{}/planet_orbital_period_days",
        axis_claim.provenance.object_id
    ));
    let Some(system_id) = claim_owner_system_id(axis_claim.provenance.object_id.as_str()) else {
        return false;
    };
    let mass_id = ClaimId::from(format!(
        "indexed-u64-le:{system_id:016x}/stellar-member:{:016x}/current_stellar_mass_msolar",
        value.host_member_id
    ));
    if !axis_claim
        .provenance
        .derivation
        .as_ref()
        .is_some_and(|derivation| {
            derivation.input_claims.len() == 2
                && derivation.input_claims.contains(&period_id)
                && derivation.input_claims.contains(&mass_id)
        })
    {
        return false;
    }
    let Some(mass) = realized_claims
        .get(&mass_id)
        .and_then(|claim| match claim.value {
            StellarClaimValue::CurrentStellarMassMsolar(mass) => Some(mass),
            _ => None,
        })
    else {
        return false;
    };
    let derived_axis = semimajor_axis_from_period_days(value.period_days, mass);
    floats_match(value.semimajor_axis_au, derived_axis)
}

fn derivation_has_suffix(claim: &ScientificClaim<StellarClaimValue>, suffix: &str) -> bool {
    claim
        .provenance
        .derivation
        .as_ref()
        .is_some_and(|derivation| {
            derivation
                .input_claims
                .iter()
                .any(|input| input.as_str().ends_with(suffix))
        })
}

fn floats_match(left: f64, right: f64) -> bool {
    left == right || (left - right).abs() <= 1e-12 * left.abs().max(right.abs()).max(1.0)
}

fn valid_relative_orbit(orbit: RelativeStellarOrbit) -> bool {
    orbit.semimajor_axis_au.is_finite()
        && orbit.semimajor_axis_au > 0.0
        && orbit.period_days.is_finite()
        && orbit.period_days > 0.0
        && orbit.eccentricity.is_finite()
        && (0.0..1.0).contains(&orbit.eccentricity)
        && orbit.combined_mass_msun.is_finite()
        && orbit.combined_mass_msun > 0.0
        && floats_match(
            orbit.period_days,
            period_days_from_semimajor_axis(orbit.semimajor_axis_au, orbit.combined_mass_msun),
        )
}

fn claim_owner_system_id(object_id: &str) -> Option<u64> {
    if let Some(owner) = object_id
        .split("stellar-system-owner:")
        .nth(1)
        .and_then(|suffix| suffix.split('/').next())
    {
        return u64::from_str_radix(owner, 16).ok();
    }
    object_id
        .strip_prefix("indexed-u64-le:")
        .and_then(|suffix| suffix.split('/').next())
        .and_then(|value| u64::from_str_radix(value, 16).ok())
}
