#![allow(clippy::needless_pass_by_value)]

use warp::{Rejection, Reply};

use super::super::Model;

pub fn get_all_albums(model: Model) -> impl Reply {
    let album_list = model.lock().unwrap().get_all_albums();
    warp::reply::json(&album_list)
}

pub fn get_album_id(id: u32, model: Model) -> Result<impl Reply, Rejection> {
    match model.lock().unwrap().get_album_id(id) {
        Some(album) => Ok(warp::reply::json(&album)),
        None => Err(warp::reject::not_found()),
    }
}

pub fn get_all_items(model: Model) -> impl Reply {
    let album_list = model.lock().unwrap().get_all_items();
    warp::reply::json(&album_list)
}

pub fn get_item_id(id: u32, model: Model) -> Result<impl Reply, Rejection> {
    match model.lock().unwrap().get_item_id(id) {
        Some(item) => Ok(warp::reply::json(&item)),
        None => Err(warp::reject::not_found()),
    }
}

pub fn get_ids(ids: String) -> Result<Vec<u32>, Rejection> {
    ids.split(',')
        .map(|s| s.parse::<u32>().map_err(|_| warp::reject::not_found()))
        .collect()
}

pub fn get_album_ids(ids: Vec<u32>, model: Model) -> Result<impl Reply, Rejection> {
    let albums = model.lock().unwrap().get_album_ids(&ids);

    if albums.is_empty() {
        Err(warp::reject::not_found())
    } else {
        Ok(warp::reply::json(&albums))
    }
}

pub fn get_item_ids(ids: Vec<u32>, model: Model) -> Result<impl Reply, Rejection> {
    let items = model.lock().unwrap().get_item_ids(&ids);

    if items.is_empty() {
        Err(warp::reject::not_found())
    } else {
        Ok(warp::reply::json(&items))
    }
}
