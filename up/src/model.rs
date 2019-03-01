use std::collections::HashSet;
use std::path::PathBuf;

use serde_derive::Serialize;

use beet_db::{read_all, Album, Item};
use beet_query::Query;

pub struct Model {
    albums: Vec<Album>,
    items: Vec<Item>,
    legal_paths: HashSet<PathBuf>,
}

#[derive(Serialize)]
pub struct Stats {
    albums: usize,
    items: usize,
}

impl Model {
    pub fn new(db_path: PathBuf) -> Self {
        let err_msg = format!("Could not read database at {:?}", db_path);
        let (albums, items) = read_all(db_path).expect(&err_msg);

        let legal_paths = albums
            .iter()
            .filter_map(|Album { artpath, .. }| artpath.clone())
            .chain(items.iter().map(|Item { path, .. }| path).cloned())
            .collect();

        Self {
            albums,
            items,
            legal_paths,
        }
    }

    pub fn get_stats(&self) -> Stats {
        Stats {
            albums: self.albums.len(),
            items: self.items.len(),
        }
    }

    pub fn get_all_albums(&self) -> Vec<Album> {
        self.albums.clone()
    }

    pub fn check_path(&self, path: &PathBuf) -> bool {
        self.legal_paths.contains(path)
    }

    pub fn get_album_items_id(&self, id: u32) -> Vec<Item> {
        self.items
            .iter()
            .filter(|Item { album_id, .. }| match &album_id {
                Some(i) if *i == id => true,
                _ => false,
            })
            .cloned()
            .collect()
    }

    pub fn get_album_id(&self, id: u32) -> Option<Album> {
        self.albums.iter().find(|a| a.id == id).cloned()
    }

    pub fn get_album_ids(&self, ids: &[u32]) -> Vec<Album> {
        let s = ids.iter().collect::<HashSet<_>>();
        self.albums
            .iter()
            .filter(|Album { id, .. }| s.contains(id))
            .cloned()
            .collect()
    }

    pub fn get_all_items(&self) -> Vec<Item> {
        self.items.clone()
    }

    pub fn get_item_id(&self, id: u32) -> Option<Item> {
        self.items.iter().find(|i| i.id == id).cloned()
    }

    pub fn get_item_ids(&self, ids: &[u32]) -> Vec<Item> {
        let s = ids.iter().collect::<HashSet<_>>();
        self.items
            .iter()
            .filter(|Item { id, .. }| s.contains(id))
            .cloned()
            .collect()
    }

    pub fn get_item_path(&self, pth: &PathBuf) -> Option<Item> {
        self.items
            .iter()
            .find(|Item { path, .. }| path == pth)
            .cloned()
    }

    pub fn query_albums(&self, q: &Query) -> Vec<Album> {
        self.albums
            .iter()
            .filter(|album| q.match_album(album))
            .cloned()
            .collect()
    }

    pub fn query_items(&self, q: &Query) -> Vec<Item> {
        self.items
            .iter()
            .filter(|item| q.match_item(item))
            .cloned()
            .collect()
    }
}
