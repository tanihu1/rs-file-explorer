use std::fs;

use eframe::egui::{self};

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
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // Lazy init
        if !self.initialized {
            self.initialize(ctx);
        }

        self.set_scale(ctx, 1.5);
        self.draw_top_panel(ctx);
        self.draw_directory_panel(ctx);
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

    fn draw_top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("nav_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Back/Forward buttons
                if ui.button("<").clicked() {
                    println!("Left nav button clicked!");
                    self.app.navigate_back();
                }

                if ui.button(">").clicked() {
                    println!("Right nav button clicked!");
                    self.app.navigate_forward();
                }

                ui.separator();

                // Current path
                if let Some(current_path) = self.app.get_current_path() {
                    ui.label(current_path);
                }
            })
        });
    }

    fn draw_directory_panel(&mut self, ctx: &egui::Context) {
        const COLUMN_WIDTH: f32 = 30.0;
        const ROW_HEIGHT: f32 = 30.0;

        egui::CentralPanel::default().show(ctx, |ui| {
            let max_column_num = (ui.available_width() / COLUMN_WIDTH) as usize;

            egui::Grid::new("file_grid")
                .min_col_width(COLUMN_WIDTH)
                .max_col_width(COLUMN_WIDTH)
                .min_row_height(ROW_HEIGHT)
                .show(ui, |ui| {
                    let file_itr_result = self.app.get_current_dir_contents().unwrap();
                    let mut col_count = 0;

                    for entry in file_itr_result {
                        if col_count == max_column_num {
                            col_count = 0;
                            ui.end_row();
                        } else {
                            col_count += 1;
                        }

                        if let Ok(entry_result) = entry {
                            let gui_dir_entry = DirEntry::from(entry_result);

                            gui_dir_entry.draw(ui, &mut self.app);
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
    // A reference to the app is needed for the buttons
    fn draw(&self, ui: &mut egui::Ui, app_ref: &mut App) {
        // Play with hover settings for a better look
        ui.vertical(|ui| {
            if self.is_dir {
                let dir_btn = ui.add(egui::ImageButton::new(egui::include_image!(
                    "../assets/folder_icon.svg"
                )));
                if dir_btn.clicked() {
                    app_ref.open_dir(self.name.clone());
                }
            } else {
                ui.add(egui::ImageButton::new(egui::include_image!(
                    "../assets/file_icon.svg"
                )));
            }
            ui.label(self.name.clone());
        });
    }
}
