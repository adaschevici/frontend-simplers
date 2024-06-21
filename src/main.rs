#![allow(non_snake_case)]

mod server;

use dioxus::prelude::*;
// use server::{get_server_data, post_server_data};
use tracing::Level;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO,).expect("failed to init logger",);
    launch(App,);
}

fn App() -> Element {
    let title = "title";
    let by = "author";
    let score = 0;
    let time = chrono::Utc::now();
    let comments = "comments";

    rsx! {"{title} by {by} ({score}) {time} {comments}"}
}
