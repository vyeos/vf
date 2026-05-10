use std::path::{Path, PathBuf};
use std::{fs, io::Result};

use crate::models::FileItem;

pub fn read_dir(path: &Path) -> Vec<FileItem> {
    let mut items = vec![];

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            items.push(FileItem {
                name: entry.file_name().to_string_lossy().to_string(),
                is_dir: path.is_dir(),
                path,
            })
        }
    } else {
        println!("Couldn't read the directory.");
    }
    items
}

pub fn create_dir(path: &Path) -> Result<()> {
    fs::create_dir(path)
}

pub fn rename_item(old_path: &Path, new_path: &Path) -> Result<()> {
    fs::rename(old_path, new_path)
}

// in caller side: if let Err(err) = rename_obj(old, new) {
//     println!("{}", err);
// }
// or
// match rename_item(old, new) {
//     Ok(_) => println!("Rename successful"),
//     Err(err) => println!("Rename failed: {}", err),
// }

pub fn add_fav(path: &Path, fav_sig: &mut Vec<PathBuf>) {
    fav_sig.push(path.to_path_buf());
}
