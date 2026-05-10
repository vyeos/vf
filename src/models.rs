use std::path::PathBuf;

use dioxus::signals::Signal;

#[derive(Clone, PartialEq)]
pub struct FileItem {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
}

#[derive(Clone)]
pub struct AppState {
    pub current_path: Signal<PathBuf>,
    pub favourites: Signal<Vec<PathBuf>>,
}
