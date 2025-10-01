use std::{
    fs::{self, ReadDir, read_dir},
    path::PathBuf,
};

pub struct App {
    current_path: PathBuf,
    path_history: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        let current_path = fs::canonicalize("./").expect("Error inititalizing current path.");

        Self {
            current_path: current_path,
            path_history: Vec::new(),
        }
    }
}

impl App {
    pub fn get_current_dir_contents(&self) -> std::io::Result<ReadDir> {
        read_dir(&self.current_path)
    }

    pub fn get_current_path(&self) -> Option<&str> {
        self.current_path.to_str()
    }

    pub fn navigate_back(&mut self) {
        // TODO make simpler
        let prev_path_suffix = self
            .current_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        // let prev_path_suffix = "/".to_string() + prev_path_suffix;

        self.path_history.push(prev_path_suffix);

        self.current_path.pop();
    }

    pub fn navigate_forward(&mut self) {
        let prev_path_suffix = self.path_history.pop();
        if let Some(suffix) = prev_path_suffix {
            dbg!(&suffix);
            self.current_path.push(suffix);
        }
    }
}
