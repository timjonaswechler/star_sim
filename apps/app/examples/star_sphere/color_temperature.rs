//! Approximate sRGB colors and visible radiance for black-body temperatures.
//!
//! This module follows the Planckian-locus conversion used by Google's
//! Filament renderer: CIE 1960 UCS to CIE 1931 XYZ, then to gamma-encoded sRGB.

use std::{error::Error, fmt};

use bevy::color::Srgba;

pub const MIN_COLOR_TEMPERATURE_K: f64 = 0.0;
pub const MAX_COLOR_TEMPERATURE_K: f64 = 40_000.0;

/// Bounds for the published accuracy of the Krystek approximation.
pub const MIN_ACCURATE_COLOR_TEMPERATURE_K: f64 = 1_000.0;
pub const MAX_ACCURATE_COLOR_TEMPERATURE_K: f64 = 15_000.0;

/// Intrinsic emission properties of an ideal black-body surface.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlackBodyEmission {
    /// Normalized display chromaticity. This intentionally contains no intensity.
    pub chromaticity: Srgba,
    /// Total radiant power emitted by one square meter of surface into a hemisphere.
    pub radiant_exitance_watts_per_square_meter: f64,
    /// Visible surface luminance weighted by the photopic CIE 1931 observer.
    pub photopic_luminance_candelas_per_square_meter: f64,
}

impl BlackBodyEmission {
    /// Total bolometric luminosity of a spherical emitter with `radius_meters`.
    pub fn spherical_luminosity_watts(self, radius_meters: f64) -> f64 {
        4.0 * std::f64::consts::PI
            * radius_meters.powi(2)
            * self.radiant_exitance_watts_per_square_meter
    }

    /// Bolometric irradiance received at `distance_meters` from the sphere center.
    pub fn irradiance_watts_per_square_meter(
        self,
        radius_meters: f64,
        distance_meters: f64,
    ) -> f64 {
        self.radiant_exitance_watts_per_square_meter * (radius_meters / distance_meters).powi(2)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorTemperatureError {
    pub kelvin: f64,
}

impl fmt::Display for ColorTemperatureError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "color temperature must be finite and between {MIN_COLOR_TEMPERATURE_K:.0} K and {MAX_COLOR_TEMPERATURE_K:.0} K, got {} K",
            self.kelvin
        )
    }
}

impl Error for ColorTemperatureError {}

/// Converts a black-body color temperature to an opaque Bevy [`Srgba`] color.
///
/// This returns normalized chromaticity, not perceived brightness.
/// The approximation has published accuracy from 1,000 K through 15,000 K.
/// Values outside that range are supported for visualization, but extrapolated.
pub fn kelvin_to_srgb(kelvin: f64) -> Result<Srgba, ColorTemperatureError> {
    if !kelvin.is_finite() || !(MIN_COLOR_TEMPERATURE_K..=MAX_COLOR_TEMPERATURE_K).contains(&kelvin)
    {
        return Err(ColorTemperatureError { kelvin });
    }

    if kelvin == 0.0 {
        return Ok(Srgba::BLACK);
    }

    let kelvin_squared = kelvin * kelvin;

    // Planckian locus in CIE 1960 UCS.
    let u = (0.860_117_757 + 1.541_182_54e-4 * kelvin + 1.286_412_12e-7 * kelvin_squared)
        / (1.0 + 8.424_202_35e-4 * kelvin + 7.081_451_63e-7 * kelvin_squared);
    let v = (0.317_398_726 + 4.228_062_45e-5 * kelvin + 4.204_816_91e-8 * kelvin_squared)
        / (1.0 - 2.897_418_16e-5 * kelvin + 1.614_560_53e-7 * kelvin_squared);

    // CIE 1960 UCS to CIE 1931 xyY, with Y normalized to 1.
    let denominator = 2.0 * u - 8.0 * v + 4.0;
    let chromaticity_x = 3.0 * u / denominator;
    let chromaticity_y = 2.0 * v / denominator;
    let x = chromaticity_x / chromaticity_y;
    let y = 1.0;
    let z = (1.0 - chromaticity_x - chromaticity_y) / chromaticity_y;

    // CIE XYZ to linear sRGB (D65), transposed from kmath's column-major form.
    let linear_red = 3.240_454_2 * x - 1.537_138_5 * y - 0.498_531_4 * z;
    let linear_green = -0.969_266_0 * x + 1.876_010_8 * y + 0.041_556_0 * z;
    let linear_blue = 0.055_643_4 * x - 0.204_025_9 * y + 1.057_225_2 * z;
    let maximum = linear_red.max(linear_green).max(linear_blue);

    Ok(Srgba::rgb(
        encode_srgb((linear_red / maximum).max(0.0)) as f32,
        encode_srgb((linear_green / maximum).max(0.0)) as f32,
        encode_srgb((linear_blue / maximum).max(0.0)) as f32,
    ))
}

/// Calculates intrinsic surface emission for an ideal black body.
///
/// The bolometric exitance follows the Stefan-Boltzmann law. Photopic luminance
/// is obtained by integrating Planck spectral radiance from 380–780 nm against
/// the CIE 1931 `y` matching function. Radius and distance are deliberately not
/// part of this calculation; use the methods on [`BlackBodyEmission`] when they
/// are needed by the simulation.
pub fn black_body_emission(kelvin: f64) -> Result<BlackBodyEmission, ColorTemperatureError> {
    let chromaticity = kelvin_to_srgb(kelvin)?;

    if kelvin == 0.0 {
        return Ok(BlackBodyEmission {
            chromaticity,
            radiant_exitance_watts_per_square_meter: 0.0,
            photopic_luminance_candelas_per_square_meter: 0.0,
        });
    }

    const STEFAN_BOLTZMANN_CONSTANT: f64 = 5.670_374_419e-8;
    let visible = visible_emission(kelvin);

    Ok(BlackBodyEmission {
        chromaticity,
        radiant_exitance_watts_per_square_meter: STEFAN_BOLTZMANN_CONSTANT * kelvin.powi(4),
        photopic_luminance_candelas_per_square_meter: 683.0 * visible.xyz[1],
    })
}

/// Approximates how bright and colorful a black-body surface appears to the
/// photopic human eye at a relative exposure measured in stops (EV).
///
/// Planck spectral radiance is integrated from 380–780 nm against analytic
/// approximations of the CIE 1931 2° color-matching functions. Exposure 0 EV is
/// calibrated so a 6,500 K surface reaches photographic middle gray (18% linear
/// display intensity) in its brightest channel after tone mapping. Every +1 EV
/// doubles the exposure. The result does not include surface area, distance, or
/// emissivity.
pub fn black_body_visible_srgb(
    kelvin: f64,
    exposure_ev: f64,
) -> Result<Srgba, ColorTemperatureError> {
    if !kelvin.is_finite() || !(MIN_COLOR_TEMPERATURE_K..=MAX_COLOR_TEMPERATURE_K).contains(&kelvin)
    {
        return Err(ColorTemperatureError { kelvin });
    }

    if kelvin == 0.0 {
        return Ok(Srgba::BLACK);
    }

    let rgb = visible_emission(kelvin).linear_rgb;
    let reference_peak = visible_emission(6_500.0)
        .linear_rgb
        .into_iter()
        .fold(0.0_f64, f64::max);
    let middle_gray_input = 0.18 / (1.0 - 0.18);
    let gain = middle_gray_input * 2.0_f64.powf(exposure_ev) / reference_peak;

    Ok(Srgba::rgb(
        encode_srgb(reinhard_tone_map(rgb[0].max(0.0) * gain)) as f32,
        encode_srgb(reinhard_tone_map(rgb[1].max(0.0) * gain)) as f32,
        encode_srgb(reinhard_tone_map(rgb[2].max(0.0) * gain)) as f32,
    ))
}

struct VisibleEmission {
    xyz: [f64; 3],
    linear_rgb: [f64; 3],
}

fn visible_emission(kelvin: f64) -> VisibleEmission {
    let mut xyz = [0.0; 3];

    for wavelength_nm in (380..=780).step_by(5) {
        let wavelength = f64::from(wavelength_nm);
        let radiance = planck_spectral_radiance(wavelength * 1.0e-9, kelvin);
        let matching = cie_1931_matching_approximation(wavelength);

        for channel in 0..3 {
            xyz[channel] += radiance * matching[channel] * 5.0e-9;
        }
    }

    let linear_rgb = [
        3.240_454_2 * xyz[0] - 1.537_138_5 * xyz[1] - 0.498_531_4 * xyz[2],
        -0.969_266_0 * xyz[0] + 1.876_010_8 * xyz[1] + 0.041_556_0 * xyz[2],
        0.055_643_4 * xyz[0] - 0.204_025_9 * xyz[1] + 1.057_225_2 * xyz[2],
    ];

    VisibleEmission { xyz, linear_rgb }
}

fn planck_spectral_radiance(wavelength_m: f64, kelvin: f64) -> f64 {
    const PLANCK_CONSTANT: f64 = 6.626_070_15e-34;
    const SPEED_OF_LIGHT: f64 = 299_792_458.0;
    const BOLTZMANN_CONSTANT: f64 = 1.380_649e-23;

    let exponent = PLANCK_CONSTANT * SPEED_OF_LIGHT / (wavelength_m * BOLTZMANN_CONSTANT * kelvin);
    let numerator = 2.0 * PLANCK_CONSTANT * SPEED_OF_LIGHT.powi(2);
    numerator / (wavelength_m.powi(5) * exponent.exp_m1())
}

// Analytic approximations of the CIE 1931 2° color-matching functions.
fn cie_1931_matching_approximation(wavelength_nm: f64) -> [f64; 3] {
    let x1 = asymmetric_gaussian(wavelength_nm, 442.0, 0.0624, 0.0374);
    let x2 = asymmetric_gaussian(wavelength_nm, 599.8, 0.0264, 0.0323);
    let x3 = asymmetric_gaussian(wavelength_nm, 501.1, 0.0490, 0.0382);
    let y1 = asymmetric_gaussian(wavelength_nm, 568.8, 0.0213, 0.0247);
    let y2 = asymmetric_gaussian(wavelength_nm, 530.9, 0.0613, 0.0322);
    let z1 = asymmetric_gaussian(wavelength_nm, 437.0, 0.0845, 0.0278);
    let z2 = asymmetric_gaussian(wavelength_nm, 459.0, 0.0385, 0.0725);

    [
        0.362 * x1 + 1.056 * x2 - 0.065 * x3,
        0.821 * y1 + 0.286 * y2,
        1.217 * z1 + 0.681 * z2,
    ]
}

fn asymmetric_gaussian(
    wavelength_nm: f64,
    center_nm: f64,
    left_scale: f64,
    right_scale: f64,
) -> f64 {
    let scale = if wavelength_nm < center_nm {
        left_scale
    } else {
        right_scale
    };
    let distance = (wavelength_nm - center_nm) * scale;
    (-0.5 * distance * distance).exp()
}

fn reinhard_tone_map(linear: f64) -> f64 {
    linear / (1.0 + linear)
}

fn encode_srgb(linear: f64) -> f64 {
    if linear <= 0.003_130_8 {
        12.92 * linear
    } else {
        1.055 * linear.powf(1.0 / 2.4) - 0.055
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_the_planckian_reference_values() {
        let cases = [
            (0.0, [0, 0, 0]),
            (1_000.0, [255, 23, 0]),
            (1_900.0, [255, 132, 0]),
            (2_700.0, [255, 173, 88]),
            (6_500.0, [255, 248, 254]),
            (10_000.0, [205, 217, 255]),
            (40_000.0, [158, 182, 255]),
        ];

        for (kelvin, expected) in cases {
            let color = kelvin_to_srgb(kelvin).unwrap();
            let actual =
                [color.red, color.green, color.blue].map(|channel| (channel * 255.0).round() as u8);
            assert_eq!(actual, expected, "mismatch at {kelvin} K");
            assert_eq!(color.alpha, 1.0);
        }
    }

    #[test]
    fn visible_preview_retains_absolute_spectral_differences() {
        let black = kelvin_to_srgb(0.0).unwrap();
        let low = black_body_visible_srgb(500.0, 0.0).unwrap();
        let warmer = black_body_visible_srgb(1_000.0, 0.0).unwrap();
        let exposed = black_body_visible_srgb(1_000.0, 1.0).unwrap();

        assert_eq!([black.red, black.green, black.blue], [0.0, 0.0, 0.0]);
        assert!(low.red < warmer.red);
        assert!(warmer.red < exposed.red);
    }

    #[test]
    fn emission_reports_surface_power_luminosity_and_received_irradiance() {
        let emission = black_body_emission(5_772.0).unwrap();
        let radius = 6.957e8;
        let distance = 1.495_978_707e11;

        assert!(emission.radiant_exitance_watts_per_square_meter > 6.0e7);
        assert!(emission.photopic_luminance_candelas_per_square_meter > 1.0e8);
        assert!(emission.spherical_luminosity_watts(radius) > 3.0e26);
        assert!(emission.irradiance_watts_per_square_meter(radius, distance) > 1_000.0);
    }

    #[test]
    fn rejects_values_outside_the_supported_range() {
        for kelvin in [-1.0, 40_001.0, f64::NAN, f64::INFINITY] {
            assert!(kelvin_to_srgb(kelvin).is_err());
        }
    }
}
