use rs_file_explorer::gui::AppGui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "rs-file-explorer",
        options,
        Box::new(|_cc| Ok(Box::new(AppGui::default()))),
    )
}
