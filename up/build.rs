use std::fs::{DirBuilder, File};
use std::io::{prelude::*, Result};

use base64::encode;

fn main() -> Result<()> {
    DirBuilder::new().recursive(true).create("tmp_static")?;

    let mut html_out = File::create("tmp_static/index.html")?;
    html_out.write_all(
        format!(
            include_str!("src/static/index.html"),
            icon = encode(&include_bytes!("src/static/beet.ico")[..]),
            styles = include_str!("src/static/styles.css"),
            script = include_str!("../target/wasm32-unknown-unknown/release/beet-up-www.js")
        )
        .as_bytes(),
    )?;

    Ok(())
}
