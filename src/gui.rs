use std::{fs, process::Command};

use eframe::egui::{self, Label, Response, RichText, TextEdit};

use crate::app::App;

pub struct AppGui {
    app: App,
    initialized: bool,
    displayed_path: String,
    is_editing_path: bool,
    selection_area: Option<egui::Rect>,
}

impl Default for AppGui {
    fn default() -> Self {
        Self {
            app: App::default(),
            initialized: false,
            displayed_path: String::new(),
            is_editing_path: false,
            selection_area: None,
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

                            gui_dir_entry.draw(ui, self);
                        }
                    }
                    ui.end_row();
                });

            // Checking for cursor mass selection
            if let Some(selection_rect) = self.get_selection_rectangle(ctx) {
                // Needed for item highlighting
                self.selection_area = Some(selection_rect.clone());

                self.draw_selection_rectangle(ui, selection_rect);
            } else {
                self.selection_area = None;
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

struct DirEntry<'a> {
    name: String,
    is_dir: bool,
    abs_path: String,
    dir_button: egui::ImageButton<'a>,
    file_button: egui::ImageButton<'a>,
}

impl From<fs::DirEntry> for DirEntry<'_> {
    fn from(value: fs::DirEntry) -> Self {
        let name = value.file_name().into_string().unwrap();
        let is_dir = value.file_type().unwrap().is_dir();
        let abs_path = value.path().to_str().unwrap().to_string();
        let dir_button =
            egui::ImageButton::new(egui::include_image!("../assets/folder_icon.svg")).frame(false);
        let file_button =
            egui::ImageButton::new(egui::include_image!("../assets/file_icon.svg")).frame(false);

        Self {
            name,
            is_dir,
            abs_path,
            dir_button,
            file_button,
        }
    }
}

/// Struct represents an entry inside the file grid.
/// Responsible for drawing itself inside the grid, including highlighting and
/// button hooks.
impl DirEntry<'_> {
    // A reference to the app is needed for button functionality
    pub fn draw(&self, ui: &mut egui::Ui, app_ref: &mut AppGui) {
        // Visual settigns

        ui.vertical(|ui| {
            if self.is_dir {
                let dir_btn_handle = ui.add(self.dir_button.clone());

                self.handle_highlighting(ui, app_ref, &dir_btn_handle);

                self.handle_click(app_ref, &dir_btn_handle);
            } else {
                let file_btn_handle = ui.add(self.file_button.clone());

                self.handle_highlighting(ui, app_ref, &file_btn_handle);

                self.handle_click(app_ref, &file_btn_handle);
            }

            ui.add(Label::new(RichText::new(self.name.clone()).size(10.0)).wrap());
        });
    }

    fn handle_highlighting(&self, ui: &mut egui::Ui, app_ref: &mut AppGui, btn_handle: &Response) {
        // Highlight visuals
        let mut highlight_stroke = ui.style().visuals.widgets.hovered.bg_stroke;
        highlight_stroke.width = 2.0;
        let highlight_padding = if self.is_dir { 1.5 } else { 1.0 };
        let highlight_rounding = 4.0;
        let highlight_kind = egui::StrokeKind::Outside;
        let highlight_area = btn_handle.rect.expand(highlight_padding);

        // There is a selection in progress
        if app_ref.selection_area.is_some() {
            let button_rect = btn_handle.rect;
            if app_ref.selection_area.unwrap().contains_rect(button_rect) {
                ui.painter().rect_stroke(
                    highlight_area,
                    highlight_rounding,
                    highlight_stroke,
                    highlight_kind,
                );
            }
        // If no selection, we still need hover effects
        } else if btn_handle.hovered() {
            ui.painter().rect_stroke(
                highlight_area,
                highlight_rounding,
                highlight_stroke,
                highlight_kind,
            );
        }
    }

    fn handle_click(&self, app_ref: &mut AppGui, btn_handle: &Response) {
        if self.is_dir {
            if btn_handle.clicked() {
                app_ref.app.open_dir(self.name.clone());
            }
        } else {
            // Platform specific open logic
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
                if btn_handle.clicked() {
                    let _ = Command::new("xdg-open").arg(&self.abs_path).spawn();
                }
            }
        }
    }
}
