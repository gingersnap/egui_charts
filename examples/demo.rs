use eframe::egui;
use egui_charts::prelude::*;
use rand::Rng;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 700.0]),
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
    theme: ThemePreset,
    animation_duration: f32,
    show_tooltip: bool,
    // Line chart options
    line_fill: bool,
    line_curved: bool,
    line_show_points: bool,
    // Pie chart options
    donut_ratio: f32,
}

impl Default for DemoApp {
    fn default() -> Self {
        Self {
            chart_type: ChartType::Bar,
            bar_data: vec![65.0, 59.0, 80.0, 81.0, 56.0, 55.0, 40.0],
            line_data: vec![28.0, 48.0, 40.0, 19.0, 86.0, 27.0, 90.0],
            pie_data: vec![30.0, 25.0, 20.0, 15.0, 10.0],
            theme: ThemePreset::Light,
            animation_duration: 0.8,
            show_tooltip: true,
            line_fill: true,
            line_curved: true,
            line_show_points: true,
            donut_ratio: 0.5,
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

                ui.separator();
                ui.label("Animation:");
                ui.add(egui::Slider::new(&mut self.animation_duration, 0.0..=2.0).suffix("s"));
            });
        });

        // Side panel for chart-specific options
        egui::SidePanel::left("options").show(ctx, |ui| {
            ui.heading("Options");
            ui.separator();

            match self.chart_type {
                ChartType::Bar => {
                    ui.label("Bar Chart Options");
                    ui.label("(using defaults)");
                }
                ChartType::Line => {
                    ui.label("Line Chart Options");
                    ui.checkbox(&mut self.line_fill, "Area Fill");
                    ui.checkbox(&mut self.line_curved, "Curved Lines");
                    ui.checkbox(&mut self.line_show_points, "Show Points");
                }
                ChartType::Pie => {
                    ui.label("Pie Chart Options");
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

                        BarChart::new()
                            .data(self.bar_data.clone())
                            .labels(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"])
                            .colors(vec![
                                "#36a2eb", "#ff6384", "#ffce56", "#4bc0c0", "#9966ff", "#ff9f40", "#c9cbcf",
                            ])
                            .animate(Animation::custom(Easing::EaseOutQuart, self.animation_duration))
                            .tooltip(self.show_tooltip)
                            .theme_preset(self.theme)
                            .size([600.0, 350.0])
                            .show(ui);
                    }
                    ChartType::Line => {
                        ui.heading("Weekly Temperature");
                        ui.add_space(10.0);

                        LineChart::new()
                            .data(self.line_data.clone())
                            .labels(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"])
                            .color("#36a2eb")
                            .fill(self.line_fill)
                            .curved(self.line_curved)
                            .show_points(self.line_show_points)
                            .line_width(3.0)
                            .point_radius(5.0)
                            .animate(Animation::custom(Easing::EaseOutQuart, self.animation_duration))
                            .tooltip(self.show_tooltip)
                            .theme_preset(self.theme)
                            .size([600.0, 350.0])
                            .show(ui);
                    }
                    ChartType::Pie => {
                        ui.heading("Browser Market Share");
                        ui.add_space(10.0);

                        PieChart::new()
                            .data(self.pie_data.clone())
                            .labels(vec!["Chrome", "Safari", "Firefox", "Edge", "Other"])
                            .colors(vec!["#36a2eb", "#ff6384", "#ffce56", "#4bc0c0", "#9966ff"])
                            .donut(self.donut_ratio)
                            .animate(Animation::custom(Easing::EaseOutQuart, self.animation_duration))
                            .tooltip(self.show_tooltip)
                            .theme_preset(self.theme)
                            .size([400.0, 400.0])
                            .show(ui);
                    }
                }
            });
        });
    }
}
