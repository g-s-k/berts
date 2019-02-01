#![deny(clippy::pedantic)]

use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use structopt::StructOpt;
use warp::Filter;

mod model;
mod router;

const LOG_TARGET: &str = "beet_up::api";

type Model = Arc<Mutex<model::Model>>;

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

fn main() {
    pretty_env_logger::init();
    let cli = Cli::from_args();

    let model = model::Model::new(cli.db_path);

    let addr = SocketAddr::new(cli.host, cli.port);
    println!("Now listening at http://{}.", addr);

    warp::serve(router::router(&Arc::new(Mutex::new(model))).with(warp::log::log(LOG_TARGET)))
        .run(addr)
}
