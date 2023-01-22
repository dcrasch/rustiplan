use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

mod components;
use components::Sheet;

fn main() {
    dioxus_desktop::launch_cfg(
        app,
        Config::new()
            .with_window(WindowBuilder::new().with_title("Build Your Own Multiplan"))
            .with_custom_head(format!(
                "<style>{}</style>",
                include_str!("assets/styling.css")
            )),
);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!(
	div {
	    Sheet {}
	}
	)
)
  }
