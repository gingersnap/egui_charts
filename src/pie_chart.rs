use egui::{Color32, CornerRadius, Id, Pos2, Response, Sense, Ui, Vec2, Widget};
use std::f32::consts::PI;

use crate::animation::{AnimationConfig, AnimationState};
use crate::elements::arc::{ArcElement, PieStyle};
use crate::helpers::color::{lighten, ChartColor};
use crate::helpers::math::compute_data_hash;
use crate::theme::{ChartTheme, ThemePreset};
use crate::tooltip::{calculate_tooltip_position, draw_tooltip, measure_tooltip_size, TooltipContent};

/// Memory stored in egui context between frames
#[derive(Clone, Default)]
struct PieChartMemory {
    animation: AnimationState,
    data_hash: u64,
    hovered_index: Option<usize>,
}

/// Response returned after showing the chart
#[derive(Clone, Debug)]
pub struct PieChartResponse {
    /// The egui Response for the chart area
    pub response: Response,
    /// Index of currently hovered segment
    pub hovered: Option<usize>,
    /// Index of clicked segment (if any this frame)
    pub clicked: Option<usize>,
}

/// Pie/Donut chart widget with Chart.js-inspired API
#[derive(Clone)]
pub struct PieChart {
    id: Option<Id>,
    data: Vec<f64>,
    labels: Vec<String>,
    colors: Vec<ChartColor>,
    animation: AnimationConfig,
    tooltip_enabled: bool,
    theme: ChartTheme,
    size: Option<Vec2>,
    min_size: Vec2,
    pie_style: PieStyle,
    show_labels: bool,
    show_percentages: bool,
}

impl Default for PieChart {
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
            min_size: Vec2::new(100.0, 100.0),
            pie_style: PieStyle::default(),
            show_labels: false,
            show_percentages: false,
        }
    }
}

impl PieChart {
    /// Create a new pie chart
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

    /// Set segment labels
    pub fn labels(mut self, labels: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.labels = labels.into_iter().map(|s| s.into()).collect();
        self
    }

    /// Set segment colors
    pub fn colors(mut self, colors: impl IntoIterator<Item = impl Into<ChartColor>>) -> Self {
        self.colors = colors.into_iter().map(|c| c.into()).collect();
        self
    }

    /// Set donut hole ratio (0.0 = pie, 0.5 = half radius hole)
    pub fn donut(mut self, ratio: f32) -> Self {
        self.pie_style.donut_ratio = ratio.clamp(0.0, 0.9);
        self
    }

    /// Set border width between segments
    pub fn border_width(mut self, width: f32) -> Self {
        self.pie_style.border_width = width;
        self
    }

    /// Set border color
    pub fn border_color(mut self, color: impl Into<ChartColor>) -> Self {
        self.pie_style.border_color = color.into().to_color32();
        self
    }

    /// Show labels outside segments
    pub fn show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    /// Show percentages on segments
    pub fn show_percentages(mut self, show: bool) -> Self {
        self.show_percentages = show;
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

    /// Show the chart and return response
    pub fn show(self, ui: &mut Ui) -> PieChartResponse {
        // Determine size (square for pie chart)
        let size = self.size.unwrap_or_else(|| {
            let available = ui.available_size();
            let s = available.x.min(available.y).min(300.0).max(self.min_size.x);
            Vec2::new(s, s)
        });

        // Allocate space
        let (response, painter) = ui.allocate_painter(size, Sense::click_and_drag());
        let rect = response.rect;

        // Generate unique ID
        let id = self.id.unwrap_or_else(|| ui.make_persistent_id("pie_chart"));

        // Load memory
        let mut memory = ui
            .ctx()
            .data_mut(|d| d.get_temp_mut_or_insert_with::<PieChartMemory>(id, Default::default).clone());

        // Check for data changes
        let new_data_hash = compute_data_hash(&self.data);
        if memory.data_hash != new_data_hash {
            memory.animation = AnimationState::new(self.animation.clone());
            memory.data_hash = new_data_hash;
        }

        let progress = memory.animation.progress();
        memory.animation.request_repaint_if_animating(ui.ctx());

        // Draw background
        if self.theme.background_color != Color32::TRANSPARENT {
            painter.rect_filled(rect, CornerRadius::ZERO, self.theme.background_color);
        }

        // Calculate pie geometry
        let center = rect.center();
        // Use more padding when labels are shown outside
        let padding = if self.show_labels || self.show_percentages { 60.0 } else { 20.0 };
        let outer_radius = (rect.width().min(rect.height()) / 2.0 - padding).max(10.0);
        let inner_radius = outer_radius * self.pie_style.donut_ratio;

        // Build arc elements
        let arcs = self.build_arc_elements(center, inner_radius, outer_radius);

        // Draw arcs
        for (i, arc) in arcs.iter().enumerate() {
            let mut arc = arc.clone();

            // Hover effect - slightly expand
            if memory.hovered_index == Some(i) {
                arc.fill_color = lighten(arc.fill_color, 0.15);
                // Expand outward slightly
                let expand = 5.0;
                let mid_angle = arc.mid_angle();
                arc.center = Pos2::new(
                    center.x + mid_angle.cos() * expand,
                    center.y + mid_angle.sin() * expand,
                );
            }

            arc.draw_animated(&painter, progress);
        }

        // Draw donut hole as a filled circle on top for perfectly round inner edge
        if inner_radius > 0.0 {
            let hole_color = if self.theme.background_color != Color32::TRANSPARENT {
                self.theme.background_color
            } else {
                // Use white or dark based on theme
                Color32::WHITE
            };
            painter.circle_filled(center, inner_radius, hole_color);
        }

        // Draw labels
        if self.show_labels || self.show_percentages {
            let total: f64 = self.data.iter().sum();
            for (i, arc) in arcs.iter().enumerate() {
                if progress > 0.5 {
                    // Only show labels after animation is halfway
                    let label_radius = outer_radius + 15.0;
                    let pos = arc.mid_point(label_radius);

                    let mut text = String::new();
                    if self.show_labels {
                        if let Some(label) = self.labels.get(i) {
                            text.push_str(label);
                        }
                    }
                    if self.show_percentages && total > 0.0 {
                        let pct = self.data.get(i).unwrap_or(&0.0) / total * 100.0;
                        if !text.is_empty() {
                            text.push_str(": ");
                        }
                        text.push_str(&format!("{:.0}%", pct));
                    }

                    if !text.is_empty() {
                        // Determine alignment based on position relative to center
                        let align = if pos.x < center.x - 10.0 {
                            egui::Align2::RIGHT_CENTER
                        } else if pos.x > center.x + 10.0 {
                            egui::Align2::LEFT_CENTER
                        } else {
                            // Near center horizontally - align based on vertical position
                            if pos.y < center.y {
                                egui::Align2::CENTER_BOTTOM
                            } else {
                                egui::Align2::CENTER_TOP
                            }
                        };

                        painter.text(
                            pos,
                            align,
                            text,
                            egui::FontId::proportional(11.0),
                            self.theme.text_color,
                        );
                    }
                }
            }
        }

        // Draw center text for donut
        if self.pie_style.donut_ratio > 0.0 {
            let total: f64 = self.data.iter().sum();
            painter.text(
                center,
                egui::Align2::CENTER_CENTER,
                format_value(total),
                egui::FontId::proportional(16.0),
                self.theme.text_color,
            );
        }

        // Handle interaction
        let mut hovered_index = None;
        let mut clicked_index = None;

        if let Some(hover_pos) = response.hover_pos() {
            for (i, arc) in arcs.iter().enumerate() {
                if arc.contains(hover_pos) {
                    hovered_index = Some(i);
                    break;
                }
            }
        }

        if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                for (i, arc) in arcs.iter().enumerate() {
                    if arc.contains(pos) {
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
                    let arc = &arcs[idx];
                    let total: f64 = self.data.iter().sum();
                    let value = self.data[idx];
                    let pct = if total > 0.0 { value / total * 100.0 } else { 0.0 };

                    let content = TooltipContent {
                        title: None,
                        label: self
                            .labels
                            .get(idx)
                            .cloned()
                            .unwrap_or_else(|| format!("Segment {}", idx + 1)),
                        value: format!("{} ({:.1}%)", format_value(value), pct),
                        color: arc.fill_color,
                    };

                    let tooltip_size = measure_tooltip_size(&painter, &content, &self.theme.tooltip);
                    let anchor = arc.mid_point((inner_radius + outer_radius) / 2.0);
                    let tooltip_pos = calculate_tooltip_position(anchor, tooltip_size, rect);

                    draw_tooltip(&painter, &content, tooltip_pos, &self.theme.tooltip);
                }
            }
        }

        // Store memory
        ui.ctx().data_mut(|d| d.insert_temp(id, memory.clone()));

        PieChartResponse {
            response,
            hovered: memory.hovered_index,
            clicked: clicked_index,
        }
    }

    /// Build arc elements from data
    fn build_arc_elements(&self, center: Pos2, inner_radius: f32, outer_radius: f32) -> Vec<ArcElement> {
        if self.data.is_empty() {
            return vec![];
        }

        let total: f64 = self.data.iter().sum();
        if total <= 0.0 {
            return vec![];
        }

        let colors: Vec<Color32> = if self.colors.is_empty() {
            self.pie_style.colors.clone()
        } else {
            self.colors.iter().map(|c| c.to_color32()).collect()
        };

        let mut start_angle = self.pie_style.start_angle;
        let mut arcs = Vec::with_capacity(self.data.len());

        for (i, &value) in self.data.iter().enumerate() {
            let sweep = (value / total) as f32 * 2.0 * PI;
            let end_angle = start_angle + sweep;

            let mut arc = ArcElement::new(center, inner_radius, outer_radius, start_angle, end_angle);
            arc.fill_color = colors.get(i % colors.len()).cloned().unwrap_or(Color32::GRAY);
            arc.border_color = self.pie_style.border_color;
            arc.border_width = self.pie_style.border_width;

            arcs.push(arc);
            start_angle = end_angle;
        }

        arcs
    }
}

impl Widget for PieChart {
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
