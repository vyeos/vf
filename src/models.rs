use std::path::PathBuf;

#[derive(Clone, PartialEq)]
pub struct FileItem {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
}

#[derive(Clone)]
struct Favourite {
    path: PathBuf,
}
