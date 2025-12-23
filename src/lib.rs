//! # egui_charts
//!
//! Chart.js-inspired animated charts for egui.
//!
//! This library provides beautiful, animated charts for egui applications,
//! with an API inspired by Chart.js.
//!
//! ## Chart Types
//!
//! - **BarChart** - Vertical bar charts
//! - **LineChart** - Line charts with optional area fill
//! - **PieChart** - Pie and donut charts
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use egui_charts::prelude::*;
//!
//! fn show_charts(ui: &mut egui::Ui) {
//!     // Bar chart
//!     BarChart::new()
//!         .data(vec![18.0, 40.0, 30.0, 25.0, 50.0])
//!         .labels(vec!["Mon", "Tue", "Wed", "Thu", "Fri"])
//!         .animate(Animation::ease_out_quart(0.8))
//!         .show(ui);
//!
//!     // Line chart
//!     LineChart::new()
//!         .data(vec![18.0, 40.0, 30.0, 25.0, 50.0])
//!         .labels(vec!["Mon", "Tue", "Wed", "Thu", "Fri"])
//!         .fill(true)
//!         .show(ui);
//!
//!     // Pie chart
//!     PieChart::new()
//!         .data(vec![30.0, 25.0, 20.0, 15.0, 10.0])
//!         .labels(vec!["Chrome", "Safari", "Firefox", "Edge", "Other"])
//!         .donut(0.5)
//!         .show(ui);
//! }
//! ```
//!
//! ## Features
//!
//! - **Smooth animations**: Charts animate with Chart.js-style easing
//! - **Interactive tooltips**: Hover to see values
//! - **Themeable**: Light, dark, and minimal themes included
//! - **Responsive**: Charts resize to fill available space
//! - **Click handling**: Detect which element was clicked
//!
//! ## Animation
//!
//! Charts animate automatically when data changes:
//!
//! ```rust,ignore
//! use egui_charts::prelude::*;
//!
//! // Default animation (easeOutQuart, 800ms)
//! BarChart::new().data(vec![1.0, 2.0, 3.0]).show(ui);
//!
//! // Custom animation
//! LineChart::new()
//!     .data(vec![1.0, 2.0, 3.0])
//!     .animate(Animation::bounce(1.2))
//!     .show(ui);
//!
//! // No animation
//! PieChart::new()
//!     .data(vec![1.0, 2.0, 3.0])
//!     .animate(Animation::none())
//!     .show(ui);
//! ```

mod animation;
mod bar_chart;
mod line_chart;
mod pie_chart;
mod interaction;
mod theme;
mod tooltip;

pub mod elements;
pub mod helpers;

// Re-exports
pub use animation::{Animation, AnimationConfig, AnimationState, Easing};
pub use bar_chart::{BarChart, BarChartResponse};
pub use line_chart::{LineChart, LineChartResponse};
pub use pie_chart::{PieChart, PieChartResponse};
pub use elements::{BarElement, BarStyle, LineElement, LineStyle, PointElement, ArcElement, PieStyle};
pub use interaction::{InteractionMode, InteractionResult};
pub use theme::{ChartTheme, ThemePreset};
pub use tooltip::{TooltipConfig, TooltipContent};

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::{
        Animation, AnimationConfig, Easing,
        BarChart, BarChartResponse, BarStyle,
        LineChart, LineChartResponse, LineStyle,
        PieChart, PieChartResponse, PieStyle,
        ChartTheme, ThemePreset, TooltipConfig,
    };
    pub use crate::helpers::color::ChartColor;
}
