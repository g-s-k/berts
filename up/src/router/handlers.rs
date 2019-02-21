#![allow(clippy::needless_pass_by_value)]

use std::path::PathBuf;

use url::percent_encoding::{percent_decode, utf8_percent_encode, DEFAULT_ENCODE_SET};
use warp::{http::Uri, Rejection, Reply};

use beet_query::Query;

use super::super::Model;

pub fn get_stats(model: Model) -> impl Reply {
    let lib_stats = model.lock().unwrap().get_stats();
    warp::reply::json(&lib_stats)
}

pub fn get_all_albums(model: Model) -> impl Reply {
    let album_list = model.lock().unwrap().get_all_albums();
    warp::reply::json(&album_list)
}

pub fn get_album_items_id(id: u32, qstr: String, model: Model) -> Result<impl Reply, Rejection> {
    if qstr.trim() == "expand" {
        let tracks = model.lock().unwrap().get_album_items_id(id);
        if tracks.is_empty() {
            Err(warp::reject::not_found())
        } else {
            Ok(warp::reply::json(&tracks))
        }
    } else {
        Err(warp::reject::not_found())
    }
}

pub fn get_album_id(id: u32, model: Model) -> Result<impl Reply, Rejection> {
    match model.lock().unwrap().get_album_id(id) {
        Some(album) => Ok(warp::reply::json(&album)),
        None => Err(warp::reject::not_found()),
    }
}

pub fn get_album_art(id: u32, model: Model) -> Result<impl Reply, Rejection> {
    match model.lock().unwrap().get_album_id(id) {
        Some(beet_db::Album {
            artpath: Some(path),
            ..
        }) => Ok(warp::redirect(
            format!(
                "/file/{}",
                utf8_percent_encode(&path.to_string_lossy(), DEFAULT_ENCODE_SET)
            )
            .parse::<Uri>()
            .unwrap(),
        )),
        _ => Err(warp::reject::not_found()),
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

pub fn get_item_path(path: warp::path::Tail, model: Model) -> Result<impl Reply, Rejection> {
    let decoded = percent_decode(path.as_str().as_bytes())
        .decode_utf8()
        .map_err(|_| warp::reject::not_found())?
        .to_string();
    match model.lock().unwrap().get_item_path(&PathBuf::from(decoded)) {
        Some(item) => Ok(warp::reply::json(&item)),
        None => Err(warp::reject::not_found()),
    }
}

pub fn get_item_file(id: u32, model: Model) -> Result<impl Reply, Rejection> {
    match model.lock().unwrap().get_item_id(id) {
        Some(beet_db::Item { path, .. }) => Ok(warp::redirect(
            format!(
                "/file/{}",
                utf8_percent_encode(&path.to_string_lossy(), DEFAULT_ENCODE_SET)
            )
            .parse::<Uri>()
            .unwrap(),
        )),
        None => Err(warp::reject::not_found()),
    }
}

pub fn parse_query(q: String) -> Result<Query, Rejection> {
    percent_decode(q.as_bytes())
        .decode_utf8()
        .map_err(|_| warp::reject::not_found())?
        .parse()
        .map_err(|_| warp::reject::not_found())
}

pub fn query_albums(q: Query, model: Model) -> impl Reply {
    let albums = model.lock().unwrap().query_albums(&q);
    Ok(warp::reply::json(&albums))
}

pub fn query_items(q: Query, model: Model) -> impl Reply {
    let items = model.lock().unwrap().query_items(&q);
    Ok(warp::reply::json(&items))
}
