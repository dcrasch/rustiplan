use dioxus::prelude::*;
use fermi::*;
use log::{info, LevelFilter};

use crate::components::{Cell, CellIndicator, FormulaBar};
static DATA: AtomRef<Vec<Vec<String>>> = AtomRef(|_| vec![vec!["".to_string(); 5]; 10]);
pub fn Sheet(cx: Scope) -> Element {
    use_init_atom_root(cx);
    let selection = use_state(cx, || None);
    let current_formula = use_state(cx, || None);
    let data = use_atom_ref(cx, &DATA);
    let columns = ('A'..='E').map(|column| {
        rsx! {
            th { format!("{}",column) }
        }
    });
    let rows = (0..10).map(move |row_index| {
        let cells = ('A'..='E')
            .enumerate()
            .map(move |(column_index, _column)| -> LazyNodes {
                let children = data
                    .read()
                    .get(row_index)
                    .unwrap()
                    .get(column_index)
                    .unwrap()
                    .to_string();
                rsx! {
                    td {
                        Cell {
                            row: row_index,
                            column: column_index,
                            children: children,
                            onupdate: move |value: String| {
                                data.write()[row_index][column_index] = value.clone();
                            },
                            onchange: move |value: String| {
                                info!("on change set formula {}",value.clone());
                                current_formula.set(Some(value.clone()));
                            },
                            onselected: move |_| {
                                selection.set(Some((row_index, column_index)));
                                current_formula.set(Some(data.read()[row_index][column_index].clone()));
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
        div {
            if let Some((row,column)) = selection.get() {
                rsx!{ CellIndicator { row: *row, column: *column } }
            } else {
                rsx!{ div { "-" } }
            }
            if let Some(formula) = current_formula.get() {
                rsx!{ FormulaBar { formula: formula.clone() } }
            }
            else {
                rsx!{ div { ".." } }
            }
        }
        table { class: "spreadsheet",
            thead {
                tr {
                    th { aria_label: "empty header" }
                    columns
                }
            }
            tbody { rows }
        }
    ))
}
