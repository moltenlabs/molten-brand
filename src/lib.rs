//! # Molten Brand
//!
//! Design tokens and brand system for Molten Labs CLI tools.
//!
//! This crate provides the **single source of truth** for all branding across
//! the Molten Labs ecosystem. It mirrors the TypeScript tokens in `@moltenlabs/alloy`
//! and ensures consistent branding in Rust applications.
//!
//! ## Quick Start
//!
//! ```rust
//! use molten_brand::{colors, products, semantic};
//!
//! // Get the primary Molten Orange
//! let primary = colors::molten::PRIMARY;
//! println!("Molten Orange: {}", primary.hex());
//!
//! // Get product-specific colors (Lair/Goblin theme)
//! let goblin_purple = products::lair::PRIMARY;
//! let cave_dark = products::lair::terminal::BACKGROUND;
//!
//! // Semantic colors work across all products
//! let success = semantic::SUCCESS;
//! ```
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │           MOLTEN LABS (Parent)          │
//! │  ┌──────────┐  ┌──────────┐  ┌──────┐  │
//! │  │  LAIR    │  │  HEARTH  │  │ALLOY │  │
//! │  │(Terminal)│  │(Content) │  │(DS)  │  │
//! │  │ Purple   │  │ Blue     │  │Orange│  │
//! │  └──────────┘  └──────────┘  └──────┘  │
//! └─────────────────────────────────────────┘
//! ```
//!
//! ## Features
//!
//! - `serde` - Enable serialization/deserialization of color types

#![deny(missing_docs)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]

mod color;
pub mod colors;
pub mod products;
pub mod semantic;
pub mod spacing;
pub mod typography;

pub use color::{Color, Rgb, Rgba};

/// Re-export all color modules for convenience.
pub mod prelude {
    pub use crate::color::{Color, Rgb, Rgba};
    pub use crate::colors::*;
    pub use crate::products::*;
    pub use crate::semantic::*;
}

/// Brand metadata.
pub mod brand {
    /// The company name.
    pub const COMPANY: &str = "Molten Labs";

    /// The primary tagline.
    pub const TAGLINE: &str = "Let them cook";

    /// The website URL.
    pub const WEBSITE: &str = "https://molten.dev";

    /// The GitHub organization.
    pub const GITHUB: &str = "https://github.com/moltenlabs";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_hex() {
        assert_eq!(colors::forge::BLACK.hex(), "#0A0A0A");
        assert_eq!(colors::molten::PRIMARY.hex(), "#F97316");
    }

    #[test]
    fn test_product_colors() {
        assert_eq!(products::lair::PRIMARY.hex(), "#7C3AED");
        assert_eq!(products::hearth::PRIMARY.hex(), "#3B82F6");
        assert_eq!(products::alloy::PRIMARY.hex(), "#F97316");
    }

    #[test]
    fn test_semantic_colors() {
        assert_eq!(semantic::SUCCESS.hex(), "#10B981");
        assert_eq!(semantic::ERROR.hex(), "#EF4444");
    }
}
