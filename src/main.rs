use dioxus::prelude::*;

mod components;
mod formula;
use crate::components::Sheet;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        div { Sheet {} }
    ))
}
