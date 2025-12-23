use egui::Color32;

use crate::elements::BarStyle;
use crate::tooltip::TooltipConfig;

/// Complete chart theme
#[derive(Clone, Debug)]
pub struct ChartTheme {
    pub background_color: Color32,
    pub grid_color: Color32,
    pub axis_color: Color32,
    pub text_color: Color32,
    pub bar_style: BarStyle,
    pub tooltip: TooltipConfig,
}

impl Default for ChartTheme {
    fn default() -> Self {
        ThemePreset::Light.to_theme()
    }
}

/// Preset themes matching common Chart.js configurations
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ThemePreset {
    #[default]
    Light,
    Dark,
    Minimal,
}

impl ThemePreset {
    pub fn to_theme(&self) -> ChartTheme {
        match self {
            ThemePreset::Light => ChartTheme {
                background_color: Color32::WHITE,
                grid_color: Color32::from_gray(230),
                axis_color: Color32::from_gray(100),
                text_color: Color32::from_gray(60),
                bar_style: BarStyle::default(),
                tooltip: TooltipConfig::default(),
            },
            ThemePreset::Dark => ChartTheme {
                background_color: Color32::from_gray(30),
                grid_color: Color32::from_gray(60),
                axis_color: Color32::from_gray(150),
                text_color: Color32::from_gray(200),
                bar_style: BarStyle {
                    fill_colors: vec![
                        Color32::from_rgb(99, 179, 237),   // Lighter blue
                        Color32::from_rgb(255, 107, 129), // Lighter red
                        Color32::from_rgb(255, 217, 102), // Lighter yellow
                        Color32::from_rgb(100, 210, 210), // Lighter teal
                        Color32::from_rgb(170, 130, 255), // Lighter purple
                    ],
                    ..Default::default()
                },
                tooltip: TooltipConfig {
                    background_color: Color32::from_rgba_unmultiplied(50, 50, 50, 240),
                    border_color: Color32::from_gray(80),
                    ..Default::default()
                },
            },
            ThemePreset::Minimal => ChartTheme {
                background_color: Color32::TRANSPARENT,
                grid_color: Color32::TRANSPARENT,
                axis_color: Color32::from_gray(180),
                text_color: Color32::from_gray(100),
                bar_style: BarStyle {
                    border_width: 1.0,
                    border_color: Color32::from_gray(200),
                    ..Default::default()
                },
                tooltip: TooltipConfig::default(),
            },
        }
    }
}

impl From<ThemePreset> for ChartTheme {
    fn from(preset: ThemePreset) -> Self {
        preset.to_theme()
    }
}
