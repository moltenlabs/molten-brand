//! Color type definitions.
//!
//! This module provides the core color types used throughout the brand system.

use std::fmt;

/// An RGB color with 8-bit components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rgb {
    /// Red component (0-255).
    pub r: u8,
    /// Green component (0-255).
    pub g: u8,
    /// Blue component (0-255).
    pub b: u8,
}

impl Rgb {
    /// Create a new RGB color.
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Create an RGB color from a hex string (without #).
    ///
    /// # Panics
    ///
    /// Panics if the hex string is invalid.
    #[must_use]
    pub fn from_hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).expect("Invalid hex");
        let g = u8::from_str_radix(&hex[2..4], 16).expect("Invalid hex");
        let b = u8::from_str_radix(&hex[4..6], 16).expect("Invalid hex");
        Self { r, g, b }
    }

    /// Convert to a hex string with # prefix.
    #[must_use]
    pub fn hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    /// Convert to an RGBA color with full opacity.
    #[must_use]
    pub const fn to_rgba(self) -> Rgba {
        Rgba {
            r: self.r,
            g: self.g,
            b: self.b,
            a: 255,
        }
    }

    /// Convert to normalized floating-point values (0.0 - 1.0).
    #[must_use]
    pub fn to_f32(&self) -> (f32, f32, f32) {
        (
            f32::from(self.r) / 255.0,
            f32::from(self.g) / 255.0,
            f32::from(self.b) / 255.0,
        )
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

/// An RGBA color with 8-bit components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rgba {
    /// Red component (0-255).
    pub r: u8,
    /// Green component (0-255).
    pub g: u8,
    /// Blue component (0-255).
    pub b: u8,
    /// Alpha component (0-255).
    pub a: u8,
}

impl Rgba {
    /// Create a new RGBA color.
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Create an RGBA color from RGB with a given opacity (0.0 - 1.0).
    #[must_use]
    pub fn from_rgb_alpha(rgb: Rgb, alpha: f32) -> Self {
        Self {
            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
            a: (alpha.clamp(0.0, 1.0) * 255.0) as u8,
        }
    }

    /// Convert to an RGB color, discarding alpha.
    #[must_use]
    pub const fn to_rgb(self) -> Rgb {
        Rgb {
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }

    /// Get the alpha as a normalized value (0.0 - 1.0).
    #[must_use]
    pub fn alpha_f32(&self) -> f32 {
        f32::from(self.a) / 255.0
    }

    /// Convert to CSS rgba() format.
    #[must_use]
    pub fn css(&self) -> String {
        format!(
            "rgba({}, {}, {}, {:.2})",
            self.r,
            self.g,
            self.b,
            self.alpha_f32()
        )
    }
}

impl fmt::Display for Rgba {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "rgba({}, {}, {}, {:.2})",
            self.r,
            self.g,
            self.b,
            self.alpha_f32()
        )
    }
}

/// A color that can be either RGB or RGBA.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Color {
    /// Solid RGB color.
    Rgb(Rgb),
    /// RGBA color with alpha.
    Rgba(Rgba),
}

impl Color {
    /// Create a new RGB color.
    #[must_use]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Rgb(Rgb::new(r, g, b))
    }

    /// Create a new RGBA color.
    #[must_use]
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::Rgba(Rgba::new(r, g, b, a))
    }

    /// Create a color from a hex string.
    #[must_use]
    pub fn from_hex(hex: &str) -> Self {
        Self::Rgb(Rgb::from_hex(hex))
    }

    /// Convert to hex string.
    #[must_use]
    pub fn hex(&self) -> String {
        match self {
            Self::Rgb(rgb) => rgb.hex(),
            Self::Rgba(rgba) => rgba.to_rgb().hex(),
        }
    }

    /// Get the RGB components.
    #[must_use]
    pub const fn to_rgb(&self) -> Rgb {
        match self {
            Self::Rgb(rgb) => *rgb,
            Self::Rgba(rgba) => rgba.to_rgb(),
        }
    }

    /// Get the RGBA components.
    #[must_use]
    pub const fn to_rgba(&self) -> Rgba {
        match self {
            Self::Rgb(rgb) => rgb.to_rgba(),
            Self::Rgba(rgba) => *rgba,
        }
    }

    /// Create a new color with modified alpha.
    #[must_use]
    pub fn with_alpha(self, alpha: f32) -> Self {
        let rgb = self.to_rgb();
        Self::Rgba(Rgba::from_rgb_alpha(rgb, alpha))
    }

    /// Transparent color.
    pub const TRANSPARENT: Self = Self::rgba(0, 0, 0, 0);

    /// Black color.
    pub const BLACK: Self = Self::rgb(0, 0, 0);

    /// White color.
    pub const WHITE: Self = Self::rgb(255, 255, 255);
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rgb(rgb) => write!(f, "{rgb}"),
            Self::Rgba(rgba) => write!(f, "{rgba}"),
        }
    }
}

impl From<Rgb> for Color {
    fn from(rgb: Rgb) -> Self {
        Self::Rgb(rgb)
    }
}

impl From<Rgba> for Color {
    fn from(rgba: Rgba) -> Self {
        Self::Rgba(rgba)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_from_hex() {
        let color = Rgb::from_hex("#F97316");
        assert_eq!(color.r, 249);
        assert_eq!(color.g, 115);
        assert_eq!(color.b, 22);
    }

    #[test]
    fn test_rgb_to_hex() {
        let color = Rgb::new(249, 115, 22);
        assert_eq!(color.hex(), "#F97316");
    }

    #[test]
    fn test_rgba_alpha() {
        let rgba = Rgba::from_rgb_alpha(Rgb::new(124, 58, 237), 0.4);
        assert_eq!(rgba.a, 102); // 0.4 * 255 ≈ 102
    }
}
