<p align="center">
  <img src="https://raw.githubusercontent.com/moltenlabs/molten-brand/main/.github/assets/banner.png" alt="Molten Brand" width="100%" />
</p>

<h1 align="center">🔥 Molten Brand</h1>

<p align="center">
  <strong>Design tokens and brand system for building beautiful CLI tools in Rust.</strong>
</p>

<p align="center">
  <a href="https://crates.io/crates/molten_brand"><img src="https://img.shields.io/crates/v/molten_brand.svg?style=flat-square&logo=rust" alt="Crates.io"></a>
  <a href="https://docs.rs/molten_brand"><img src="https://img.shields.io/docsrs/molten_brand?style=flat-square&logo=docs.rs" alt="Documentation"></a>
  <a href="https://github.com/moltenlabs/molten-brand/actions"><img src="https://img.shields.io/github/actions/workflow/status/moltenlabs/molten-brand/ci.yml?style=flat-square&logo=github" alt="CI"></a>
  <a href="#license"><img src="https://img.shields.io/badge/license-MIT%2FApache--2.0-blue?style=flat-square" alt="License"></a>
</p>

<p align="center">
  <a href="#features">Features</a> •
  <a href="#installation">Installation</a> •
  <a href="#quick-start">Quick Start</a> •
  <a href="#documentation">Documentation</a> •
  <a href="#ecosystem">Ecosystem</a>
</p>

---

## Why Molten Brand?

Building beautiful CLI tools shouldn't mean hardcoding hex values everywhere. **Molten Brand** gives you a complete design system in Rust—the same tokens powering [Lair](https://github.com/moltenlabs/lair), our GPU-rendered terminal.

```rust
use molten_brand::{colors, products, semantic};

// One source of truth for your entire CLI
let primary = colors::molten::PRIMARY;        // #F97316 - Molten Orange
let success = semantic::SUCCESS;               // #10B981 - Consistent everywhere
let terminal_bg = products::lair::terminal::BACKGROUND;  // #0F0F1A - Cave Dark
```

> **"Update colors in one place. Watch your entire ecosystem transform."**

---

## Features

<table>
<tr>
<td width="50%">

### 🎨 Complete Color System
- **Forge Brand** - Parent company palette
- **Molten Scale** - Warm orange (50-950)
- **Neutral Scale** - Gray scale for text/borders
- **Semantic Colors** - Success, warning, error, info

</td>
<td width="50%">

### 🏭 Product Themes
- **Lair** - Goblin Purple for terminals
- **Hearth** - Iron Blue for content
- **Alloy** - Molten Orange for design systems

</td>
</tr>
<tr>
<td width="50%">

### 📐 Spacing & Typography
- Consistent 4px-based spacing scale
- Font stacks (Geist Sans, Geist Mono)
- Type scale with semantic presets

</td>
<td width="50%">

### ⚡ Zero Runtime Cost
- All tokens are `const` 
- Compile-time color resolution
- No heap allocations for colors

</td>
</tr>
</table>

---

## Installation

```bash
cargo add molten_brand
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
molten_brand = "0.1"
```

---

## Quick Start

### Colors

```rust
use molten_brand::colors::{forge, molten, neutral, surface, text};

// Forge brand colors (shared across all products)
let bg = forge::BLACK;        // #0A0A0A
let accent = forge::MOLTEN;   // #F97316

// Full color scales
let orange_light = molten::SCALE_300;   // #FDBA74
let orange = molten::PRIMARY;            // #F97316
let orange_dark = molten::SCALE_700;    // #C2410C

// Surfaces for dark mode UIs
let card_bg = surface::RAISED;    // #18181B
let modal_bg = surface::OVERLAY;  // #27272A

// Text colors
let primary_text = text::PRIMARY;      // #FAFAFA
let secondary_text = text::SECONDARY;  // #A1A1AA
```

### Product Themes

Each Molten Labs product has its own visual identity:

```rust
use molten_brand::products::{lair, hearth, alloy};

// Lair - The Terminal for Goblins 👺
let goblin_purple = lair::PRIMARY;              // #7C3AED
let cave_dark = lair::terminal::BACKGROUND;     // #0F0F1A
let cursor = lair::terminal::CURSOR;            // #7C3AED
let glow = lair::goblin::GLOW;                  // Purple glow effect

// Hearth - Content Platform 📰
let editorial_blue = hearth::PRIMARY;           // #3B82F6
let card = hearth::content::CARD;               // #111111

// Alloy - Design System 🎨
let system_orange = alloy::PRIMARY;             // #F97316
```

### Semantic Colors

Colors that convey meaning—consistent across your entire app:

```rust
use molten_brand::semantic;

let success = semantic::SUCCESS;      // #10B981 - Green
let warning = semantic::WARNING;      // #F59E0B - Amber
let error = semantic::ERROR;          // #EF4444 - Red
let info = semantic::INFO;            // #3B82F6 - Blue

// Agent status colors (for AI/automation UIs)
let running = semantic::agent::RUNNING;    // Green
let thinking = semantic::agent::THINKING;  // Amber
let failed = semantic::agent::FAILED;      // Red
```

### Typography

```rust
use molten_brand::typography::{families, sizes, presets};

// Font families
let sans = families::SANS;       // Geist Sans + system fallbacks
let mono = families::MONO;       // Geist Mono + system fallbacks
let display = families::DISPLAY; // Space Grotesk for headlines

// Sizes
let body = sizes::BASE;    // 16px
let h1 = sizes::H1;        // 36px
let huge = sizes::DISPLAY; // 48px

// Presets (family + size + weight + line-height)
let heading_style = presets::H1;
let body_style = presets::BODY;
let code_style = presets::CODE;
```

### Spacing

```rust
use molten_brand::spacing::{scale, semantic, units};

// Scale values
let xs = scale::S2;    // 8px
let md = scale::S4;    // 16px
let xl = scale::S8;    // 32px

// Semantic aliases
let padding = semantic::COMPONENT_MD;  // 12px
let gap = semantic::GAP_MD;            // 16px
let section = semantic::SECTION;       // 32px

// Calculate from units (4px base)
let custom = units(5);  // 20px
```

---

## Brand Hierarchy

```
┌─────────────────────────────────────────────────────┐
│              MOLTEN LABS (Parent)                   │
│                                                     │
│   ┌─────────────┐ ┌─────────────┐ ┌─────────────┐  │
│   │    LAIR     │ │   HEARTH    │ │    ALLOY    │  │
│   │  Terminal   │ │   Content   │ │Design System│  │
│   │   Purple    │ │    Blue     │ │   Orange    │  │
│   │  #7C3AED    │ │  #3B82F6    │ │  #F97316    │  │
│   └─────────────┘ └─────────────┘ └─────────────┘  │
└─────────────────────────────────────────────────────┘
```

---

## Use with Sigil (Terminal Styling)

Combine with [sigil](https://github.com/moltenlabs/sigil) for beautiful terminal output:

```rust
use molten_brand::colors::molten;
use sigil::{style, Color};

// Convert brand colors to terminal colors
let styled = style("🔥 Molten Labs")
    .fg(Color::from_hex(&molten::PRIMARY.hex()))
    .bold()
    .to_string();

println!("{}", styled);
```

---

## Ecosystem

Molten Brand is part of the **Molten Labs** open source ecosystem:

| Crate | Description | Status |
|-------|-------------|--------|
| **[molten-brand](https://github.com/moltenlabs/molten-brand)** | Design tokens (you are here) | ✅ Released |
| **[sigil](https://github.com/moltenlabs/sigil)** | ANSI escape sequences | ✅ Released |
| **[lacquer](https://github.com/moltenlabs/lacquer)** | Terminal styling (like lipgloss) | 🚧 Coming Soon |
| **[cauldron](https://github.com/moltenlabs/cauldron)** | TUI framework (like bubbletea) | 📋 Planned |
| **[rune](https://github.com/moltenlabs/rune)** | Shell script tools (like gum) | 📋 Planned |
| **[ember](https://github.com/moltenlabs/ember)** | Markdown renderer (like glow) | 📋 Planned |

---

## Documentation

- 📖 [API Documentation](https://docs.rs/molten_brand)
- 🎨 [Brand Guidelines](https://github.com/moltenlabs/molten-brand/blob/main/docs/BRAND.md)
- 💡 [Examples](https://github.com/moltenlabs/molten-brand/tree/main/examples)

---

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

```bash
# Clone and run tests
git clone https://github.com/moltenlabs/molten-brand
cd molten-brand
cargo test
```

---

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

---

<p align="center">
  <sub>Built with 🔥 by <a href="https://github.com/moltenlabs">Molten Labs</a></sub>
</p>

<p align="center">
  <sub><i>"Let them cook."</i></sub>
</p>
