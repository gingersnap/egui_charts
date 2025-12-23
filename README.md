# egui_charts

Chart.js-inspired animated charts for [egui](https://github.com/emilk/egui).

![Demo](https://img.shields.io/badge/egui-0.31-blue)
![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-green)

## Features

- **Smooth animations** - Bars animate with Chart.js-style easing (easeOutQuart, bounce, elastic)
- **Interactive tooltips** - Hover to see values with automatic positioning
- **Themeable** - Light, Dark, and Minimal themes included
- **Responsive** - Charts resize to fill available space
- **Click handling** - Detect which bar was clicked

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
egui_charts = "0.1"
```

## Quick Start

```rust
use egui_charts::prelude::*;

// In your egui update function:
BarChart::new()
    .data(vec![65.0, 59.0, 80.0, 81.0, 56.0])
    .labels(vec!["Mon", "Tue", "Wed", "Thu", "Fri"])
    .colors(vec!["#36a2eb", "#ff6384", "#ffce56", "#4bc0c0", "#9966ff"])
    .animate(Animation::ease_out_quart(0.8))
    .tooltip(true)
    .show(ui);
```

## API

### BarChart Builder

| Method | Description |
|--------|-------------|
| `.data(vec![...])` | Set bar values |
| `.labels(vec![...])` | Set category labels |
| `.colors(vec![...])` | Set bar colors (hex strings or `Color32`) |
| `.animate(config)` | Configure animation |
| `.tooltip(bool)` | Enable/disable tooltips |
| `.theme_preset(preset)` | Use Light, Dark, or Minimal theme |
| `.size([w, h])` | Set fixed size |
| `.grid(bool)` | Show/hide grid lines |
| `.show(ui)` | Render and return `BarChartResponse` |

### Animation Options

```rust
// Default Chart.js easing (recommended)
Animation::ease_out_quart(0.8)  // 800ms

// Other options
Animation::linear(0.5)
Animation::bounce(1.0)
Animation::elastic(1.2)
Animation::none()  // Instant, no animation
```

### Themes

```rust
.theme_preset(ThemePreset::Light)   // White background, dark text
.theme_preset(ThemePreset::Dark)    // Dark background, light text
.theme_preset(ThemePreset::Minimal) // Transparent, subtle styling
```

### Handling Interactions

```rust
let response = BarChart::new()
    .data(data)
    .show(ui);

if let Some(clicked_index) = response.clicked {
    println!("Clicked bar {}", clicked_index);
}

if let Some(hovered_index) = response.hovered {
    // Bar is being hovered
}
```

## Example

Run the interactive demo:

```bash
cargo run --example demo
```

## Chart.js Compatibility

This library ports key concepts from [Chart.js](https://www.chartjs.org/):

- **Easing functions** from `helpers.easing.js`
- **Bar geometry** from `elements/element.bar.js`
- **Hit detection** from `core.interaction.js`
- **Tooltip positioning** from `core.tooltip.js`
- **Default color palette** matching Chart.js defaults

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
