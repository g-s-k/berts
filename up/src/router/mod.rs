use std::path::PathBuf;

use warp::{filters::BoxedFilter, path, Filter, Reply};

use super::Model;

mod handlers;

pub fn router(model: &Model) -> BoxedFilter<(impl Reply,)> {
    let fallback = warp::any().and(warp::fs::dir("static"));
    route_items(model.clone())
        .or(route_albums(model.clone()))
        .or(route_stats(model.clone()))
        .or(fallback)
        .boxed()
}

fn route_stats(model: Model) -> BoxedFilter<(impl Reply,)> {
    let db = warp::any().map(move || model.clone());
    path("stats")
        .and(db.clone())
        .map(handlers::get_stats)
        .boxed()
}

fn route_albums(model: Model) -> BoxedFilter<(impl Reply,)> {
    let db = warp::any().map(move || model.clone());

    let get_all = path::end().and(db.clone()).map(handlers::get_all_albums);
    let get_items_by_id = path::param()
        .and(warp::query::raw())
        .and(db.clone())
        .and_then(handlers::get_album_items_id);
    let get_by_id = path::param()
        .and(db.clone())
        .and_then(handlers::get_album_id);
    // TODO: implement this
    let get_art_by_id = path!(u32 / "art").map(|id| format!("get album art for album id {}.", id));
    let get_by_ids = path::param()
        .and_then(handlers::get_ids)
        .and(db.clone())
        .and_then(handlers::get_album_ids);
    // TODO: implement this
    let get_by_query = path("query")
        .and(path::param())
        .map(|q: String| format!("get the results of this query: {:?}", q));

    path("album")
        .and(
            get_all
                .or(get_items_by_id)
                .or(get_by_id)
                .or(get_art_by_id)
                .or(get_by_query)
                .or(get_by_ids),
        )
        .boxed()
}

fn route_items(model: Model) -> BoxedFilter<(impl Reply,)> {
    let db = warp::any().map(move || model.clone());

    let get_all = path::end().and(db.clone()).map(handlers::get_all_items);
    let get_by_id = path!(u32)
        .and(path::end())
        .and(db.clone())
        .and_then(handlers::get_item_id);
    let get_file_by_id = path!(u32 / "file")
        .and(path::end())
        .and(db.clone())
        .and_then(handlers::get_item_file);
    let get_by_ids = path::param()
        .and(path::end())
        .and_then(handlers::get_ids)
        .and(db.clone())
        .and_then(handlers::get_item_ids);
    // TODO: implement this
    let get_by_path = path("path").and(path::tail()).map(|t: path::Tail| {
        format!(
            "get the track with this path: {:#?}",
            PathBuf::from(t.as_str())
        )
    });
    // TODO: implement this
    let get_by_query = path("query")
        .and(path::param())
        .map(|q: String| format!("get the results of this query: {:?}", q));

    path("item")
        .and(
            get_all
                .or(get_by_id)
                .or(get_file_by_id)
                .or(get_by_path)
                .or(get_by_query)
                .or(get_by_ids),
        )
        .boxed()
}
