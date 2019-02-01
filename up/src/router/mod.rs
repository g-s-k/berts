use std::path::PathBuf;

use warp::{filters::BoxedFilter, path, Filter, Reply};

use super::Model;

mod handlers;

pub fn router(model: &Model) -> BoxedFilter<(impl Reply,)> {
    let stats = path("stats").map(|| "library stats");
    let fallback = warp::any().and(warp::fs::dir("static"));
    route_items(model.clone())
        .or(route_albums(model.clone()))
        .or(stats)
        .or(fallback)
        .boxed()
}

fn route_albums(model: Model) -> BoxedFilter<(impl Reply,)> {
    let db = warp::any().map(move || model.clone());

    let get_all = path::end().and(db.clone()).map(handlers::get_all_albums);
    let get_by_id = path::param()
        .and(db.clone())
        .and_then(handlers::get_album_id);
    let get_by_ids = path::param().map(|ids: String| format!("get these album ids: {}", ids));
    let get_by_query = path("query")
        .and(path::param())
        .map(|q: String| format!("get the results of this query: {:?}", q));

    path("album")
        .and(get_all.or(get_by_id).or(get_by_query).or(get_by_ids))
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
        .map(|id| format!("get the file for track id {}", id));
    let get_by_ids = path::param().map(|ids: String| format!("get these track ids: {}", ids));
    let get_by_path = path("path").and(path::tail()).map(|t: path::Tail| {
        format!(
            "get the track with this path: {:#?}",
            PathBuf::from(t.as_str())
        )
    });
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
