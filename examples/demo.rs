use eframe::egui;
use egui_charts::prelude::*;
use rand::Rng;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 700.0]),
        ..Default::default()
    };
    eframe::run_native(
        "egui_charts Demo",
        options,
        Box::new(|_cc| Ok(Box::new(DemoApp::default()))),
    )
}

#[derive(PartialEq, Clone, Copy)]
enum ChartType {
    Bar,
    Line,
    Pie,
}

struct DemoApp {
    chart_type: ChartType,
    bar_data: Vec<f64>,
    line_data: Vec<f64>,
    pie_data: Vec<f64>,
    labels: Vec<String>,
    pie_labels: Vec<String>,
    theme: ThemePreset,
    animation_duration: f32,
    show_tooltip: bool,
    show_legend: bool,
    // Bar chart options
    bar_width: f32,
    bar_border_radius: u8,
    bar_show_grid: bool,
    // Line chart options
    line_fill: bool,
    line_curved: bool,
    line_show_points: bool,
    line_show_grid: bool,
    line_width: f32,
    // Pie chart options
    donut_ratio: f32,
    pie_show_labels: bool,
    pie_show_percentages: bool,
}

impl Default for DemoApp {
    fn default() -> Self {
        Self {
            chart_type: ChartType::Bar,
            bar_data: vec![65.0, 59.0, 80.0, 81.0, 56.0, 55.0, 40.0],
            line_data: vec![28.0, 48.0, 40.0, 19.0, 86.0, 27.0, 90.0],
            pie_data: vec![30.0, 25.0, 20.0, 15.0, 10.0],
            labels: vec!["Mon".into(), "Tue".into(), "Wed".into(), "Thu".into(), "Fri".into(), "Sat".into(), "Sun".into()],
            pie_labels: vec!["Chrome".into(), "Safari".into(), "Firefox".into(), "Edge".into(), "Other".into()],
            theme: ThemePreset::Light,
            animation_duration: 0.8,
            show_tooltip: true,
            show_legend: true,
            // Bar chart options
            bar_width: 0.8,
            bar_border_radius: 4,
            bar_show_grid: true,
            // Line chart options
            line_fill: true,
            line_curved: true,
            line_show_points: true,
            line_show_grid: true,
            line_width: 3.0,
            // Pie chart options
            donut_ratio: 0.5,
            pie_show_labels: false,
            pie_show_percentages: false,
        }
    }
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("egui_charts");
                ui.separator();

                // Chart type selector
                ui.selectable_value(&mut self.chart_type, ChartType::Bar, "Bar");
                ui.selectable_value(&mut self.chart_type, ChartType::Line, "Line");
                ui.selectable_value(&mut self.chart_type, ChartType::Pie, "Pie");

                ui.separator();

                if ui.button("Randomize").clicked() {
                    let mut rng = rand::thread_rng();
                    match self.chart_type {
                        ChartType::Bar => {
                            self.bar_data = (0..7).map(|_| rng.gen_range(20.0..100.0)).collect();
                        }
                        ChartType::Line => {
                            self.line_data = (0..7).map(|_| rng.gen_range(10.0..100.0)).collect();
                        }
                        ChartType::Pie => {
                            self.pie_data = (0..5).map(|_| rng.gen_range(10.0..50.0)).collect();
                        }
                    }
                }

                ui.separator();

                ui.label("Theme:");
                egui::ComboBox::from_id_salt("theme")
                    .selected_text(format!("{:?}", self.theme))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.theme, ThemePreset::Light, "Light");
                        ui.selectable_value(&mut self.theme, ThemePreset::Dark, "Dark");
                        ui.selectable_value(&mut self.theme, ThemePreset::Minimal, "Minimal");
                    });

                ui.separator();
                ui.checkbox(&mut self.show_tooltip, "Tooltips");
                ui.checkbox(&mut self.show_legend, "Legend");

                ui.separator();
                ui.label("Animation:");
                ui.add(egui::Slider::new(&mut self.animation_duration, 0.0..=2.0).suffix("s"));
            });
        });

        // Side panel for chart-specific options
        egui::SidePanel::left("options").min_width(160.0).show(ctx, |ui| {
            ui.heading("Options");
            ui.separator();

            match self.chart_type {
                ChartType::Bar => {
                    ui.label("Bar Chart");
                    ui.separator();
                    ui.checkbox(&mut self.bar_show_grid, "Show Grid");
                    ui.add_space(8.0);
                    ui.add(
                        egui::Slider::new(&mut self.bar_width, 0.3..=1.0)
                            .text("Bar Width"),
                    );
                    ui.add(
                        egui::Slider::new(&mut self.bar_border_radius, 0..=20)
                            .text("Corner Radius"),
                    );
                }
                ChartType::Line => {
                    ui.label("Line Chart");
                    ui.separator();
                    ui.checkbox(&mut self.line_show_grid, "Show Grid");
                    ui.checkbox(&mut self.line_fill, "Area Fill");
                    ui.checkbox(&mut self.line_curved, "Curved Lines");
                    ui.checkbox(&mut self.line_show_points, "Show Points");
                    ui.add_space(8.0);
                    ui.add(
                        egui::Slider::new(&mut self.line_width, 1.0..=6.0)
                            .text("Line Width"),
                    );
                }
                ChartType::Pie => {
                    ui.label("Pie Chart");
                    ui.separator();
                    ui.checkbox(&mut self.pie_show_labels, "Show Labels");
                    ui.checkbox(&mut self.pie_show_percentages, "Show Percentages");
                    ui.add_space(8.0);
                    ui.add(
                        egui::Slider::new(&mut self.donut_ratio, 0.0..=0.8)
                            .text("Donut Hole"),
                    );
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Apply dark background if dark theme
            if self.theme == ThemePreset::Dark {
                ui.painter().rect_filled(
                    ui.available_rect_before_wrap(),
                    0.0,
                    egui::Color32::from_gray(30),
                );
            }

            ui.vertical_centered(|ui| {
                ui.add_space(20.0);

                match self.chart_type {
                    ChartType::Bar => {
                        ui.heading("Weekly Sales Data");
                        ui.add_space(10.0);

                        let colors = vec![
                            "#36a2eb", "#ff6384", "#ffce56", "#4bc0c0", "#9966ff", "#ff9f40", "#c9cbcf",
                        ];

                        BarChart::new()
                            .data(self.bar_data.clone())
                            .labels(self.labels.clone())
                            .colors(colors.clone())
                            .bar_width(self.bar_width)
                            .border_radius(self.bar_border_radius)
                            .grid(self.bar_show_grid)
                            .animate(Animation::custom(Easing::EaseOutQuart, self.animation_duration))
                            .tooltip(self.show_tooltip)
                            .theme_preset(self.theme)
                            .size([600.0, 350.0])
                            .show(ui);

                        // Draw legend
                        if self.show_legend {
                            ui.add_space(15.0);
                            draw_legend(ui, &self.labels, &colors, self.theme);
                        }
                    }
                    ChartType::Line => {
                        ui.heading("Weekly Temperature");
                        ui.add_space(10.0);

                        let color = "#36a2eb";

                        LineChart::new()
                            .data(self.line_data.clone())
                            .labels(self.labels.clone())
                            .color(color)
                            .fill(self.line_fill)
                            .curved(self.line_curved)
                            .show_points(self.line_show_points)
                            .line_width(self.line_width)
                            .point_radius(5.0)
                            .grid(self.line_show_grid)
                            .animate(Animation::custom(Easing::EaseOutQuart, self.animation_duration))
                            .tooltip(self.show_tooltip)
                            .theme_preset(self.theme)
                            .size([600.0, 350.0])
                            .show(ui);

                        // Draw legend
                        if self.show_legend {
                            ui.add_space(15.0);
                            draw_legend(ui, &["Temperature".to_string()], &[color], self.theme);
                        }
                    }
                    ChartType::Pie => {
                        ui.heading("Browser Market Share");
                        ui.add_space(10.0);

                        let colors = vec!["#36a2eb", "#ff6384", "#ffce56", "#4bc0c0", "#9966ff"];

                        PieChart::new()
                            .data(self.pie_data.clone())
                            .labels(self.pie_labels.clone())
                            .colors(colors.clone())
                            .donut(self.donut_ratio)
                            .show_labels(self.pie_show_labels)
                            .show_percentages(self.pie_show_percentages)
                            .animate(Animation::custom(Easing::EaseOutQuart, self.animation_duration))
                            .tooltip(self.show_tooltip)
                            .theme_preset(self.theme)
                            .size([350.0, 350.0])
                            .show(ui);

                        // Draw legend
                        if self.show_legend {
                            ui.add_space(15.0);
                            draw_legend(ui, &self.pie_labels, &colors, self.theme);
                        }
                    }
                }
            });
        });
    }
}

/// Draw a simple legend below the chart
fn draw_legend<S: AsRef<str>>(ui: &mut egui::Ui, labels: &[S], colors: &[&str], theme: ThemePreset) {
    let text_color = match theme {
        ThemePreset::Dark => egui::Color32::from_gray(220),
        _ => egui::Color32::from_gray(60),
    };

    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 16.0;

        for (i, label) in labels.iter().enumerate() {
            let color_str = colors.get(i % colors.len()).unwrap_or(&"#888888");
            let color = parse_color(color_str);

            ui.horizontal(|ui| {
                // Color box
                let (rect, _) = ui.allocate_exact_size(egui::vec2(12.0, 12.0), egui::Sense::hover());
                ui.painter().rect_filled(rect, 2.0, color);

                // Label
                ui.label(egui::RichText::new(label.as_ref()).color(text_color).size(12.0));
            });
        }
    });
}

/// Parse hex color string to Color32
fn parse_color(s: &str) -> egui::Color32 {
    let s = s.trim_start_matches('#');
    if s.len() == 6 {
        let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(128);
        let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(128);
        let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(128);
        egui::Color32::from_rgb(r, g, b)
    } else {
        egui::Color32::GRAY
    }
}
