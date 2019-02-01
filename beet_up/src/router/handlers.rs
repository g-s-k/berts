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
