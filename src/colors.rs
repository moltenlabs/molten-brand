//! Core color palettes.
//!
//! This module contains the foundational color definitions shared across
//! all Molten Labs products.

use crate::color::Color;

/// Forge brand colors - the parent company palette.
///
/// These colors represent the core Forge brand and are shared across all products.
pub mod forge {
    use super::*;

    /// Primary background color - deep black.
    pub const BLACK: Color = Color::rgb(10, 10, 10); // #0A0A0A

    /// Secondary text and borders - steel gray.
    pub const STEEL: Color = Color::rgb(113, 113, 122); // #71717A

    /// Primary text color - near white.
    pub const WHITE: Color = Color::rgb(250, 250, 250); // #FAFAFA

    /// Energy and highlights - molten orange.
    pub const MOLTEN: Color = Color::rgb(249, 115, 22); // #F97316

    /// Accent and alerts - ember red.
    pub const EMBER: Color = Color::rgb(239, 68, 68); // #EF4444

    /// Links and interactive elements - iron blue.
    pub const IRON: Color = Color::rgb(59, 130, 246); // #3B82F6
}

/// Molten orange scale - the primary brand color.
///
/// A warm orange that represents energy, creativity, and the forge's fire.
pub mod molten {
    use super::*;

    /// Molten 50 - lightest.
    pub const SCALE_50: Color = Color::rgb(255, 247, 237); // #FFF7ED
    /// Molten 100.
    pub const SCALE_100: Color = Color::rgb(255, 237, 213); // #FFEDD5
    /// Molten 200.
    pub const SCALE_200: Color = Color::rgb(254, 215, 170); // #FED7AA
    /// Molten 300.
    pub const SCALE_300: Color = Color::rgb(253, 186, 116); // #FDBA74
    /// Molten 400.
    pub const SCALE_400: Color = Color::rgb(251, 146, 60); // #FB923C
    /// Molten 500 - primary brand color.
    pub const SCALE_500: Color = Color::rgb(249, 115, 22); // #F97316
    /// Molten 600.
    pub const SCALE_600: Color = Color::rgb(234, 88, 12); // #EA580C
    /// Molten 700.
    pub const SCALE_700: Color = Color::rgb(194, 65, 12); // #C2410C
    /// Molten 800.
    pub const SCALE_800: Color = Color::rgb(154, 52, 18); // #9A3412
    /// Molten 900.
    pub const SCALE_900: Color = Color::rgb(124, 45, 18); // #7C2D12
    /// Molten 950 - darkest.
    pub const SCALE_950: Color = Color::rgb(67, 20, 7); // #431407

    /// Primary brand color (alias for SCALE_500).
    pub const PRIMARY: Color = SCALE_500;
}

/// Neutral gray scale.
///
/// A consistent gray scale used for text, borders, and backgrounds.
pub mod neutral {
    use super::*;

    /// Neutral 0 - pure white.
    pub const SCALE_0: Color = Color::rgb(255, 255, 255); // #FFFFFF
    /// Neutral 50.
    pub const SCALE_50: Color = Color::rgb(250, 250, 250); // #FAFAFA
    /// Neutral 100.
    pub const SCALE_100: Color = Color::rgb(244, 244, 245); // #F4F4F5
    /// Neutral 200.
    pub const SCALE_200: Color = Color::rgb(228, 228, 231); // #E4E4E7
    /// Neutral 300.
    pub const SCALE_300: Color = Color::rgb(212, 212, 216); // #D4D4D8
    /// Neutral 400.
    pub const SCALE_400: Color = Color::rgb(161, 161, 170); // #A1A1AA
    /// Neutral 500 - mid gray.
    pub const SCALE_500: Color = Color::rgb(113, 113, 122); // #71717A
    /// Neutral 600.
    pub const SCALE_600: Color = Color::rgb(82, 82, 91); // #52525B
    /// Neutral 700.
    pub const SCALE_700: Color = Color::rgb(63, 63, 70); // #3F3F46
    /// Neutral 800.
    pub const SCALE_800: Color = Color::rgb(39, 39, 42); // #27272A
    /// Neutral 900.
    pub const SCALE_900: Color = Color::rgb(24, 24, 27); // #18181B
    /// Neutral 950 - near black.
    pub const SCALE_950: Color = Color::rgb(10, 10, 10); // #0A0A0A
}

/// Surface colors for dark mode UI.
pub mod surface {
    use super::*;

    /// Base app background.
    pub const BASE: Color = Color::rgb(10, 10, 10); // #0A0A0A

    /// Raised surfaces (cards, etc.).
    pub const RAISED: Color = Color::rgb(24, 24, 27); // #18181B

    /// Overlay surfaces (modals, dropdowns).
    pub const OVERLAY: Color = Color::rgb(39, 39, 42); // #27272A

    /// Muted/disabled surfaces.
    pub const MUTED: Color = Color::rgb(63, 63, 70); // #3F3F46
}

/// Text colors.
pub mod text {
    use super::*;

    /// Primary text color.
    pub const PRIMARY: Color = Color::rgb(250, 250, 250); // #FAFAFA

    /// Secondary text color.
    pub const SECONDARY: Color = Color::rgb(161, 161, 170); // #A1A1AA

    /// Muted text color.
    pub const MUTED: Color = Color::rgb(113, 113, 122); // #71717A

    /// Inverse text (for light backgrounds).
    pub const INVERSE: Color = Color::rgb(10, 10, 10); // #0A0A0A

    /// Brand-colored text.
    pub const BRAND: Color = Color::rgb(249, 115, 22); // #F97316
}

/// Glass/transparency effects.
pub mod glass {
    use crate::color::Color;

    /// Glass background (3% white).
    pub const BACKGROUND: Color = Color::rgba(255, 255, 255, 8); // ~3% opacity

    /// Glass background on hover.
    pub const BACKGROUND_HOVER: Color = Color::rgba(249, 115, 22, 13); // ~5% opacity

    /// Glass border.
    pub const BORDER: Color = Color::rgba(255, 255, 255, 15); // ~6% opacity

    /// Glass border on hover.
    pub const BORDER_HOVER: Color = Color::rgba(249, 115, 22, 77); // ~30% opacity
}

/// Helper to get a neutral color by scale (0-950).
#[must_use]
pub fn neutral_scale(scale: u16) -> Color {
    match scale {
        0 => neutral::SCALE_0,
        50 => neutral::SCALE_50,
        100 => neutral::SCALE_100,
        200 => neutral::SCALE_200,
        300 => neutral::SCALE_300,
        400 => neutral::SCALE_400,
        500 => neutral::SCALE_500,
        600 => neutral::SCALE_600,
        700 => neutral::SCALE_700,
        800 => neutral::SCALE_800,
        900 => neutral::SCALE_900,
        950 => neutral::SCALE_950,
        _ => neutral::SCALE_500, // Default to mid
    }
}

/// Helper to get a molten color by scale (50-950).
#[must_use]
pub fn molten_scale(scale: u16) -> Color {
    match scale {
        50 => molten::SCALE_50,
        100 => molten::SCALE_100,
        200 => molten::SCALE_200,
        300 => molten::SCALE_300,
        400 => molten::SCALE_400,
        500 => molten::SCALE_500,
        600 => molten::SCALE_600,
        700 => molten::SCALE_700,
        800 => molten::SCALE_800,
        900 => molten::SCALE_900,
        950 => molten::SCALE_950,
        _ => molten::SCALE_500, // Default to primary
    }
}
