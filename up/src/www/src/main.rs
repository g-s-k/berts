#![recursion_limit = "128"]

#[macro_use]
extern crate yew;

use yew::prelude::*;

mod app;

fn main() {
    yew::initialize();
    App::<app::App>::new().mount_to_body();
    yew::run_loop();
}
