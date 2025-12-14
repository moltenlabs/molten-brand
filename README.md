# Molten Brand 🔥

> Design tokens and brand system for Molten Labs CLI tools.

[![Crates.io](https://img.shields.io/crates/v/molten_brand.svg)](https://crates.io/crates/molten_brand)
[![Documentation](https://docs.rs/molten_brand/badge.svg)](https://docs.rs/molten_brand)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)

## Overview

`molten_brand` is the **single source of truth** for all design tokens in the Molten Labs ecosystem. It provides:

- 🎨 **Color palettes** - Forge brand colors, product-specific themes, semantic colors
- 📐 **Spacing scale** - Consistent spacing values across all components
- 🔤 **Typography** - Font stacks, sizes, weights, and presets
- 🏭 **Product tokens** - Lair (Terminal), Hearth (Content), Alloy (Design System)

This crate mirrors the TypeScript tokens in `@moltenlabs/alloy` and ensures consistent branding across Rust and web applications.

## Installation

```bash
cargo add molten_brand
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
molten_brand = "0.1"
```

## Quick Start

```rust
use molten_brand::{colors, products, semantic};

// Get the primary Molten Orange
let primary = colors::molten::PRIMARY;
println!("Molten Orange: {}", primary.hex()); // #F97316

// Get product-specific colors (Lair/Goblin theme)
let goblin_purple = products::lair::PRIMARY;
let cave_dark = products::lair::terminal::BACKGROUND;

// Semantic colors work across all products
let success = semantic::SUCCESS;
let error = semantic::ERROR;
```

## Brand Hierarchy

```
┌─────────────────────────────────────────┐
│           MOLTEN LABS (Parent)          │
│  ┌──────────┐  ┌──────────┐  ┌──────┐  │
│  │  LAIR    │  │  HEARTH  │  │ALLOY │  │
│  │(Terminal)│  │(Content) │  │(DS)  │  │
│  │ Purple   │  │ Blue     │  │Orange│  │
│  └──────────┘  └──────────┘  └──────┘  │
└─────────────────────────────────────────┘
```

## Color Modules

### `colors::forge` - Parent Brand

The foundational Forge brand colors shared across all products.

```rust
use molten_brand::colors::forge;

let bg = forge::BLACK;     // #0A0A0A - Primary background
let text = forge::WHITE;   // #FAFAFA - Primary text
let steel = forge::STEEL;  // #71717A - Secondary elements
let accent = forge::MOLTEN; // #F97316 - Energy/highlights
```

### `colors::molten` - Primary Scale

The warm orange scale representing the forge's fire.

```rust
use molten_brand::colors::molten;

let light = molten::SCALE_300;  // #FDBA74
let primary = molten::PRIMARY;   // #F97316 (same as SCALE_500)
let dark = molten::SCALE_700;   // #C2410C
```

### `colors::neutral` - Gray Scale

Consistent grays for text, borders, and backgrounds.

```rust
use molten_brand::colors::neutral;

let text_primary = neutral::SCALE_50;   // #FAFAFA
let text_secondary = neutral::SCALE_400; // #A1A1AA
let border = neutral::SCALE_800;        // #27272A
let bg = neutral::SCALE_950;            // #0A0A0A
```

## Product Tokens

### Lair - Terminal for Goblins

```rust
use molten_brand::products::lair;

// Brand colors
let primary = lair::PRIMARY;      // Goblin Purple #7C3AED
let secondary = lair::SECONDARY;  // #A78BFA

// Terminal colors
let term_bg = lair::terminal::BACKGROUND;  // Cave Dark #0F0F1A
let term_fg = lair::terminal::FOREGROUND;  // #E4E4E7
let cursor = lair::terminal::CURSOR;       // #7C3AED

// Goblin effects
let glow = lair::goblin::GLOW;  // rgba(124, 58, 237, 0.4)
```

### Hearth - Content Platform

```rust
use molten_brand::products::hearth;

// Brand colors
let primary = hearth::PRIMARY;  // Iron Blue #3B82F6

// Editorial colors
let text = hearth::editorial::TEXT;       // #E5E5E5
let secondary = hearth::editorial::SECONDARY; // #A3A3A3

// Content surfaces
let card = hearth::content::CARD;         // #111111
let card_hover = hearth::content::CARD_HOVER; // #161616
```

### Alloy - Design System

```rust
use molten_brand::products::alloy;

// Brand colors  
let primary = alloy::PRIMARY;  // Molten Orange #F97316

// Glass effects
let glass_bg = alloy::glass::BACKGROUND;      // rgba(255, 255, 255, 0.03)
let glass_border = alloy::glass::BORDER_HOVER; // rgba(249, 115, 22, 0.3)
```

## Semantic Colors

Colors that convey meaning, consistent across all products:

```rust
use molten_brand::semantic;

let success = semantic::SUCCESS;  // #10B981 - Positive outcomes
let warning = semantic::WARNING;  // #F59E0B - Caution needed
let error = semantic::ERROR;      // #EF4444 - Problems/failures
let info = semantic::INFO;        // #3B82F6 - Neutral information

// Agent status colors (for Lair/Goblin)
let running = semantic::agent::RUNNING;   // Green
let thinking = semantic::agent::THINKING; // Amber
let failed = semantic::agent::FAILED;     // Red
```

## Typography

```rust
use molten_brand::typography::{families, sizes, weights, presets};

// Font families
let sans = families::SANS;   // Geist Sans stack
let mono = families::MONO;   // Geist Mono stack
let display = families::DISPLAY; // Space Grotesk

// Sizes
let body_size = sizes::BASE;     // 16px
let heading_size = sizes::H1;    // 36px

// Presets
let h1_style = presets::H1;
let body_style = presets::BODY;
let code_style = presets::CODE;
```

## Spacing

```rust
use molten_brand::spacing::{scale, semantic, get, units};

// Scale values
let small = scale::S2;   // 8px
let medium = scale::S4;  // 16px
let large = scale::S8;   // 32px

// Semantic aliases
let component_padding = semantic::COMPONENT_MD; // 12px
let section_spacing = semantic::SECTION;        // 32px

// Helpers
let by_index = get(4);    // 16px
let by_units = units(5);  // 20px (5 × 4px base)
```

## Features

- `serde` - Enable serialization/deserialization of color types

```toml
[dependencies]
molten_brand = { version = "0.1", features = ["serde"] }
```

## Integration with Other Crates

`molten_brand` is designed to work with the Molten Labs ecosystem:

- **[lacquer](https://crates.io/crates/lacquer)** - Terminal styling (uses brand tokens)
- **[cauldron](https://crates.io/crates/cauldron)** - TUI framework (uses brand tokens)
- **[sigil](https://crates.io/crates/sigil)** - ANSI sequences
- **[ember](https://crates.io/crates/ember)** - Markdown rendering

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

*Part of the [Molten Labs](https://github.com/moltenlabs) ecosystem. "Let them cook." 🔥*
