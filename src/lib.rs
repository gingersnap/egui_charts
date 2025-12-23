//! # egui_charts
//!
//! Chart.js-inspired animated charts for egui.
//!
//! This library provides beautiful, animated bar charts for egui applications,
//! with an API inspired by Chart.js.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use egui_charts::prelude::*;
//!
//! // In your egui app's update function:
//! fn show_chart(ui: &mut egui::Ui) {
//!     BarChart::new()
//!         .data(vec![18.0, 40.0, 30.0, 25.0, 50.0])
//!         .labels(vec!["Mon", "Tue", "Wed", "Thu", "Fri"])
//!         .colors(vec!["#36a2eb", "#ff6384", "#ffce56", "#4bc0c0", "#9966ff"])
//!         .animate(Animation::ease_out_quart(0.8))
//!         .tooltip(true)
//!         .show(ui);
//! }
//! ```
//!
//! ## Features
//!
//! - **Smooth animations**: Bars animate from zero with Chart.js-style easing
//! - **Interactive tooltips**: Hover over bars to see values
//! - **Themeable**: Light, dark, and minimal themes included
//! - **Responsive**: Charts resize to fill available space
//! - **Click handling**: Detect which bar was clicked
//!
//! ## Animation
//!
//! Charts animate automatically when data changes:
//!
//! ```rust,ignore
//! use egui_charts::prelude::*;
//!
//! // Default animation (easeOutQuart, 800ms)
//! BarChart::new()
//!     .data(vec![1.0, 2.0, 3.0])
//!     .show(ui);
//!
//! // Custom animation
//! BarChart::new()
//!     .data(vec![1.0, 2.0, 3.0])
//!     .animate(Animation::bounce(1.2))
//!     .show(ui);
//!
//! // No animation
//! BarChart::new()
//!     .data(vec![1.0, 2.0, 3.0])
//!     .animate(Animation::none())
//!     .show(ui);
//! ```
//!
//! ## Theming
//!
//! ```rust,ignore
//! use egui_charts::prelude::*;
//!
//! // Use a preset theme
//! BarChart::new()
//!     .data(vec![1.0, 2.0, 3.0])
//!     .theme_preset(ThemePreset::Dark)
//!     .show(ui);
//! ```

mod animation;
mod bar_chart;
mod interaction;
mod theme;
mod tooltip;

pub mod elements;
pub mod helpers;

// Re-exports
pub use animation::{Animation, AnimationConfig, AnimationState, Easing};
pub use bar_chart::{BarChart, BarChartResponse};
pub use elements::{BarElement, BarStyle};
pub use interaction::{InteractionMode, InteractionResult};
pub use theme::{ChartTheme, ThemePreset};
pub use tooltip::{TooltipConfig, TooltipContent};

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::{
        Animation, AnimationConfig, BarChart, BarChartResponse, BarStyle, ChartTheme, Easing, ThemePreset,
        TooltipConfig,
    };
    pub use crate::helpers::color::ChartColor;
}
