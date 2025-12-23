use eframe::egui;
use egui_charts::prelude::*;
use rand::Rng;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "egui_charts Demo",
        options,
        Box::new(|_cc| Ok(Box::new(DemoApp::default()))),
    )
}

struct DemoApp {
    data: Vec<f64>,
    theme: ThemePreset,
    animation_duration: f32,
    show_tooltip: bool,
    show_grid: bool,
    selected_bar: Option<usize>,
}

impl Default for DemoApp {
    fn default() -> Self {
        Self {
            data: vec![65.0, 59.0, 80.0, 81.0, 56.0, 55.0, 40.0],
            theme: ThemePreset::Light,
            animation_duration: 0.8,
            show_tooltip: true,
            show_grid: true,
            selected_bar: None,
        }
    }
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("egui_charts Demo");
                ui.separator();

                if ui.button("Randomize Data").clicked() {
                    let mut rng = rand::thread_rng();
                    self.data = (0..7).map(|_| rng.gen_range(20.0..100.0)).collect();
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
                ui.checkbox(&mut self.show_grid, "Grid");

                ui.separator();

                ui.label("Animation:");
                ui.add(egui::Slider::new(&mut self.animation_duration, 0.0..=2.0).suffix("s"));
            });
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
                ui.heading("Weekly Sales Data");
                ui.add_space(10.0);

                let response = BarChart::new()
                    .data(self.data.clone())
                    .labels(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"])
                    .colors(vec![
                        "#36a2eb", "#ff6384", "#ffce56", "#4bc0c0", "#9966ff", "#ff9f40", "#c9cbcf",
                    ])
                    .animate(Animation::custom(Easing::EaseOutQuart, self.animation_duration))
                    .tooltip(self.show_tooltip)
                    .grid(self.show_grid)
                    .theme_preset(self.theme)
                    .size([700.0, 400.0])
                    .show(ui);

                // Handle click
                if let Some(clicked) = response.clicked {
                    self.selected_bar = Some(clicked);
                }

                ui.add_space(20.0);

                // Show selected bar info
                if let Some(idx) = self.selected_bar {
                    let days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
                    let day = days.get(idx).unwrap_or(&"Unknown");
                    ui.label(format!("Selected: {} - {:.1}", day, self.data.get(idx).unwrap_or(&0.0)));
                }

                // Show hover info
                if let Some(idx) = response.hovered {
                    ui.label(format!("Hovering bar {} (value: {:.1})", idx + 1, self.data.get(idx).unwrap_or(&0.0)));
                }
            });
        });
    }
}
