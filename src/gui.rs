use eframe::egui;

use crate::App;

pub struct AppGui {
    app: App,
}

impl Default for AppGui {
    fn default() -> Self {
        Self { app: App {} }
    }
}

impl eframe::App for AppGui {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        self.draw_top_panel(ctx);
    }
}

impl AppGui {
    fn draw_top_panel(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("NavBar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("LEFT").clicked() {
                    println!("Left nav button clicked!");
                }

                if ui.button("RIGHT").clicked() {
                    println!("Right nav button clicked!");
                }
                ui.separator();
                ui.label("Nav path placeholder");
            })
        });
    }

    fn draw_bottom_panel(&self, ui: egui::Ui) {
        todo!();
    }
}
