use std::{
    fs::{self, ReadDir, read_dir},
    io,
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

    // TODO validate request?
    pub fn open_dir(&mut self, dir_name: String) {
        self.current_path.push(dir_name);

        // Branch might have divereged, clear history
        self.path_history.clear();
    }

    pub fn set_path(&mut self, path: String) {
        if Self::is_dir(&path) {
            self.current_path = PathBuf::from(path);
        }
    }

    pub fn delete_file_or_dir(&mut self, path: String) -> io::Result<()> {
        if Self::is_file(&path) {
            fs::remove_file(path)
        } else if Self::is_dir(&path) {
            fs::remove_dir_all(path)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Path doesnt lead to a valid file or a directory",
            ))
        }
    }

    pub fn rename_file_or_dir(&mut self, path: String, new_name: String) -> io::Result<()> {
        let mut new_path = PathBuf::from(path.clone());
        new_path.pop();
        new_path.push(new_name);

        fs::rename(path, new_path)
    }

    fn is_file(path: &String) -> bool {
        fs::metadata(path).map(|m| m.is_file()).unwrap_or(false)
    }

    fn is_dir(path: &String) -> bool {
        fs::metadata(path).map(|m| m.is_dir()).unwrap_or(false)
    }
}
