use egui::{Color32, CornerRadius, Id, Painter, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};

use crate::animation::{AnimationConfig, AnimationState};
use crate::elements::{BarElement, BarStyle};
use crate::helpers::color::{lighten, ChartColor};
use crate::helpers::math::{compute_data_hash, nice_ticks};
use crate::interaction::evaluate_interaction;
use crate::theme::{ChartTheme, ThemePreset};
use crate::tooltip::{calculate_tooltip_position, draw_tooltip, measure_tooltip_size, TooltipContent};

/// Memory stored in egui context between frames
#[derive(Clone, Default)]
struct BarChartMemory {
    animation: AnimationState,
    data_hash: u64,
    hovered_index: Option<usize>,
}

/// Response returned after showing the chart
#[derive(Clone, Debug)]
pub struct BarChartResponse {
    /// The egui Response for the chart area
    pub response: Response,
    /// Index of currently hovered bar
    pub hovered: Option<usize>,
    /// Index of clicked bar (if any this frame)
    pub clicked: Option<usize>,
}

/// Bar chart widget with Chart.js-inspired API
#[derive(Clone)]
pub struct BarChart {
    id: Option<Id>,
    data: Vec<f64>,
    labels: Vec<String>,
    colors: Vec<ChartColor>,
    animation: AnimationConfig,
    tooltip_enabled: bool,
    theme: ChartTheme,
    size: Option<Vec2>,
    min_size: Vec2,
    show_grid: bool,
    show_axes: bool,
    bar_style: Option<BarStyle>,
}

impl Default for BarChart {
    fn default() -> Self {
        Self {
            id: None,
            data: Vec::new(),
            labels: Vec::new(),
            colors: Vec::new(),
            animation: AnimationConfig::default(),
            tooltip_enabled: true,
            theme: ChartTheme::default(),
            size: None,
            min_size: Vec2::new(100.0, 80.0),
            show_grid: true,
            show_axes: true,
            bar_style: None,
        }
    }
}

impl BarChart {
    /// Create a new bar chart
    pub fn new() -> Self {
        Self::default()
    }

    /// Set unique ID for this chart instance
    /// Required if multiple charts exist with same data
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set chart data values
    pub fn data(mut self, data: impl IntoIterator<Item = impl Into<f64>>) -> Self {
        self.data = data.into_iter().map(|v| v.into()).collect();
        self
    }

    /// Set category labels
    pub fn labels(mut self, labels: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.labels = labels.into_iter().map(|s| s.into()).collect();
        self
    }

    /// Set bar colors
    pub fn colors(mut self, colors: impl IntoIterator<Item = impl Into<ChartColor>>) -> Self {
        self.colors = colors.into_iter().map(|c| c.into()).collect();
        self
    }

    /// Configure animation
    pub fn animate(mut self, config: AnimationConfig) -> Self {
        self.animation = config;
        self
    }

    /// Enable/disable tooltips
    pub fn tooltip(mut self, enabled: bool) -> Self {
        self.tooltip_enabled = enabled;
        self
    }

    /// Set theme
    pub fn theme(mut self, theme: impl Into<ChartTheme>) -> Self {
        self.theme = theme.into();
        self
    }

    /// Use theme preset
    pub fn theme_preset(mut self, preset: ThemePreset) -> Self {
        self.theme = preset.to_theme();
        self
    }

    /// Set fixed size
    pub fn size(mut self, size: impl Into<Vec2>) -> Self {
        self.size = Some(size.into());
        self
    }

    /// Set minimum size (used when auto-sizing)
    pub fn min_size(mut self, min_size: impl Into<Vec2>) -> Self {
        self.min_size = min_size.into();
        self
    }

    /// Show/hide grid lines
    pub fn grid(mut self, show: bool) -> Self {
        self.show_grid = show;
        self
    }

    /// Show/hide axes
    pub fn axes(mut self, show: bool) -> Self {
        self.show_axes = show;
        self
    }

    /// Set bar styling
    pub fn bar_style(mut self, style: BarStyle) -> Self {
        self.bar_style = Some(style);
        self
    }

    /// Set bar width percentage (0.0 to 1.0)
    pub fn bar_width(mut self, percentage: f32) -> Self {
        let style = self.bar_style.get_or_insert_with(BarStyle::default);
        style.bar_percentage = percentage.clamp(0.1, 1.0);
        self
    }

    /// Set bar corner radius
    pub fn border_radius(mut self, radius: u8) -> Self {
        let style = self.bar_style.get_or_insert_with(BarStyle::default);
        style.border_radius = CornerRadius::same(radius);
        self
    }

    /// Set bar border width
    pub fn border_width(mut self, width: f32) -> Self {
        let style = self.bar_style.get_or_insert_with(BarStyle::default);
        style.border_width = width;
        self
    }

    /// Set bar border color
    pub fn border_color(mut self, color: impl Into<ChartColor>) -> Self {
        let style = self.bar_style.get_or_insert_with(BarStyle::default);
        style.border_color = color.into().to_color32();
        self
    }

    /// Show the chart and return response
    pub fn show(self, ui: &mut Ui) -> BarChartResponse {
        // Determine size
        let size = self.size.unwrap_or_else(|| {
            let available = ui.available_size();
            Vec2::new(
                available.x.max(self.min_size.x),
                available.y.min(300.0).max(self.min_size.y),
            )
        });

        // Allocate space and get response
        let (response, painter) = ui.allocate_painter(size, Sense::click_and_drag());
        let rect = response.rect;

        // Generate unique ID for state storage
        let id = self.id.unwrap_or_else(|| ui.make_persistent_id("bar_chart"));

        // Load/update memory from egui context
        let mut memory = ui
            .ctx()
            .data_mut(|d| d.get_temp_mut_or_insert_with::<BarChartMemory>(id, Default::default).clone());

        // Check for data changes
        let new_data_hash = compute_data_hash(&self.data);
        if memory.data_hash != new_data_hash {
            memory.animation = AnimationState::new(self.animation.clone());
            memory.data_hash = new_data_hash;
        }

        // Get animation progress
        let progress = memory.animation.progress();

        // Request repaint if still animating
        memory.animation.request_repaint_if_animating(ui.ctx());

        // Calculate layout regions
        let y_axis_width = 45.0;
        let x_axis_height = 30.0;
        let top_padding = 15.0;
        let right_padding = 15.0;

        let chart_rect = Rect::from_min_max(
            Pos2::new(rect.min.x + y_axis_width, rect.min.y + top_padding),
            Pos2::new(rect.max.x - right_padding, rect.max.y - x_axis_height),
        );

        // Draw background
        if self.theme.background_color != Color32::TRANSPARENT {
            painter.rect_filled(rect, CornerRadius::ZERO, self.theme.background_color);
        }

        // Build bar elements
        let bars = self.build_bar_elements(chart_rect);

        // Draw grid
        if self.show_grid {
            self.draw_grid(&painter, chart_rect);
        }

        // Draw bars with animation
        for (i, bar) in bars.iter().enumerate() {
            let mut bar = bar.clone();

            // Apply hover effect
            if memory.hovered_index == Some(i) {
                bar.fill_color = lighten(bar.fill_color, 0.15);
            }

            bar.draw(&painter, progress);
        }

        // Draw axes (on top of bars)
        if self.show_axes {
            self.draw_axes(&painter, chart_rect);
        }

        // Draw labels
        self.draw_labels(&painter, chart_rect, &bars);

        // Handle interaction
        let interaction = evaluate_interaction(&bars, &response);
        memory.hovered_index = interaction.hovered_index;

        // Draw tooltip if hovering
        if self.tooltip_enabled {
            if let Some(idx) = memory.hovered_index {
                if idx < self.data.len() {
                    let bar = &bars[idx];
                    let content = TooltipContent {
                        title: None,
                        label: self
                            .labels
                            .get(idx)
                            .cloned()
                            .unwrap_or_else(|| format!("Item {}", idx + 1)),
                        value: format_value(self.data[idx]),
                        color: bar.fill_color,
                    };

                    let tooltip_size = measure_tooltip_size(&painter, &content, &self.theme.tooltip);
                    let anchor = Pos2::new(bar.x, bar.y.min(bar.base));
                    let tooltip_pos = calculate_tooltip_position(anchor, tooltip_size, rect);

                    draw_tooltip(&painter, &content, tooltip_pos, &self.theme.tooltip);
                }
            }
        }

        // Store updated memory
        ui.ctx().data_mut(|d| {
            d.insert_temp(id, memory.clone());
        });

        BarChartResponse {
            response,
            hovered: memory.hovered_index,
            clicked: interaction.clicked_index,
        }
    }

    /// Build bar elements from data
    fn build_bar_elements(&self, chart_rect: Rect) -> Vec<BarElement> {
        if self.data.is_empty() {
            return Vec::new();
        }

        let style = self.bar_style.clone().unwrap_or(self.theme.bar_style.clone());
        let colors: Vec<Color32> = if self.colors.is_empty() {
            style.fill_colors.clone()
        } else {
            self.colors.iter().map(|c| c.to_color32()).collect()
        };

        let n = self.data.len();
        let total_width = chart_rect.width();
        let category_width = total_width / n as f32 * style.category_percentage;
        let bar_width = category_width * style.bar_percentage;

        // Calculate y scale
        let max_val = self
            .data
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max)
            .max(0.0);
        let min_val = self
            .data
            .iter()
            .cloned()
            .fold(f64::INFINITY, f64::min)
            .min(0.0);

        // Add some padding to max value for visual breathing room
        let padded_max = if max_val > 0.0 { max_val * 1.1 } else { max_val };

        let y_range = padded_max - min_val;
        let y_scale = if y_range > 0.0 {
            chart_rect.height() as f64 / y_range
        } else {
            1.0
        };

        let baseline_y = chart_rect.max.y - (-min_val * y_scale) as f32;

        self.data
            .iter()
            .enumerate()
            .map(|(i, &val)| {
                let x = chart_rect.min.x + (i as f32 + 0.5) * total_width / n as f32;
                let height = (val * y_scale) as f32;
                let y = baseline_y - height;

                let color = colors.get(i % colors.len()).cloned().unwrap_or(Color32::GRAY);

                let mut bar = BarElement::new(x, y, baseline_y, bar_width);
                bar.fill_color = color;
                bar.border_radius = style.border_radius;
                bar.border_width = style.border_width;
                bar.border_color = style.border_color;
                bar
            })
            .collect()
    }

    /// Draw grid lines
    fn draw_grid(&self, painter: &Painter, chart_rect: Rect) {
        // Calculate nice tick values
        let max_val = self.data.iter().cloned().fold(0.0_f64, f64::max) * 1.1;
        let ticks = nice_ticks(0.0, max_val, 5);

        let y_scale = if max_val > 0.0 {
            chart_rect.height() as f64 / max_val
        } else {
            1.0
        };

        for tick in &ticks {
            let y = chart_rect.max.y - (*tick * y_scale) as f32;
            if y >= chart_rect.min.y && y <= chart_rect.max.y {
                painter.line_segment(
                    [Pos2::new(chart_rect.min.x, y), Pos2::new(chart_rect.max.x, y)],
                    Stroke::new(1.0, self.theme.grid_color),
                );
            }
        }
    }

    /// Draw axes
    fn draw_axes(&self, painter: &Painter, chart_rect: Rect) {
        let stroke = Stroke::new(1.0, self.theme.axis_color);

        // Y axis
        painter.line_segment([chart_rect.left_bottom(), chart_rect.left_top()], stroke);

        // X axis
        painter.line_segment([chart_rect.left_bottom(), chart_rect.right_bottom()], stroke);

        // Y axis labels
        let max_val = self.data.iter().cloned().fold(0.0_f64, f64::max) * 1.1;
        let ticks = nice_ticks(0.0, max_val, 5);

        let y_scale = if max_val > 0.0 {
            chart_rect.height() as f64 / max_val
        } else {
            1.0
        };

        for tick in &ticks {
            let y = chart_rect.max.y - (*tick * y_scale) as f32;
            if y >= chart_rect.min.y && y <= chart_rect.max.y {
                let text = format_axis_value(*tick);

                painter.text(
                    Pos2::new(chart_rect.min.x - 8.0, y),
                    egui::Align2::RIGHT_CENTER,
                    text,
                    egui::FontId::proportional(11.0),
                    self.theme.text_color,
                );
            }
        }
    }

    /// Draw category labels
    fn draw_labels(&self, painter: &Painter, chart_rect: Rect, bars: &[BarElement]) {
        for (i, bar) in bars.iter().enumerate() {
            let label = self.labels.get(i).cloned().unwrap_or_else(|| format!("{}", i + 1));

            painter.text(
                Pos2::new(bar.x, chart_rect.max.y + 12.0),
                egui::Align2::CENTER_TOP,
                label,
                egui::FontId::proportional(11.0),
                self.theme.text_color,
            );
        }
    }
}

impl Widget for BarChart {
    fn ui(self, ui: &mut Ui) -> Response {
        self.show(ui).response
    }
}

/// Format a value for display in tooltip
fn format_value(value: f64) -> String {
    if value.abs() >= 1_000_000.0 {
        format!("{:.1}M", value / 1_000_000.0)
    } else if value.abs() >= 1_000.0 {
        format!("{:.1}K", value / 1_000.0)
    } else if value.fract().abs() < 0.001 {
        format!("{:.0}", value)
    } else {
        format!("{:.1}", value)
    }
}

/// Format a value for axis labels
fn format_axis_value(value: f64) -> String {
    if value.abs() >= 1_000_000.0 {
        format!("{:.0}M", value / 1_000_000.0)
    } else if value.abs() >= 1_000.0 {
        format!("{:.0}K", value / 1_000.0)
    } else if value.fract().abs() < 0.001 {
        format!("{:.0}", value)
    } else {
        format!("{:.1}", value)
    }
}
