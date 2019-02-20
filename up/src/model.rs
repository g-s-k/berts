use std::path::PathBuf;

use beet_db::{read_all, Album, Item};

pub struct Model {
    albums: Vec<Album>,
    items: Vec<Item>,
}

impl Model {
    pub fn new(db_path: PathBuf) -> Self {
        let err_msg = format!("Could not read database at {:?}", db_path);
        let (albums, items) = read_all(db_path).expect(&err_msg);
        Self { albums, items }
    }

    pub fn get_all_albums(&self) -> Vec<Album> {
        self.albums.clone()
    }

    pub fn get_album_id(&self, id: u32) -> Option<Album> {
        self.albums.iter().find(|a| a.id == id).cloned()
    }

    pub fn get_all_items(&self) -> Vec<Item> {
        self.items.clone()
    }

    pub fn get_item_id(&self, id: u32) -> Option<Item> {
        self.items.iter().find(|i| i.id == id).cloned()
    }
}
