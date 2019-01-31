use std::path::PathBuf;

use warp::{filters::BoxedFilter, path, Filter, Rejection, Reply};

pub fn router() -> impl Filter<Extract = impl Reply, Error = Rejection> {
    let stats = path("stats").map(|| "library stats");
    route_items().or(route_albums()).or(stats)
}

fn route_albums() -> BoxedFilter<(impl Reply,)> {
    let get_all = path::end().map(|| "get all");
    let get_by_id = path::param().map(|id: u32| format!("get the album with this id: {}", id));
    let get_by_ids = path::param().map(|ids: String| format!("get these album ids: {}", ids));
    let get_by_query = path("query")
        .and(path::param())
        .map(|q: String| format!("get the results of this query: {:?}", q));

    path("album")
        .and(get_all.or(get_by_id).or(get_by_query).or(get_by_ids))
        .boxed()
}

fn route_items() -> impl Filter<Extract = impl Reply, Error = Rejection> {
    let get_all = path::end().map(|| "get all");
    let get_by_id = path::param().map(|id: u32| format!("get the track with this id: {}", id));
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

    // TODO: /item/:id/file
    path("item").and(
        get_all
            .or(get_by_id)
            .or(get_by_path)
            .or(get_by_query)
            .or(get_by_ids),
    )
}
