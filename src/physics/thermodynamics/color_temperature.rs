//! Approximate display colors for black-body color temperatures.
//!
//! The RGB approximation is the Tanner Helland algorithm used by WuTools. It is
//! intended for screen previews, not for spectral or colorimetric calculations.

use std::{error::Error, fmt};

pub const MIN_COLOR_TEMPERATURE_K: f64 = 1_000.0;
pub const MAX_COLOR_TEMPERATURE_K: f64 = 40_000.0;
/// Upper bound for the published accuracy of the Krystek approximation.
pub const MAX_ACCURATE_PLANCKIAN_TEMPERATURE_K: f64 = 15_000.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Srgb {
    /// Gamma-encoded sRGB channel in the range 0.0..=1.0.
    pub red: f64,
    /// Gamma-encoded sRGB channel in the range 0.0..=1.0.
    pub green: f64,
    /// Gamma-encoded sRGB channel in the range 0.0..=1.0.
    pub blue: f64,
}

impl Srgb {
    pub fn to_rgb8(self) -> Rgb {
        Rgb {
            red: normalized_channel(self.red),
            green: normalized_channel(self.green),
            blue: normalized_channel(self.blue),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Rgb {
    pub fn to_hex(self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }

    pub fn to_hsl(self) -> Hsl {
        let red = f64::from(self.red) / 255.0;
        let green = f64::from(self.green) / 255.0;
        let blue = f64::from(self.blue) / 255.0;
        let maximum = red.max(green).max(blue);
        let minimum = red.min(green).min(blue);
        let delta = maximum - minimum;
        let lightness = (maximum + minimum) / 2.0;

        if delta == 0.0 {
            return Hsl {
                hue_degrees: 0,
                saturation_percent: 0,
                lightness_percent: percent(lightness),
            };
        }

        let hue = if maximum == red {
            60.0 * ((green - blue) / delta).rem_euclid(6.0)
        } else if maximum == green {
            60.0 * ((blue - red) / delta + 2.0)
        } else {
            60.0 * ((red - green) / delta + 4.0)
        };
        let saturation = delta / (1.0 - (2.0 * lightness - 1.0).abs());

        Hsl {
            hue_degrees: (hue.round() as u16) % 360,
            saturation_percent: percent(saturation),
            lightness_percent: percent(lightness),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hsl {
    pub hue_degrees: u16,
    pub saturation_percent: u8,
    pub lightness_percent: u8,
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

/// Converts a color temperature to an approximate sRGB triplet.
///
/// This deliberately rejects values outside the range supported by the fitted
/// curves instead of silently clamping them.
pub fn kelvin_to_rgb(kelvin: f64) -> Result<Rgb, ColorTemperatureError> {
    if !kelvin.is_finite() || !(MIN_COLOR_TEMPERATURE_K..=MAX_COLOR_TEMPERATURE_K).contains(&kelvin)
    {
        return Err(ColorTemperatureError { kelvin });
    }

    let temperature = kelvin / 100.0;

    let red = if temperature <= 66.0 {
        255.0
    } else {
        329.698_727_446 * (temperature - 60.0).powf(-0.133_204_759_2)
    };

    let green = if temperature <= 66.0 {
        99.470_802_586_1 * temperature.ln() - 161.119_568_166_1
    } else {
        288.122_169_528_3 * (temperature - 60.0).powf(-0.075_514_849_2)
    };

    let blue = if temperature >= 66.0 {
        255.0
    } else if temperature <= 19.0 {
        0.0
    } else {
        138.517_731_223_1 * (temperature - 10.0).ln() - 305.044_792_730_7
    };

    Ok(Rgb {
        red: channel(red),
        green: channel(green),
        blue: channel(blue),
    })
}

pub fn kelvin_to_hsl(kelvin: f64) -> Result<Hsl, ColorTemperatureError> {
    kelvin_to_rgb(kelvin).map(Rgb::to_hsl)
}

/// Converts the Planckian locus to normalized, gamma-encoded sRGB.
///
/// This follows the CC0 implementation shared by `kettle11`, based on the
/// Krystek approximation documented by Google's Filament renderer. The
/// approximation has published accuracy through 15,000 K; values through
/// 40,000 K are accepted here to support the same working range as WuTools,
/// but should be treated as an extrapolation above 15,000 K.
pub fn kelvin_to_planckian_srgb(kelvin: f64) -> Result<Srgb, ColorTemperatureError> {
    if !kelvin.is_finite() || !(MIN_COLOR_TEMPERATURE_K..=MAX_COLOR_TEMPERATURE_K).contains(&kelvin)
    {
        return Err(ColorTemperatureError { kelvin });
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

    Ok(Srgb {
        red: encode_srgb((linear_red / maximum).max(0.0)),
        green: encode_srgb((linear_green / maximum).max(0.0)),
        blue: encode_srgb((linear_blue / maximum).max(0.0)),
    })
}

fn channel(value: f64) -> u8 {
    value.round().clamp(0.0, 255.0) as u8
}

fn percent(value: f64) -> u8 {
    (value * 100.0).round().clamp(0.0, 100.0) as u8
}

fn normalized_channel(value: f64) -> u8 {
    (value * 255.0).round().clamp(0.0, 255.0) as u8
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
    fn matches_wutools_reference_values() {
        let cases = [
            (
                1_000.0,
                Rgb {
                    red: 255,
                    green: 68,
                    blue: 0,
                },
                Hsl {
                    hue_degrees: 16,
                    saturation_percent: 100,
                    lightness_percent: 50,
                },
            ),
            (
                2_700.0,
                Rgb {
                    red: 255,
                    green: 167,
                    blue: 87,
                },
                Hsl {
                    hue_degrees: 29,
                    saturation_percent: 100,
                    lightness_percent: 67,
                },
            ),
            (
                6_500.0,
                Rgb {
                    red: 255,
                    green: 254,
                    blue: 250,
                },
                Hsl {
                    hue_degrees: 48,
                    saturation_percent: 100,
                    lightness_percent: 99,
                },
            ),
            (
                10_000.0,
                Rgb {
                    red: 202,
                    green: 218,
                    blue: 255,
                },
                Hsl {
                    hue_degrees: 222,
                    saturation_percent: 100,
                    lightness_percent: 90,
                },
            ),
            (
                40_000.0,
                Rgb {
                    red: 152,
                    green: 186,
                    blue: 255,
                },
                Hsl {
                    hue_degrees: 220,
                    saturation_percent: 100,
                    lightness_percent: 80,
                },
            ),
        ];

        for (kelvin, expected_rgb, expected_hsl) in cases {
            assert_eq!(kelvin_to_rgb(kelvin), Ok(expected_rgb));
            assert_eq!(kelvin_to_hsl(kelvin), Ok(expected_hsl));
        }
    }

    #[test]
    fn formats_uppercase_hex() {
        assert_eq!(kelvin_to_rgb(2_700.0).unwrap().to_hex(), "#FFA757");
    }

    #[test]
    fn planckian_conversion_matches_the_cc0_reference_implementation() {
        let cases = [
            (
                1_000.0,
                Rgb {
                    red: 255,
                    green: 23,
                    blue: 0,
                },
            ),
            (
                1_900.0,
                Rgb {
                    red: 255,
                    green: 132,
                    blue: 0,
                },
            ),
            (
                2_700.0,
                Rgb {
                    red: 255,
                    green: 173,
                    blue: 88,
                },
            ),
            (
                6_500.0,
                Rgb {
                    red: 255,
                    green: 248,
                    blue: 254,
                },
            ),
            (
                10_000.0,
                Rgb {
                    red: 205,
                    green: 217,
                    blue: 255,
                },
            ),
            (
                40_000.0,
                Rgb {
                    red: 158,
                    green: 182,
                    blue: 255,
                },
            ),
        ];

        for (kelvin, expected) in cases {
            assert_eq!(
                kelvin_to_planckian_srgb(kelvin).unwrap().to_rgb8(),
                expected
            );
        }
    }

    #[test]
    fn rejects_values_outside_the_fitted_range() {
        for kelvin in [999.0, 40_001.0, f64::NAN, f64::INFINITY] {
            assert!(kelvin_to_rgb(kelvin).is_err());
        }
    }
}
