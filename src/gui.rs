use std::fs;

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

        self.set_scale(ctx, 1.5);
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

    fn set_scale(&self, ctx: &egui::Context, scale: f32) {
        ctx.set_pixels_per_point(scale);
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
            egui::Grid::new("file_grid")
                .min_col_width(30.0)
                .min_row_height(30.0)
                .show(ui, |ui| {
                    let file_itr_result = self.app.get_current_dir_contents().unwrap();

                    for entry in file_itr_result {
                        if let Ok(entry_result) = entry {
                            let gui_dir_entry = DirEntry::from(entry_result);

                            gui_dir_entry.draw(ui);
                        }
                    }
                    ui.end_row();
                });
        });
    }
}

struct DirEntry {
    name: String,
    is_dir: bool,
}

impl From<fs::DirEntry> for DirEntry {
    fn from(value: fs::DirEntry) -> Self {
        let name = value.file_name().into_string().unwrap();
        let is_dir = value.file_type().unwrap().is_dir();

        Self { name, is_dir }
    }
}

impl DirEntry {
    fn draw(&self, ui: &mut egui::Ui) {
        // TODO match the file type
        ui.vertical(|ui| {
            if self.is_dir {
                ui.add(egui::Image::new(egui::include_image!(
                    "../assets/folder_icon.svg"
                )));
            } else {
                ui.add(egui::Image::new(egui::include_image!(
                    "../assets/file_icon.svg"
                )));
            }
            ui.label(self.name.clone());
        });
    }
}
