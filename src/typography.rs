//! Typography tokens.
//!
//! Font families, sizes, weights, and line heights for consistent typography.

/// Font family stacks.
pub mod families {
    /// Primary sans-serif font stack (Geist Sans).
    pub const SANS: &str = "\"Geist Sans\", system-ui, -apple-system, BlinkMacSystemFont, \"Segoe UI\", Roboto, \"Helvetica Neue\", Arial, sans-serif";

    /// Monospace font stack (Geist Mono).
    pub const MONO: &str = "\"Geist Mono\", \"SF Mono\", Menlo, Monaco, Consolas, \"Liberation Mono\", \"Courier New\", monospace";

    /// Display font stack (Space Grotesk for headlines).
    pub const DISPLAY: &str =
        "\"Space Grotesk\", \"Geist Sans\", system-ui, -apple-system, sans-serif";

    /// Serif font stack (for Hearth editorial content).
    pub const SERIF: &str =
        "\"Fraunces\", \"Georgia\", \"Times New Roman\", \"Times\", serif";
}

/// Font size scale in pixels.
pub mod sizes {
    /// 12px - Tiny (footnotes, legal).
    pub const TINY: u16 = 12;
    /// 14px - Small (captions, labels).
    pub const SMALL: u16 = 14;
    /// 16px - Base (body text).
    pub const BASE: u16 = 16;
    /// 18px - Large body.
    pub const LARGE: u16 = 18;
    /// 20px - Lead paragraph.
    pub const LEAD: u16 = 20;
    /// 22px - H4 / Subsection.
    pub const H4: u16 = 22;
    /// 24px - H3.
    pub const H3: u16 = 24;
    /// 28px - H2 / Section headers.
    pub const H2: u16 = 28;
    /// 36px - H1 / Page titles.
    pub const H1: u16 = 36;
    /// 48px - Display / Hero headlines.
    pub const DISPLAY: u16 = 48;
    /// 60px - Display large.
    pub const DISPLAY_LG: u16 = 60;
    /// 72px - Display extra large.
    pub const DISPLAY_XL: u16 = 72;
}

/// Font weights.
pub mod weights {
    /// 100 - Thin.
    pub const THIN: u16 = 100;
    /// 200 - Extra Light.
    pub const EXTRA_LIGHT: u16 = 200;
    /// 300 - Light.
    pub const LIGHT: u16 = 300;
    /// 400 - Regular/Normal.
    pub const REGULAR: u16 = 400;
    /// 500 - Medium.
    pub const MEDIUM: u16 = 500;
    /// 600 - Semi Bold.
    pub const SEMI_BOLD: u16 = 600;
    /// 700 - Bold.
    pub const BOLD: u16 = 700;
    /// 800 - Extra Bold.
    pub const EXTRA_BOLD: u16 = 800;
    /// 900 - Black.
    pub const BLACK: u16 = 900;
}

/// Line height multipliers.
pub mod line_heights {
    /// Tight line height (1.1) - for headings.
    pub const TIGHT: f32 = 1.1;
    /// Snug line height (1.25) - for subheadings.
    pub const SNUG: f32 = 1.25;
    /// Normal line height (1.5) - for body text.
    pub const NORMAL: f32 = 1.5;
    /// Relaxed line height (1.625) - for comfortable reading.
    pub const RELAXED: f32 = 1.625;
    /// Loose line height (2.0) - for emphasis.
    pub const LOOSE: f32 = 2.0;
}

/// Letter spacing adjustments (in em units).
pub mod letter_spacing {
    /// Tighter (-0.025em) - for large headings.
    pub const TIGHTER: f32 = -0.025;
    /// Tight (-0.015em) - for headings.
    pub const TIGHT: f32 = -0.015;
    /// Normal (0) - for body text.
    pub const NORMAL: f32 = 0.0;
    /// Wide (0.025em) - for small caps, labels.
    pub const WIDE: f32 = 0.025;
    /// Wider (0.05em) - for all caps.
    pub const WIDER: f32 = 0.05;
    /// Widest (0.1em) - for extreme emphasis.
    pub const WIDEST: f32 = 0.1;
}

/// Typography preset for a text style.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextStyle {
    /// Font family.
    pub family: &'static str,
    /// Font size in pixels.
    pub size: u16,
    /// Font weight.
    pub weight: u16,
    /// Line height multiplier.
    pub line_height: f32,
    /// Letter spacing in em.
    pub letter_spacing: f32,
}

/// Pre-defined text style presets.
pub mod presets {
    use super::{families, letter_spacing, line_heights, sizes, weights, TextStyle};

    /// Display heading style.
    pub const DISPLAY: TextStyle = TextStyle {
        family: families::DISPLAY,
        size: sizes::DISPLAY,
        weight: weights::BOLD,
        line_height: line_heights::TIGHT,
        letter_spacing: letter_spacing::TIGHTER,
    };

    /// H1 heading style.
    pub const H1: TextStyle = TextStyle {
        family: families::SANS,
        size: sizes::H1,
        weight: weights::BOLD,
        line_height: line_heights::TIGHT,
        letter_spacing: letter_spacing::TIGHT,
    };

    /// H2 heading style.
    pub const H2: TextStyle = TextStyle {
        family: families::SANS,
        size: sizes::H2,
        weight: weights::SEMI_BOLD,
        line_height: line_heights::SNUG,
        letter_spacing: letter_spacing::TIGHT,
    };

    /// H3 heading style.
    pub const H3: TextStyle = TextStyle {
        family: families::SANS,
        size: sizes::H3,
        weight: weights::SEMI_BOLD,
        line_height: line_heights::SNUG,
        letter_spacing: letter_spacing::NORMAL,
    };

    /// Body text style.
    pub const BODY: TextStyle = TextStyle {
        family: families::SANS,
        size: sizes::BASE,
        weight: weights::REGULAR,
        line_height: line_heights::NORMAL,
        letter_spacing: letter_spacing::NORMAL,
    };

    /// Small text style.
    pub const SMALL: TextStyle = TextStyle {
        family: families::SANS,
        size: sizes::SMALL,
        weight: weights::REGULAR,
        line_height: line_heights::NORMAL,
        letter_spacing: letter_spacing::NORMAL,
    };

    /// Code/monospace style.
    pub const CODE: TextStyle = TextStyle {
        family: families::MONO,
        size: sizes::SMALL,
        weight: weights::REGULAR,
        line_height: line_heights::RELAXED,
        letter_spacing: letter_spacing::NORMAL,
    };

    /// Label style (small caps effect).
    pub const LABEL: TextStyle = TextStyle {
        family: families::SANS,
        size: sizes::TINY,
        weight: weights::MEDIUM,
        line_height: line_heights::NORMAL,
        letter_spacing: letter_spacing::WIDE,
    };
}
