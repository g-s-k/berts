#![allow(clippy::needless_pass_by_value)]

use std::fs::File;
use std::io::Read;

use warp::{http::Response, Rejection, Reply};

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

pub fn get_item_file(id: u32, model: Model) -> Result<impl Reply, Rejection> {
    match model.lock().unwrap().get_item_id(id) {
        Some(beet_db::Item {
            bitrate,
            format,
            length,
            path,
            ..
        }) => {
            let mut f = match File::open(path) {
                Ok(f) => f,
                Err(_) => return Err(warp::reject::not_found()),
            };

            #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
            let mut buffer = Vec::with_capacity(bitrate as usize * length as usize);
            if f.read_to_end(&mut buffer).is_err() {
                return Err(warp::reject::not_found());
            }

            Ok(Response::builder()
                .header(
                    "content-type",
                    match format.as_ref() {
                        "AIFF" => "audio/x-aiff",
                        "FLAC" => "audio/flac",
                        "MP3" => "audio/mpeg",
                        "OGG" => "audio/ogg",
                        _ => "application/octet-stream",
                    },
                )
                .body(buffer))
        }
        None => Err(warp::reject::not_found()),
    }
}
