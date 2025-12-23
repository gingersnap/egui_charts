# CLAUDE.md

## Project Overview

`egui_charts` is a Rust chart library for egui, porting Chart.js's bar chart implementation with animations, tooltips, and interactivity.

## Build & Test Commands

```bash
# Check compilation
cargo check

# Run tests
cargo test

# Run the demo
cargo run --example demo

# Build release
cargo build --release
```

## Architecture

```
src/
├── lib.rs              # Public API exports
├── bar_chart.rs        # Main BarChart widget + builder pattern
├── animation.rs        # Easing functions (easeOutQuart, bounce, elastic)
├── interaction.rs      # Hit detection for hover/click
├── tooltip.rs          # Tooltip positioning + rendering
├── theme.rs            # Light/Dark/Minimal theme presets
├── helpers/
│   ├── color.rs        # Hex color parsing, lighten/darken
│   └── math.rs         # nice_ticks for axes, data hashing
└── elements/
    └── bar.rs          # Bar geometry, animated drawing
```

## Key Patterns

- **Builder API**: `BarChart::new().data(...).labels(...).show(ui)`
- **State Storage**: Uses `ui.ctx().data_mut()` with Id-based storage for animation state
- **Animation**: Automatic re-animation on data change (detected via hash comparison)
- **Immediate Mode**: Follows egui's immediate mode paradigm - redraws every frame during animation

## Adding New Features

1. **New easing function**: Add variant to `Easing` enum in `animation.rs`, implement formula in `apply()`
2. **New theme**: Add variant to `ThemePreset` in `theme.rs`, implement in `to_theme()`
3. **New chart type**: Create new file (e.g., `line_chart.rs`), follow `BarChart` structure

## Dependencies

- `egui = "0.31"` - Core UI library
- `eframe` (dev) - For running examples
