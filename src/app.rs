use std::{
    fs::{self, ReadDir, read_dir},
    path::PathBuf,
};

pub struct App {
    current_path: PathBuf,
}

impl Default for App {
    fn default() -> Self {
        let mut current_path = PathBuf::new();
        current_path.push("./");

        Self {
            current_path: current_path,
        }
    }
}

impl App {
    pub fn get_current_dir_contents(&self) -> std::io::Result<ReadDir> {
        read_dir(&self.current_path)
    }
}
