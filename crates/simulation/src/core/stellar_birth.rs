//! Initial-mass and multiplicity sampling for stellar systems.

use super::*;

pub(crate) const PRIMARY_MASS_PRESCRIPTION_NAMESPACE: &str = "stellar_birth/primary_mass/v1";
pub(crate) const INITIAL_STELLAR_MASS_CLAIM_KEY: &str = "initial_stellar_mass_msolar";

pub(crate) fn primary_member_object_id(system_id: u64, member_id: u64) -> ObjectId {
    ObjectId::from(format!(
        "indexed-u64-le:{system_id:016x}/stellar-member:{member_id:016x}"
    ))
}

/// Two-segment stellar initial mass function, dN/dm proportional to m^-alpha.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct KroupaInitialMassFunction {
    pub minimum_mass_msun: f64,
    pub break_mass_msun: f64,
    pub maximum_mass_msun: f64,
    pub low_mass_exponent: f64,
    pub high_mass_exponent: f64,
}

/// Multiplicity and companion-mass model for one primary-mass interval.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MassConditionedMultiplicity {
    pub minimum_primary_mass_msun: f64,
    pub maximum_primary_mass_msun: f64,
    pub single_system_fraction: f64,
    pub binary_system_fraction: f64,
    pub triple_system_fraction: f64,
    pub higher_order_system_fraction: f64,
    pub representative_higher_order_members: u8,
    pub minimum_mass_ratio: f64,
    /// Exponent gamma in p(q) proportional to q^gamma.
    pub mass_ratio_power: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StellarBirthMassModel {
    pub initial_mass_function: KroupaInitialMassFunction,
    pub minimum_companion_mass_msun: f64,
    pub multiplicity_bins: Vec<MassConditionedMultiplicity>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StellarMemberRole {
    Primary,
    Companion,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StellarBirthMember {
    pub id: u64,
    pub role: StellarMemberRole,
    pub initial_mass_msun: f64,
    pub mass_ratio_to_primary: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StellarBirthSystem {
    pub members: Vec<StellarBirthMember>,
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum StellarBirthMassError {
    #[error("Kroupa IMF bounds and exponents must be finite, positive, and ordered")]
    InvalidInitialMassFunction,
    #[error("minimum companion mass must be finite and inside the IMF mass range")]
    InvalidMinimumCompanionMass,
    #[error("multiplicity bins must be valid, ordered, contiguous, and cover the IMF range")]
    InvalidMultiplicityBins,
}

#[derive(Debug, Clone)]
pub struct StellarBirthMassSampler {
    model: StellarBirthMassModel,
    low_segment_weight: f64,
    total_imf_weight: f64,
}

impl StellarBirthMassSampler {
    pub fn new(model: StellarBirthMassModel) -> Result<Self, StellarBirthMassError> {
        let imf = model.initial_mass_function;
        if [
            imf.minimum_mass_msun,
            imf.break_mass_msun,
            imf.maximum_mass_msun,
            imf.low_mass_exponent,
            imf.high_mass_exponent,
        ]
        .into_iter()
        .any(|value| !value.is_finite() || value <= 0.0)
            || !(imf.minimum_mass_msun < imf.break_mass_msun
                && imf.break_mass_msun < imf.maximum_mass_msun)
        {
            return Err(StellarBirthMassError::InvalidInitialMassFunction);
        }
        if !model.minimum_companion_mass_msun.is_finite()
            || !(imf.minimum_mass_msun..=imf.maximum_mass_msun)
                .contains(&model.minimum_companion_mass_msun)
        {
            return Err(StellarBirthMassError::InvalidMinimumCompanionMass);
        }
        if !valid_multiplicity_bins(&model) {
            return Err(StellarBirthMassError::InvalidMultiplicityBins);
        }

        let low_segment_weight = power_law_integral(
            imf.minimum_mass_msun,
            imf.break_mass_msun,
            imf.low_mass_exponent,
        );
        let continuity = imf
            .break_mass_msun
            .powf(imf.high_mass_exponent - imf.low_mass_exponent);
        let high_segment_weight = continuity
            * power_law_integral(
                imf.break_mass_msun,
                imf.maximum_mass_msun,
                imf.high_mass_exponent,
            );
        Ok(Self {
            model,
            low_segment_weight,
            total_imf_weight: low_segment_weight + high_segment_weight,
        })
    }

    pub fn sample(&self, seed: u64, system_id: u64) -> StellarBirthSystem {
        let primary_mass_msun = self.sample_primary_mass(seed, system_id);
        let multiplicity = self.multiplicity_for(primary_mass_msun);
        let mut multiplicity_rng = domain_rng(
            seed,
            b"stellar_birth/system_multiplicity/v1",
            Some(system_id),
        );
        let draw = multiplicity_rng.gen_range(0.0..1.0);
        let binary_end = multiplicity.single_system_fraction + multiplicity.binary_system_fraction;
        let triple_end = binary_end + multiplicity.triple_system_fraction;
        let member_count = if draw < multiplicity.single_system_fraction {
            1
        } else if draw < binary_end {
            2
        } else if draw < triple_end {
            3
        } else {
            multiplicity.representative_higher_order_members
        };

        let mut companion_mass_ratios = (1..member_count)
            .map(|rank| {
                let minimum_q = multiplicity
                    .minimum_mass_ratio
                    .max(self.model.minimum_companion_mass_msun / primary_mass_msun);
                let mut rng = domain_rng(
                    seed,
                    b"stellar_birth/companion_mass_ratio/v1",
                    Some(stable_member_id(system_id, rank)),
                );
                sample_power_law(minimum_q, 1.0, -multiplicity.mass_ratio_power, &mut rng)
            })
            .collect::<Vec<_>>();
        companion_mass_ratios.sort_by(|left, right| right.total_cmp(left));

        let mut members = Vec::with_capacity(usize::from(member_count));
        members.push(StellarBirthMember {
            id: stable_member_id(system_id, 0),
            role: StellarMemberRole::Primary,
            initial_mass_msun: primary_mass_msun,
            mass_ratio_to_primary: None,
        });
        members.extend(
            companion_mass_ratios
                .into_iter()
                .enumerate()
                .map(|(index, mass_ratio)| StellarBirthMember {
                    id: stable_member_id(system_id, index as u8 + 1),
                    role: StellarMemberRole::Companion,
                    initial_mass_msun: primary_mass_msun * mass_ratio,
                    mass_ratio_to_primary: Some(mass_ratio),
                }),
        );
        StellarBirthSystem { members }
    }

    pub fn expected_members_per_system(&self) -> f64 {
        self.model
            .multiplicity_bins
            .iter()
            .map(|bin| {
                let probability = self.imf_weight_between(
                    bin.minimum_primary_mass_msun,
                    bin.maximum_primary_mass_msun,
                ) / self.total_imf_weight;
                let mean_members = bin.single_system_fraction
                    + 2.0 * bin.binary_system_fraction
                    + 3.0 * bin.triple_system_fraction
                    + f64::from(bin.representative_higher_order_members)
                        * bin.higher_order_system_fraction;
                probability * mean_members
            })
            .sum()
    }

    pub fn multiplicity_fraction_for_primary_mass(&self, primary_mass_msun: f64) -> Option<f64> {
        let imf = self.model.initial_mass_function;
        if !primary_mass_msun.is_finite()
            || !(imf.minimum_mass_msun..=imf.maximum_mass_msun).contains(&primary_mass_msun)
        {
            return None;
        }
        Some(
            1.0 - self
                .multiplicity_for(primary_mass_msun)
                .single_system_fraction,
        )
    }

    fn sample_primary_mass(&self, seed: u64, system_id: u64) -> f64 {
        let imf = self.model.initial_mass_function;
        let mut rng = domain_rng(seed, b"stellar_birth/primary_mass/v1", Some(system_id));
        let draw = rng.gen_range(0.0..self.total_imf_weight);
        if draw < self.low_segment_weight {
            sample_power_law(
                imf.minimum_mass_msun,
                imf.break_mass_msun,
                imf.low_mass_exponent,
                &mut rng,
            )
        } else {
            sample_power_law(
                imf.break_mass_msun,
                imf.maximum_mass_msun,
                imf.high_mass_exponent,
                &mut rng,
            )
        }
    }

    fn multiplicity_for(&self, primary_mass_msun: f64) -> MassConditionedMultiplicity {
        *self
            .model
            .multiplicity_bins
            .iter()
            .find(|bin| primary_mass_msun <= bin.maximum_primary_mass_msun)
            .expect("validated multiplicity coverage")
    }

    fn imf_weight_between(&self, minimum: f64, maximum: f64) -> f64 {
        let imf = self.model.initial_mass_function;
        let continuity = imf
            .break_mass_msun
            .powf(imf.high_mass_exponent - imf.low_mass_exponent);
        let low_maximum = maximum.min(imf.break_mass_msun);
        let low_weight = if minimum < low_maximum {
            power_law_integral(minimum, low_maximum, imf.low_mass_exponent)
        } else {
            0.0
        };
        let high_minimum = minimum.max(imf.break_mass_msun);
        let high_weight = if high_minimum < maximum {
            continuity * power_law_integral(high_minimum, maximum, imf.high_mass_exponent)
        } else {
            0.0
        };
        low_weight + high_weight
    }
}

fn valid_multiplicity_bins(model: &StellarBirthMassModel) -> bool {
    let imf = model.initial_mass_function;
    if model.multiplicity_bins.is_empty() {
        return false;
    }
    let mut expected_minimum = imf.minimum_mass_msun;
    for bin in &model.multiplicity_bins {
        let fractions = [
            bin.single_system_fraction,
            bin.binary_system_fraction,
            bin.triple_system_fraction,
            bin.higher_order_system_fraction,
        ];
        if !bin.minimum_primary_mass_msun.is_finite()
            || !bin.maximum_primary_mass_msun.is_finite()
            || (bin.minimum_primary_mass_msun - expected_minimum).abs() > 1e-12
            || bin.minimum_primary_mass_msun >= bin.maximum_primary_mass_msun
            || fractions
                .into_iter()
                .any(|fraction| !fraction.is_finite() || fraction < 0.0)
            || (fractions.into_iter().sum::<f64>() - 1.0).abs() > 1e-9
            || bin.representative_higher_order_members < 4
            || !bin.minimum_mass_ratio.is_finite()
            || !(0.0..=1.0).contains(&bin.minimum_mass_ratio)
            || !bin.mass_ratio_power.is_finite()
        {
            return false;
        }
        expected_minimum = bin.maximum_primary_mass_msun;
    }
    (expected_minimum - imf.maximum_mass_msun).abs() <= 1e-12
}

fn power_law_integral(minimum: f64, maximum: f64, exponent: f64) -> f64 {
    if (exponent - 1.0).abs() < 1e-12 {
        (maximum / minimum).ln()
    } else {
        (maximum.powf(1.0 - exponent) - minimum.powf(1.0 - exponent)) / (1.0 - exponent)
    }
}

fn sample_power_law(minimum: f64, maximum: f64, exponent: f64, rng: &mut ChaCha8Rng) -> f64 {
    let draw = rng.gen_range(0.0..1.0);
    if (exponent - 1.0).abs() < 1e-12 {
        minimum * (maximum / minimum).powf(draw)
    } else {
        let power = 1.0 - exponent;
        (minimum.powf(power) + draw * (maximum.powf(power) - minimum.powf(power))).powf(1.0 / power)
    }
}

fn stable_member_id(system_id: u64, rank: u8) -> u64 {
    let mut input = Vec::with_capacity(48);
    input.extend_from_slice(b"star_sim/stellar_member_id/v1");
    input.extend_from_slice(&system_id.to_le_bytes());
    input.push(rank);
    let hash = blake3::hash(&input);
    u64::from_le_bytes(
        hash.as_bytes()[..8]
            .try_into()
            .expect("eight-byte hash prefix"),
    )
}
