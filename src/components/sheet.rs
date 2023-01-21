use dioxus::prelude::*;
use fermi::*;

use crate::components::cell::Cell;

static DATA: AtomRef<Vec<Vec<String>>> = |_| vec![vec!["".to_string(); 5]; 10];

pub fn Sheet(cx: Scope) -> Element {
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
