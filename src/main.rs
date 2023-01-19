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

#[derive(Props, PartialEq)]
pub struct CellProps {
    pub row: usize,
    pub column: usize,
    pub children: String,
}

#[allow(non_snake_case)]
pub fn Cell(cx: Scope<CellProps>) -> Element {
    let data = use_atom_ref(cx, DATA);

    let is_editing = use_state(cx, || false);
    let contents = cx.props.children.clone();
    let editing = if **is_editing { "editing" } else { "" };

    cx.render(rsx! {
    div { class: "cell {editing}",
          onclick: move |_| { is_editing.set(true) },
          label {
                  "{contents}"
              },
          is_editing.then(|| {
          rsx! {
                      input {
              value: "{contents.clone()}",
              autofocus: "true",
              tabindex: 0,
              onchange: move |evt| {
                  data.write()[cx.props.row][cx.props.column]=evt.value.clone();
                              is_editing.set(false);
              },
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
                td {
            Cell { row:row_index, column:column_index, children:children}
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
