use dioxus::prelude::*;
use log::{info, LevelFilter};

mod components;
mod formula;
use crate::components::Sheet;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        div { Sheet {} }
    ))
}
