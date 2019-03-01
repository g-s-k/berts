use std::fs::{DirBuilder, File};
use std::io::{prelude::*, Result};

use base64::{encode_config, URL_SAFE};

fn main() -> Result<()> {
    DirBuilder::new().recursive(true).create("tmp_static")?;

    let mut html_out = File::create("tmp_static/index.html")?;
    html_out.write_all(
        format!(
            include_str!("src/static/index.html"),
            icon = encode_config(&include_bytes!("src/static/beet.ico")[..], URL_SAFE),
            styles = include_str!("src/static/styles.css"),
            script = include_str!("../target/wasm32-unknown-unknown/release/beet-up-www.js")
        )
        .as_bytes(),
    )?;

    Ok(())
}
