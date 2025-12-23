use egui::{Color32, CornerRadius, Id, Painter, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};

use crate::animation::{AnimationConfig, AnimationState};
use crate::elements::line::{LineElement, LineStyle, PointElement};
use crate::helpers::color::{lighten, ChartColor};
use crate::helpers::math::{compute_data_hash, nice_ticks};
use crate::theme::{ChartTheme, ThemePreset};
use crate::tooltip::{calculate_tooltip_position, draw_tooltip, measure_tooltip_size, TooltipContent};

/// Memory stored in egui context between frames
#[derive(Clone, Default)]
struct LineChartMemory {
    animation: AnimationState,
    data_hash: u64,
    hovered_index: Option<usize>,
}

/// Response returned after showing the chart
#[derive(Clone, Debug)]
pub struct LineChartResponse {
    /// The egui Response for the chart area
    pub response: Response,
    /// Index of currently hovered point
    pub hovered: Option<usize>,
    /// Index of clicked point (if any this frame)
    pub clicked: Option<usize>,
}

/// Line chart widget with Chart.js-inspired API
#[derive(Clone)]
pub struct LineChart {
    id: Option<Id>,
    data: Vec<f64>,
    labels: Vec<String>,
    color: ChartColor,
    animation: AnimationConfig,
    tooltip_enabled: bool,
    theme: ChartTheme,
    size: Option<Vec2>,
    min_size: Vec2,
    show_grid: bool,
    show_axes: bool,
    line_style: LineStyle,
}

impl Default for LineChart {
    fn default() -> Self {
        Self {
            id: None,
            data: Vec::new(),
            labels: Vec::new(),
            color: ChartColor::Rgba(Color32::from_rgb(54, 162, 235)),
            animation: AnimationConfig::default(),
            tooltip_enabled: true,
            theme: ChartTheme::default(),
            size: None,
            min_size: Vec2::new(100.0, 80.0),
            show_grid: true,
            show_axes: true,
            line_style: LineStyle::default(),
        }
    }
}

impl LineChart {
    /// Create a new line chart
    pub fn new() -> Self {
        Self::default()
    }

    /// Set unique ID for this chart instance
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

    /// Set line color
    pub fn color(mut self, color: impl Into<ChartColor>) -> Self {
        self.color = color.into();
        self
    }

    /// Set line width
    pub fn line_width(mut self, width: f32) -> Self {
        self.line_style.width = width;
        self
    }

    /// Set point radius
    pub fn point_radius(mut self, radius: f32) -> Self {
        self.line_style.point_radius = radius;
        self
    }

    /// Show/hide data points
    pub fn show_points(mut self, show: bool) -> Self {
        self.line_style.show_points = show;
        self
    }

    /// Enable area fill under line
    pub fn fill(mut self, enabled: bool) -> Self {
        self.line_style.fill = enabled;
        self
    }

    /// Set fill color (defaults to line color with transparency)
    pub fn fill_color(mut self, color: impl Into<ChartColor>) -> Self {
        self.line_style.fill_color = Some(color.into().to_color32());
        self
    }

    /// Use curved (bezier) or straight lines
    pub fn curved(mut self, curved: bool) -> Self {
        self.line_style.curved = curved;
        self
    }

    /// Set curve tension (0.0 = straight, 0.4 = default, 1.0 = very curved)
    pub fn tension(mut self, tension: f32) -> Self {
        self.line_style.tension = tension.clamp(0.0, 1.0);
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

    /// Set minimum size
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

    /// Show the chart and return response
    pub fn show(self, ui: &mut Ui) -> LineChartResponse {
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
        let id = self.id.unwrap_or_else(|| ui.make_persistent_id("line_chart"));

        // Load/update memory
        let mut memory = ui
            .ctx()
            .data_mut(|d| d.get_temp_mut_or_insert_with::<LineChartMemory>(id, Default::default).clone());

        // Check for data changes
        let new_data_hash = compute_data_hash(&self.data);
        if memory.data_hash != new_data_hash {
            memory.animation = AnimationState::new(self.animation.clone());
            memory.data_hash = new_data_hash;
        }

        // Get animation progress
        let progress = memory.animation.progress();
        memory.animation.request_repaint_if_animating(ui.ctx());

        // Calculate layout
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

        // Build line and points
        let (line, points) = self.build_line_elements(chart_rect);
        let base_y = chart_rect.max.y;

        // Draw grid
        if self.show_grid {
            self.draw_grid(&painter, chart_rect);
        }

        // Draw fill (before line)
        if self.line_style.fill {
            let fill_color = self.line_style.fill_color.unwrap_or_else(|| {
                let c = self.color.to_color32();
                Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), 50)
            });
            line.draw_fill_animated(&painter, base_y, progress, fill_color);
        }

        // Draw line
        line.draw_animated(&painter, base_y, progress);

        // Draw points
        if self.line_style.show_points {
            for (i, point) in points.iter().enumerate() {
                let mut point = point.clone();

                // Hover effect
                if memory.hovered_index == Some(i) {
                    point.radius *= 1.3;
                    point.fill_color = lighten(point.fill_color, 0.2);
                }

                point.draw_animated(&painter, base_y, progress);
            }
        }

        // Draw axes
        if self.show_axes {
            self.draw_axes(&painter, chart_rect);
        }

        // Draw labels
        self.draw_labels(&painter, chart_rect, &points);

        // Handle interaction - check point hover
        let mut hovered_index = None;
        let mut clicked_index = None;

        if let Some(hover_pos) = response.hover_pos() {
            for (i, point) in points.iter().enumerate() {
                // Check animated position
                let animated_y = base_y + (point.y - base_y) * progress;
                let animated_point = PointElement {
                    y: animated_y,
                    ..point.clone()
                };
                if animated_point.contains(hover_pos) {
                    hovered_index = Some(i);
                    break;
                }
            }
        }

        if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                for (i, point) in points.iter().enumerate() {
                    let animated_y = base_y + (point.y - base_y) * progress;
                    let animated_point = PointElement {
                        y: animated_y,
                        ..point.clone()
                    };
                    if animated_point.contains(pos) {
                        clicked_index = Some(i);
                        break;
                    }
                }
            }
        }

        memory.hovered_index = hovered_index;

        // Draw tooltip
        if self.tooltip_enabled {
            if let Some(idx) = memory.hovered_index {
                if idx < self.data.len() {
                    let point = &points[idx];
                    let animated_y = base_y + (point.y - base_y) * progress;

                    let content = TooltipContent {
                        title: None,
                        label: self
                            .labels
                            .get(idx)
                            .cloned()
                            .unwrap_or_else(|| format!("Point {}", idx + 1)),
                        value: format_value(self.data[idx]),
                        color: point.fill_color,
                    };

                    let tooltip_size = measure_tooltip_size(&painter, &content, &self.theme.tooltip);
                    let anchor = Pos2::new(point.x, animated_y);
                    let tooltip_pos = calculate_tooltip_position(anchor, tooltip_size, rect);

                    draw_tooltip(&painter, &content, tooltip_pos, &self.theme.tooltip);
                }
            }
        }

        // Store memory
        ui.ctx().data_mut(|d| d.insert_temp(id, memory.clone()));

        LineChartResponse {
            response,
            hovered: memory.hovered_index,
            clicked: clicked_index,
        }
    }

    /// Build line and point elements
    fn build_line_elements(&self, chart_rect: Rect) -> (LineElement, Vec<PointElement>) {
        if self.data.is_empty() {
            return (LineElement::new(vec![]), vec![]);
        }

        let line_color = self.color.to_color32();
        let n = self.data.len();

        // Calculate scales
        let max_val = self.data.iter().cloned().fold(f64::NEG_INFINITY, f64::max).max(0.0) * 1.1;
        let min_val = self.data.iter().cloned().fold(f64::INFINITY, f64::min).min(0.0);

        let y_range = max_val - min_val;
        let y_scale = if y_range > 0.0 {
            chart_rect.height() as f64 / y_range
        } else {
            1.0
        };

        let x_step = chart_rect.width() / (n - 1).max(1) as f32;

        // Build points
        let points: Vec<PointElement> = self
            .data
            .iter()
            .enumerate()
            .map(|(i, &val)| {
                let x = chart_rect.min.x + i as f32 * x_step;
                let y = chart_rect.max.y - ((val - min_val) * y_scale) as f32;

                let mut point = PointElement::new(x, y);
                point.fill_color = line_color;
                point.radius = self.line_style.point_radius;
                point.border_width = self.line_style.point_border_width;
                point.border_color = self.line_style.point_border_color;
                point
            })
            .collect();

        // Build line
        let mut line = LineElement::new(points.clone());
        line.color = line_color;
        line.width = self.line_style.width;
        line.curved = self.line_style.curved;
        line.tension = self.line_style.tension;

        (line, points)
    }

    /// Draw grid lines
    fn draw_grid(&self, painter: &Painter, chart_rect: Rect) {
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

        painter.line_segment([chart_rect.left_bottom(), chart_rect.left_top()], stroke);
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
                painter.text(
                    Pos2::new(chart_rect.min.x - 8.0, y),
                    egui::Align2::RIGHT_CENTER,
                    format_axis_value(*tick),
                    egui::FontId::proportional(11.0),
                    self.theme.text_color,
                );
            }
        }
    }

    /// Draw labels
    fn draw_labels(&self, painter: &Painter, chart_rect: Rect, points: &[PointElement]) {
        for (i, point) in points.iter().enumerate() {
            let label = self.labels.get(i).cloned().unwrap_or_else(|| format!("{}", i + 1));

            painter.text(
                Pos2::new(point.x, chart_rect.max.y + 12.0),
                egui::Align2::CENTER_TOP,
                label,
                egui::FontId::proportional(11.0),
                self.theme.text_color,
            );
        }
    }
}

impl Widget for LineChart {
    fn ui(self, ui: &mut Ui) -> Response {
        self.show(ui).response
    }
}

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
