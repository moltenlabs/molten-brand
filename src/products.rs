//! Product-specific design tokens.
//!
//! Each Molten Labs product has its own visual identity while sharing
//! the core brand DNA. This module provides product-specific color palettes.

use crate::color::Color;

/// Lair product tokens - Terminal for Goblins.
///
/// Lair is the GPU-rendered terminal and multi-agent orchestration platform.
/// Its visual identity is built around the mystical "Goblin Purple" theme.
///
/// # Example
///
/// ```rust
/// use molten_brand::products::lair;
///
/// // Get the Goblin Purple primary color
/// let primary = lair::PRIMARY;
///
/// // Get terminal-specific colors
/// let bg = lair::terminal::BACKGROUND;
/// let cursor = lair::terminal::CURSOR;
/// ```
pub mod lair {
    use super::Color;

    /// Primary color - Goblin Purple (mystical, powerful).
    pub const PRIMARY: Color = Color::rgb(124, 58, 237); // #7C3AED

    /// Secondary color - Purple Light (hover states).
    pub const SECONDARY: Color = Color::rgb(167, 139, 250); // #A78BFA

    /// Accent color - Purple Dark (active states).
    pub const ACCENT: Color = Color::rgb(91, 33, 182); // #5B21B6

    /// Terminal-specific colors.
    pub mod terminal {
        use crate::color::Color;

        /// Terminal background - Cave Dark.
        pub const BACKGROUND: Color = Color::rgb(15, 15, 26); // #0F0F1A

        /// Terminal foreground text.
        pub const FOREGROUND: Color = Color::rgb(228, 228, 231); // #E4E4E7

        /// Cursor color - Goblin Purple.
        pub const CURSOR: Color = Color::rgb(124, 58, 237); // #7C3AED

        /// Selection highlight.
        pub const SELECTION: Color = Color::rgba(124, 58, 237, 77); // ~30% opacity
    }

    /// Goblin effect colors.
    pub mod goblin {
        use crate::color::Color;

        /// Primary goblin color.
        pub const PRIMARY: Color = Color::rgb(124, 58, 237); // #7C3AED

        /// Goblin glow effect.
        pub const GLOW: Color = Color::rgba(124, 58, 237, 102); // ~40% opacity

        /// Goblin shadow.
        pub const SHADOW: Color = Color::rgba(124, 58, 237, 51); // ~20% opacity

        /// Goblin pulse (for animations).
        pub const PULSE: Color = Color::rgba(124, 58, 237, 153); // ~60% opacity
    }

    /// Surface colors for Lair UI.
    pub mod surface {
        use crate::color::Color;

        /// Base surface.
        pub const BASE: Color = Color::rgb(15, 15, 26); // #0F0F1A

        /// Raised surface.
        pub const RAISED: Color = Color::rgb(26, 26, 46); // #1A1A2E

        /// Surface with purple tint.
        pub const TINTED: Color = Color::rgb(37, 37, 56); // #252538

        /// Border color.
        pub const BORDER: Color = Color::rgba(124, 58, 237, 51); // ~20% opacity

        /// Border hover color.
        pub const BORDER_HOVER: Color = Color::rgba(124, 58, 237, 102); // ~40% opacity
    }

    /// Product metadata.
    pub mod meta {
        /// Product name.
        pub const NAME: &str = "Lair";

        /// Product tagline.
        pub const TAGLINE: &str = "The terminal where goblins ship code";

        /// Product description.
        pub const DESCRIPTION: &str =
            "GPU-rendered terminal and multi-agent orchestration platform";
    }
}

/// Hearth product tokens - Content Marketing Platform.
///
/// Hearth is the content-first editorial platform for the Molten Labs community.
/// Its visual identity uses Iron Blue for a trustworthy, editorial feel.
pub mod hearth {
    use super::Color;

    /// Primary color - Iron Blue (trustworthy, editorial).
    pub const PRIMARY: Color = Color::rgb(59, 130, 246); // #3B82F6

    /// Secondary color - Blue Light (hover states).
    pub const SECONDARY: Color = Color::rgb(96, 165, 250); // #60A5FA

    /// Accent color - Blue Dark (emphasis).
    pub const ACCENT: Color = Color::rgb(37, 99, 235); // #2563EB

    /// Editorial text colors.
    pub mod editorial {
        use crate::color::Color;

        /// Primary editorial text.
        pub const TEXT: Color = Color::rgb(229, 229, 229); // #E5E5E5

        /// Secondary text.
        pub const SECONDARY: Color = Color::rgb(163, 163, 163); // #A3A3A3

        /// Tertiary/muted text.
        pub const TERTIARY: Color = Color::rgb(82, 82, 82); // #525252

        /// Border color.
        pub const BORDER: Color = Color::rgb(38, 38, 38); // #262626
    }

    /// Content surface colors.
    pub mod content {
        use crate::color::Color;

        /// Page background.
        pub const BACKGROUND: Color = Color::rgb(10, 10, 10); // #0A0A0A

        /// Card background.
        pub const CARD: Color = Color::rgb(17, 17, 17); // #111111

        /// Card hover state.
        pub const CARD_HOVER: Color = Color::rgb(22, 22, 22); // #161616

        /// Content border.
        pub const BORDER: Color = Color::rgb(38, 38, 38); // #262626
    }

    /// Product metadata.
    pub mod meta {
        /// Product name.
        pub const NAME: &str = "Hearth";

        /// Product tagline.
        pub const TAGLINE: &str = "The warm center where the community gathers";

        /// Product description.
        pub const DESCRIPTION: &str = "Content marketing platform and community hub";
    }
}

/// Alloy product tokens - Design System.
///
/// Alloy is the official design system for Molten Labs, providing the
/// foundation for all products. It uses Molten Orange as its primary color.
pub mod alloy {
    use super::Color;

    /// Primary color - Molten Orange (energy, CTAs).
    pub const PRIMARY: Color = Color::rgb(249, 115, 22); // #F97316

    /// Secondary color - Orange Light.
    pub const SECONDARY: Color = Color::rgb(251, 146, 60); // #FB923C

    /// Accent color - Orange Dark.
    pub const ACCENT: Color = Color::rgb(234, 88, 12); // #EA580C

    /// System surface colors.
    pub mod system {
        use crate::color::Color;

        /// Primary system color.
        pub const PRIMARY: Color = Color::rgb(249, 115, 22); // #F97316

        /// Neutral system color.
        pub const NEUTRAL: Color = Color::rgb(113, 113, 122); // #71717A

        /// Surface color.
        pub const SURFACE: Color = Color::rgb(24, 24, 27); // #18181B
    }

    /// Glass effects for Alloy.
    pub mod glass {
        use crate::color::Color;

        /// Glass background.
        pub const BACKGROUND: Color = Color::rgba(255, 255, 255, 8); // ~3%

        /// Glass background hover.
        pub const BACKGROUND_HOVER: Color = Color::rgba(249, 115, 22, 13); // ~5%

        /// Glass border.
        pub const BORDER: Color = Color::rgba(255, 255, 255, 15); // ~6%

        /// Glass border hover.
        pub const BORDER_HOVER: Color = Color::rgba(249, 115, 22, 77); // ~30%
    }

    /// Product metadata.
    pub mod meta {
        /// Product name.
        pub const NAME: &str = "Alloy";

        /// Product tagline.
        pub const TAGLINE: &str = "Components forged together";

        /// Product description.
        pub const DESCRIPTION: &str = "The official design system for Molten Labs";
    }
}

/// Get product tokens by name.
///
/// # Example
///
/// ```rust
/// use molten_brand::products::get_product_primary;
///
/// let lair_primary = get_product_primary("lair");
/// let hearth_primary = get_product_primary("hearth");
/// ```
#[must_use]
pub fn get_product_primary(product: &str) -> Color {
    match product.to_lowercase().as_str() {
        "lair" => lair::PRIMARY,
        "hearth" => hearth::PRIMARY,
        _ => alloy::PRIMARY, // Default to Alloy (includes "alloy")
    }
}

/// Get product tagline by name.
#[must_use]
pub fn get_product_tagline(product: &str) -> &'static str {
    match product.to_lowercase().as_str() {
        "lair" => lair::meta::TAGLINE,
        "hearth" => hearth::meta::TAGLINE,
        _ => alloy::meta::TAGLINE, // Default to Alloy (includes "alloy")
    }
}
