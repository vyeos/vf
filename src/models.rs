use std::{collections::HashSet, path::PathBuf};

use dioxus::{prelude::{ReadableExt, WritableExt}, signals::Signal};

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
    pub selected_items: Signal<HashSet<PathBuf>>,
}

impl AppState {
    pub fn new(current_path: PathBuf) -> Self {
        Self {
            current_path: Signal::new(current_path),
            favourites: Signal::new(vec![]),
            selected_items: Signal::new(HashSet::new()),
        }
    }

    pub fn is_selected(&self, path: &PathBuf) -> bool {
        self.selected_items.read().contains(path)
    }

    pub fn select_item(&mut self, path: PathBuf, keep_existing: bool) {
        self.selected_items.with_mut(|selected| {
            if keep_existing {
                if !selected.insert(path.clone()) {
                    selected.remove(&path);
                }
            } else {
                selected.clear();
                selected.insert(path);
            }
        });
    }

    pub fn clear_selection(&mut self) {
        self.selected_items.write().clear();
    }

    pub fn set_current_path(&mut self, path: PathBuf) {
        self.current_path.set(path);
        self.clear_selection();
    }

    pub fn open_item(&mut self, item: &FileItem) {
        if item.is_dir {
            self.set_current_path(item.path.clone());
        }
    }
}
