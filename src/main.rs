#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, CentralPanel, Window};

struct MyApp {
    numbers: Vec<u32>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            // 假设我们有一个包含五个数字的数组
            numbers: vec![10, 20, 30, 40, 50],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("This is the central panel.");
        });

        Window::new("Window 1").show(ctx, |ui| {
            ui.label("Hello");
        });

        Window::new("Window 2").show(ctx, |ui| {
            ui.horizontal(|ui| {
                for &number in &self.numbers {
                    ui.vertical(|ui| {
                        let text = format!("{}", number);
                        let size = egui::vec2(50.0, 25.0);
                        let (rect, _response) = ui.allocate_exact_size(size, egui::Sense::hover());
                        if ui.is_rect_visible(rect) {
                            ui.painter().rect_filled(rect, 0.0, egui::Color32::LIGHT_BLUE);
                            let galley = ui.painter().layout_no_wrap(text.clone(), egui::FontId::new(20f32, egui::FontFamily::Monospace), egui::Color32::BLACK);
                            let text_pos = rect.center() - galley.size() / 2.0;
                            ui.painter().galley(text_pos, galley);
                        }
                    });
                }
            });
        });

        Window::new("Window 3").show(ctx, |ui| {
            if ui.button("Swap").clicked() {
                self.swap(0, 1);
                ctx.request_repaint();
            }
        });
    }
}

impl MyApp {
    fn swap(&mut self, f: usize, s: usize) {
        self.numbers.swap(f, s);
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Two Windows Example",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}
