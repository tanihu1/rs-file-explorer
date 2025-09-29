use eframe::egui;

use crate::app::App;

pub struct AppGui {
    app: App,
    initialized: bool,
}

impl Default for AppGui {
    fn default() -> Self {
        Self {
            app: App::default(),
            initialized: false,
        }
    }
}

impl eframe::App for AppGui {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        // Lazy init
        if !self.initialized {
            self.initialize(ctx);
        }

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
                ui.label("Nav path placeholder");
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
