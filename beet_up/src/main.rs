use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;

use structopt::StructOpt;
use warp::{path, Filter, Rejection, Reply};

#[derive(Debug, StructOpt)]
#[structopt(name = "beet-up")]
#[structopt(about = "a web player for beets")]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
#[structopt(rename_all = "kebab-case")]
struct Cli {
    /// The server hostname.
    #[structopt(long, parse(try_from_str), default_value = "0.0.0.0")]
    host: IpAddr,
    /// The port to listen on.
    #[structopt(short, long, default_value = "8337")]
    port: u16,
    /// The CORS allowed origin. CORS is off if not provided.
    #[structopt(long)]
    cors: Option<String>,
    /// Support credentials when using CORS.
    #[structopt(long, requires = "cors")]
    cors_supports_credentials: bool,
    /// Respect forwarded headers when behind a reverse proxy.
    #[structopt(long)]
    reverse_proxy: bool,
    /// Include paths in item responses.
    #[structopt(long)]
    include_paths: bool,
    /// Path to your beet database.
    #[structopt(parse(from_os_str))]
    db_path: PathBuf,
}

fn router() -> impl Filter<Extract = impl Reply, Error = Rejection> {
    let get_all = path::end().map(|| "get all");
    let get_by_id = path::param().map(|id: u32| format!("get one id: {}", id));
    let get_by_ids = path::param().map(|ids: String| format!("get these ids: {}", ids));
    let get_by_path = path("path")
        .and(path::tail())
        .map(|t: path::Tail| format!("get this path: {:#?}", PathBuf::from(t.as_str())));
    let get_by_query = path("query")
        .and(path::param())
        .map(|q: String| format!("get the results of this query: {:?}", q));

    // TODO: /item/:id/file
    let item = path("item").and(
        get_all
            .or(get_by_id)
            .or(get_by_path)
            .or(get_by_query)
            .or(get_by_ids),
    );

    let album = path("album").and(get_all.or(get_by_id).or(get_by_query).or(get_by_ids));

    let stats = path("stats").map(|| "library stats");

    item.or(album).or(stats)
}

fn main() {
    let cli = Cli::from_args();
    let addr = SocketAddr::new(cli.host, cli.port);
    println!("Now listening at http://{}.", addr);

    warp::serve(router()).run(addr)
}
