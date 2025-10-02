use std::{fs, process::Command};

use eframe::egui::{self, Label, RichText, TextEdit};

use crate::app::App;

pub struct AppGui {
    app: App,
    initialized: bool,
    displayed_path: String,
    is_editing_path: bool,
}

impl Default for AppGui {
    fn default() -> Self {
        Self {
            app: App::default(),
            initialized: false,
            displayed_path: String::new(),
            is_editing_path: false,
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

        let displayed_path = self.app.get_current_path().unwrap();
        self.displayed_path = displayed_path.to_string();

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
                    self.app.navigate_back();
                }

                if ui.button(">").clicked() {
                    self.app.navigate_forward();
                }

                ui.separator();

                if !self.is_editing_path {
                    self.displayed_path = self.app.get_current_path().unwrap().to_string();
                }

                let path_text_box = ui.add(
                    TextEdit::singleline(&mut self.displayed_path)
                        .cursor_at_end(true)
                        .frame(false)
                        .clip_text(false),
                );

                if path_text_box.has_focus() {
                    self.is_editing_path = true;
                } else {
                    if self.is_editing_path {
                        self.is_editing_path = false;

                        // New path given by user. Need to be tested:
                        self.app.set_path(self.displayed_path.clone());
                    }
                }
            })
        });
    }

    fn draw_directory_panel(&mut self, ctx: &egui::Context) {
        const COLUMN_WIDTH: f32 = 35.0;
        const ROW_HEIGHT: f32 = 30.0;
        const SPACING: egui::Vec2 = egui::vec2(20.0, 20.0);

        egui::CentralPanel::default().show(ctx, |ui| {
            let max_column_num = (ui.available_width() / COLUMN_WIDTH) as usize;

            egui::Grid::new("file_grid")
                .min_col_width(COLUMN_WIDTH)
                .max_col_width(COLUMN_WIDTH)
                .min_row_height(ROW_HEIGHT)
                .spacing(SPACING)
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

            // First cursor drag rect attempt
            if let Some(selection_rect) = self.get_selection_rectangle(ctx) {
                self.draw_selection_rectangle(ui, selection_rect);
            }
        });
    }

    fn get_selection_rectangle(&self, ctx: &egui::Context) -> Option<egui::Rect> {
        // ctx.input locks ctx!
        let mut result = None;
        ctx.input(|i| {
            if i.pointer.any_down() {
                let start_point = i.pointer.press_origin();
                let current_point = i.pointer.latest_pos();

                if start_point.is_some() && current_point.is_some() {
                    result = Some(egui::Rect::from_two_pos(
                        start_point.unwrap(),
                        current_point.unwrap(),
                    ));
                }
            }
        });
        result
    }

    fn draw_selection_rectangle(&self, ui: &egui::Ui, rect: egui::Rect) {
        let stroke = ui.style().visuals.widgets.hovered.bg_stroke;
        let stroke_kind = egui::StrokeKind::Outside;
        let corner_radius = 1.0;

        ui.painter()
            .rect_stroke(rect, corner_radius, stroke, stroke_kind);
    }
}

struct DirEntry {
    name: String,
    is_dir: bool,
    abs_path: String,
}

impl From<fs::DirEntry> for DirEntry {
    fn from(value: fs::DirEntry) -> Self {
        let name = value.file_name().into_string().unwrap();
        let is_dir = value.file_type().unwrap().is_dir();
        let abs_path = value.path().to_str().unwrap().to_string();

        Self {
            name,
            is_dir,
            abs_path,
        }
    }
}

impl DirEntry {
    // A reference to the app is needed for button functionality
    fn draw(&self, ui: &mut egui::Ui, app_ref: &mut App) {
        let dir_btn =
            egui::ImageButton::new(egui::include_image!("../assets/folder_icon.svg")).frame(false);
        let file_btn =
            egui::ImageButton::new(egui::include_image!("../assets/file_icon.svg")).frame(false);

        // Visual settigns
        let hightlight_stroke = ui.style().visuals.widgets.hovered.bg_stroke;
        let dir_hightlight_padding = 2.0;
        let file_hightlight_padding = 1.0;
        let highlight_rounding = 4.0;
        let hightlight_kind = egui::StrokeKind::Outside;

        ui.vertical(|ui| {
            if self.is_dir {
                let dir_btn_handle = ui.add(dir_btn);

                // Creating custom hover effect for button
                if dir_btn_handle.hovered() {
                    let hightlight_area = dir_btn_handle.rect.expand(dir_hightlight_padding);
                    ui.painter().rect_stroke(
                        hightlight_area,
                        highlight_rounding,
                        hightlight_stroke,
                        hightlight_kind,
                    );
                }

                if dir_btn_handle.clicked() {
                    app_ref.open_dir(self.name.clone());
                }
            } else {
                let file_btn_handle = ui.add(file_btn);

                if file_btn_handle.hovered() {
                    let highlight_area = file_btn_handle.rect.expand(file_hightlight_padding);
                    ui.painter().rect_stroke(
                        highlight_area,
                        highlight_rounding,
                        hightlight_stroke,
                        hightlight_kind,
                    );
                }

                // TODO Verify this works
                #[cfg(target_os = "windows")]
                {
                    let path = self.abs_path.clone();
                    if file_btn.clicked() {
                        let _ = Command::new("explorer").arg("/select,").arg(&path).spawn();
                    }
                }

                #[cfg(target_os = "linux")]
                {
                    if file_btn_handle.clicked() {
                        let _ = Command::new("xdg-open").arg(&self.abs_path).spawn();
                    }
                }
            }
            // ui.label(RichText::new(self.name.clone()).size(10.0));
            ui.add(Label::new(RichText::new(self.name.clone()).size(10.0)).wrap());
        });
    }
}
