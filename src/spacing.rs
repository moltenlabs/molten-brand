//! Spacing scale tokens.
//!
//! A consistent spacing scale used across all Molten Labs products.
//! Based on a 4px base unit with deliberate gaps for visual rhythm.

/// Base spacing unit in pixels.
pub const BASE: u16 = 4;

/// Spacing scale values.
///
/// The scale is: 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 40, 48, 64
/// This creates visual rhythm while allowing fine-grained control.
pub mod scale {
    /// 0px - No spacing.
    pub const S0: u16 = 0;
    /// 4px - Tiny spacing (1 unit).
    pub const S1: u16 = 4;
    /// 8px - Extra small (2 units).
    pub const S2: u16 = 8;
    /// 12px - Small (3 units).
    pub const S3: u16 = 12;
    /// 16px - Medium-small (4 units).
    pub const S4: u16 = 16;
    /// 20px - Medium (5 units).
    pub const S5: u16 = 20;
    /// 24px - Medium-large (6 units).
    pub const S6: u16 = 24;
    /// 32px - Large (8 units).
    pub const S8: u16 = 32;
    /// 40px - Extra large (10 units).
    pub const S10: u16 = 40;
    /// 48px - 2X large (12 units).
    pub const S12: u16 = 48;
    /// 64px - 3X large (16 units).
    pub const S16: u16 = 64;
    /// 80px - 4X large (20 units).
    pub const S20: u16 = 80;
    /// 96px - 5X large (24 units).
    pub const S24: u16 = 96;
    /// 128px - 6X large (32 units).
    pub const S32: u16 = 128;
    /// 160px - 7X large (40 units).
    pub const S40: u16 = 160;
    /// 192px - 8X large (48 units).
    pub const S48: u16 = 192;
    /// 256px - 9X large (64 units).
    pub const S64: u16 = 256;
}

/// Semantic spacing aliases.
pub mod semantic {
    use super::scale;

    /// Padding for inline elements.
    pub const INLINE: u16 = scale::S1;

    /// Padding for small components (buttons, inputs).
    pub const COMPONENT_SM: u16 = scale::S2;

    /// Padding for medium components.
    pub const COMPONENT_MD: u16 = scale::S3;

    /// Padding for large components.
    pub const COMPONENT_LG: u16 = scale::S4;

    /// Gap between related items.
    pub const GAP_SM: u16 = scale::S2;

    /// Standard gap.
    pub const GAP_MD: u16 = scale::S4;

    /// Large gap.
    pub const GAP_LG: u16 = scale::S6;

    /// Section padding.
    pub const SECTION: u16 = scale::S8;

    /// Page margin.
    pub const PAGE: u16 = scale::S16;
}

/// Get spacing value by scale index.
///
/// # Example
///
/// ```rust
/// use molten_brand::spacing::get;
///
/// assert_eq!(get(4), 16);
/// assert_eq!(get(8), 32);
/// ```
#[must_use]
pub const fn get(index: u16) -> u16 {
    match index {
        0 => scale::S0,
        1 => scale::S1,
        2 => scale::S2,
        3 => scale::S3,
        5 => scale::S5,
        6 => scale::S6,
        8 => scale::S8,
        10 => scale::S10,
        12 => scale::S12,
        16 => scale::S16,
        20 => scale::S20,
        24 => scale::S24,
        32 => scale::S32,
        40 => scale::S40,
        48 => scale::S48,
        64 => scale::S64,
        _ => scale::S4, // Default to medium (includes 4)
    }
}

/// Convert spacing units to pixels.
#[must_use]
pub const fn units(n: u16) -> u16 {
    n * BASE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale() {
        assert_eq!(scale::S1, 4);
        assert_eq!(scale::S4, 16);
        assert_eq!(scale::S8, 32);
    }

    #[test]
    fn test_units() {
        assert_eq!(units(1), 4);
        assert_eq!(units(4), 16);
        assert_eq!(units(10), 40);
    }
}
