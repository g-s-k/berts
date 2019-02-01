use std::collections::HashMap;
use std::path::PathBuf;

use beet_db::{read_all, Album, Item};

pub struct Model {
    albums: HashMap<u32, Album>,
    items: HashMap<u32, Item>,
}

impl Model {
    pub fn new(db_path: PathBuf) -> Self {
        let err_msg = format!("Could not read database at {:?}", db_path);
        let (album_list, item_list) = read_all(db_path).expect(&err_msg);

        let albums = album_list.into_iter().map(|a| (a.id, a)).collect();
        let items = item_list.into_iter().map(|a| (a.id, a)).collect();

        Self { albums, items }
    }

    pub fn get_all_albums(&self) -> Vec<Album> {
        self.albums.values().cloned().collect::<Vec<_>>()
    }

    pub fn get_album_id(&self, id: u32) -> Option<Album> {
        self.albums.get(&id).cloned()
    }

    pub fn get_all_items(&self) -> Vec<Item> {
        self.items.values().cloned().collect::<Vec<_>>()
    }

    pub fn get_item_id(&self, id: u32) -> Option<Item> {
        self.items.get(&id).cloned()
    }
}
