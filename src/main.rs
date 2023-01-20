use dioxus::html::input_data::keyboard_types::Key;
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
use fermi::*;

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

static DATA: AtomRef<Vec<Vec<String>>> = |_| vec![vec!["".to_string(); 5]; 10];

#[derive(Props)]
pub struct CellProps<'a> {
    pub row: usize,
    pub column: usize,
    pub children: String,
    pub onupdate: EventHandler<'a, String>,
}

#[allow(non_snake_case)]
pub fn Cell<'a>(cx: Scope<'a, CellProps<'a>>) -> Element {
    let is_editing = use_state(cx, || false);
    let contents = use_state(cx, || cx.props.children.clone());
    let editing = if **is_editing { "editing" } else { "" };

    cx.render(rsx! {
    div { class: "cell {editing}",
          onclick: move |_| is_editing.set(true),
          label { "{contents}"},
          is_editing.then(|| {
              rsx! {
                input {
                    value: "{contents.clone()}",
                    oninput: move |evt| {
                        contents.set(evt.value.clone());
                    },
            onblur: move |evt| {
            cx.props.onupdate.call(contents.get().to_string());
            is_editing.set(false);
            },
            onkeypress: move |evt| {
            match evt.key() {
                Key::Escape => {
                contents.set(cx.props.children.to_string());
                is_editing.set(false);
                },
                Key::Enter => {
                cx.props.onupdate.call(contents.get().to_string());
                is_editing.set(false);
                }
                _ => {}
            }
            }
                }
            }
        }),
    }
    })
}

fn app(cx: Scope) -> Element {
    use_init_atom_root(cx);
    let data = use_atom_ref(cx, DATA);
    let columns = ('A'..='E').map(|column| {
        rsx! {
        th {
            format!("{}",column) }}
    });
    let rows = (0..10).map(move |row_index| {
        let cells = ('A'..='E').enumerate().map(move |(column_index, _column)| {
            let children = data
                .read()
                .get(row_index)
                .unwrap()
                .get(column_index)
                .unwrap()
                .to_string();
            rsx! {
                td { Cell { row:row_index, column:column_index, children:children,
			    onupdate: move |value: String| {
				data.write()[row_index][column_index]=value.clone();
			    }
		}
                }
            }
        });
        rsx! {
            tr {
                th { scope: "row", format!("{}",row_index+1) }
                cells
            }
        }
    });

    cx.render(rsx! (
        table {
            class : "spreadsheet",
            thead {
                tr {
                    th { aria_label: "empty header"}
                    columns
                }
            }
            tbody {
                rows
            }
        }
    ))
}
