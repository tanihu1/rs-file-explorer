use eframe::egui::{self, FontDefinitions};

use crate::app::App;

pub struct AppGui {
    app: App,
    initialized: bool,
    font_id: egui::FontId,
}

impl Default for AppGui {
    fn default() -> Self {
        Self {
            app: App::default(),
            initialized: false,
            font_id: egui::FontId::proportional(18.0),
        }
    }
}

impl eframe::App for AppGui {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // Lazy init
        if !self.initialized {
            self.initialize(ctx);
        }

        self.set_scale(ctx);
        self.draw_top_panel(ctx);
        self.draw_bottom_panel(ctx);
    }
}

impl AppGui {
    fn initialize(&mut self, ctx: &eframe::egui::Context) {
        if self.initialized {
            panic!("Gui initialization called twice!");
        }

        egui_extras::install_image_loaders(ctx);

        self.initialized = true;
    }

    fn set_scale(&self, ctx: &egui::Context) {
        ctx.set_pixels_per_point(1.5);
    }

    fn draw_top_panel(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("nav_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Back/Forward buttons
                if ui.button("<").clicked() {
                    println!("Left nav button clicked!");
                }

                if ui.button(">").clicked() {
                    println!("Right nav button clicked!");
                }

                ui.separator();

                // Current path
                if let Some(current_path) = self.app.get_current_path() {
                    ui.label(current_path);
                }
            })
        });
    }

    fn draw_bottom_panel(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("file_grid").show(ui, |ui| {
                let file_itr_result = self.app.get_current_dir_contents();

                if let Ok(itr) = file_itr_result {
                    for _ in itr {
                        ui.add(egui::Image::new(egui::include_image!(
                            "../assets/folder_icon.svg"
                        )));
                    }
                }
            });
        });
    }
}
