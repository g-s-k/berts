#![allow(clippy::needless_pass_by_value)]

use std::path::PathBuf;

use url::percent_encoding::{percent_decode, utf8_percent_encode, DEFAULT_ENCODE_SET};
use warp::{
    http::Uri,
    reject::{custom, not_found},
    reply::json,
    Rejection, Reply,
};

use beet_query::Query;

use super::Error;
use super::super::Model;


fn req_err<T>(_: T) -> Rejection {
    custom(Error::BadRequest)
}

fn sync_err<T>(_: T) -> Rejection {
    custom(Error::Sync)
}

pub fn get_stats(model: Model) -> Result<impl Reply, Rejection> {
    model.lock().map_err(sync_err).map(|m| json(&m.get_stats()))
}

pub fn get_all_albums(model: Model) -> Result<impl Reply, Rejection> {
    model
        .lock()
        .map_err(sync_err)
        .map(|m| json(&m.get_all_albums()))
}

pub fn get_album_items_id(id: u32, qstr: String, model: Model) -> Result<impl Reply, Rejection> {
    if qstr.trim() == "expand" {
        let tracks = model.lock().map_err(sync_err)?.get_album_items_id(id);
        if tracks.is_empty() {
            Err(not_found())
        } else {
            Ok(json(&tracks))
        }
    } else {
        Err(not_found())
    }
}

pub fn get_album_id(id: u32, model: Model) -> Result<impl Reply, Rejection> {
    model
        .lock()
        .map_err(sync_err)?
        .get_album_id(id)
        .ok_or_else(not_found)
        .map(|a| json(&a))
}

pub fn get_album_art(id: u32, model: Model) -> Result<impl Reply, Rejection> {
    match model.lock().map_err(sync_err)?.get_album_id(id) {
        Some(beet_db::Album {
            artpath: Some(path),
            ..
        }) => Ok(warp::redirect(
            format!(
                "/file/{}",
                utf8_percent_encode(&path.to_string_lossy(), DEFAULT_ENCODE_SET)
            )
            .parse::<Uri>()
            .map_err(req_err)?,
        )),
        _ => Err(not_found()),
    }
}

pub fn get_all_items(model: Model) -> Result<impl Reply, Rejection> {
    model
        .lock()
        .map_err(sync_err)
        .map(|m| json(&m.get_all_items()))
}

pub fn get_item_id(id: u32, model: Model) -> Result<impl Reply, Rejection> {
    model
        .lock()
        .map_err(sync_err)?
        .get_item_id(id)
        .ok_or_else(not_found)
        .map(|i| json(&i))
}

pub fn get_ids(ids: String) -> Result<Vec<u32>, Rejection> {
    ids.split(',')
        .map(|s| s.parse::<u32>().map_err(req_err))
        .collect()
}

pub fn get_album_ids(ids: Vec<u32>, model: Model) -> Result<impl Reply, Rejection> {
    model
        .lock()
        .map_err(sync_err)
        .map(|m| json(&m.get_album_ids(&ids)))
}

pub fn get_item_ids(ids: Vec<u32>, model: Model) -> Result<impl Reply, Rejection> {
    model
        .lock()
        .map_err(sync_err)
        .map(|m| json(&m.get_item_ids(&ids)))
}

pub fn get_item_path(path: warp::path::Tail, model: Model) -> Result<impl Reply, Rejection> {
    model
        .lock()
        .map_err(sync_err)?
        .get_item_path(&PathBuf::from(
            percent_decode(path.as_str().as_bytes())
                .decode_utf8()
                .map_err(req_err)?
                .to_string(),
        ))
        .ok_or_else(not_found)
        .map(|item| json(&item))
}

pub fn get_item_file(id: u32, model: Model) -> Result<impl Reply, Rejection> {
    model
        .lock()
        .map_err(sync_err)?
        .get_item_id(id)
        .ok_or_else(not_found)
        .and_then(|beet_db::Item { path, .. }| {
            Ok(warp::redirect(
                format!(
                    "/file/{}",
                    utf8_percent_encode(&path.to_string_lossy(), DEFAULT_ENCODE_SET)
                )
                .parse::<Uri>()
                .map_err(req_err)?,
            ))
        })
}

pub fn parse_query(q: String) -> Result<Query, Rejection> {
    percent_decode(q.as_bytes())
        .decode_utf8()
        .map_err(req_err)?
        .parse()
        .map_err(req_err)
}

pub fn query_albums(q: Query, model: Model) -> Result<impl Reply, Rejection> {
    model.lock().map_err(sync_err).map(|m| json(&m.query_albums(&q)))
}

pub fn query_items(q: Query, model: Model) -> Result<impl Reply, Rejection> {
    model.lock().map_err(sync_err).map(|m| json(&m.query_items(&q)))
}
