use std::{fs, process::Command};

use eframe::egui::{
    self, Label, Popup, PopupCloseBehavior, RectAlign, Response, RichText, TextEdit,
};

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
        // TODO Lazy init, check for better initialization?
        if !self.initialized {
            self.initialize(ctx);
        }

        self.set_scale(ctx, 1.5);
        self.draw_top_panel(ctx);
        self.draw_directory_panel(ctx);
    }
}

impl AppGui {
    const COLUMN_WIDTH: f32 = 35.0;
    const ROW_HEIGHT: f32 = 30.0;
    const SPACING: egui::Vec2 = egui::vec2(20.0, 20.0);

    /// Initialize the app. Mostly needed for image loaders installation.
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
                self.draw_navigation_buttons(ui);

                ui.separator();

                self.draw_path_box(ui);
            })
        });
    }

    fn draw_navigation_buttons(&mut self, ui: &mut egui::Ui) {
        if ui.button("<").clicked() {
            self.app.navigate_back();
        }

        if ui.button(">").clicked() {
            self.app.navigate_forward();
        }
    }

    fn draw_path_box(&mut self, ui: &mut egui::Ui) {
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

                // New path given manually by user. If path is invalid, App will ignore
                self.app.set_path(self.displayed_path.clone());
            }
        }
    }

    fn draw_directory_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |central_ui| {
            // max num needs to be calculated here for access to central_ui
            let max_column_num = (central_ui.available_width() / AppGui::COLUMN_WIDTH) as usize;

            egui::Grid::new("file_grid")
                .min_col_width(AppGui::COLUMN_WIDTH)
                .max_col_width(AppGui::COLUMN_WIDTH)
                .min_row_height(AppGui::ROW_HEIGHT)
                .spacing(AppGui::SPACING)
                .show(central_ui, |grid_ui| {
                    self.draw_dir_entries(grid_ui, max_column_num);
                });

            // Checking for cursor mass selection
            self.handle_mass_selection(ctx, central_ui);
        });
    }

    fn draw_dir_entries(&mut self, ui: &mut egui::Ui, max_column_num: usize) {
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
    }

    fn handle_mass_selection(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        if let Some(selection_rect) = self.get_selection_rectangle(ctx) {
            // Needed for item highlighting
            self.selection_area = Some(selection_rect.clone());

            self.draw_selection_rectangle(ui, selection_rect);
        } else {
            self.selection_area = None;
        }
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

/// Represents an entry inside the directory
struct DirEntry<'a> {
    name: String,
    is_dir: bool,
    abs_path: String,
    dir_button: egui::ImageButton<'a>,
    file_button: egui::ImageButton<'a>,
}

/// Creating the entry is done by using from on fs::DirEntry
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

                self.handle_right_click(app_ref, &dir_btn_handle);
            } else {
                let file_btn_handle = ui.add(self.file_button.clone());

                self.handle_highlighting(ui, app_ref, &file_btn_handle);

                self.handle_click(app_ref, &file_btn_handle);

                self.handle_right_click(app_ref, &file_btn_handle);
            }

            ui.add(
                Label::new(RichText::new(self.name.clone()).size(10.0))
                    .wrap()
                    .selectable(false),
            );
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
            if app_ref.selection_area.unwrap().intersects(button_rect) {
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
                    dbg!(&self.abs_path);
                    let _ = Command::new("xdg-open").arg(&self.abs_path).spawn();
                }
            }
        }
    }

    fn handle_right_click(&self, app_ref: &mut AppGui, btn_handle: &Response) {
        let close_behavior = PopupCloseBehavior::CloseOnClickOutside;
        let align = RectAlign::BOTTOM_START;
        Popup::context_menu(btn_handle)
            .gap(4.0)
            .align(align)
            .close_behavior(close_behavior)
            .show(|ui| {
                if ui.button("Delete").clicked() {
                    // TODO Handle possible error in action
                    let _ = app_ref.app.delete_file_or_dir(self.abs_path.clone());
                    ui.close();
                }
            });
    }
}
