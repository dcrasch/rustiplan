use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct CellIndicatorProps {
    pub row: usize,
    pub column: usize,
}

pub fn CellIndicator(cx: Scope<CellIndicatorProps>) -> Element {
    cx.render(rsx! {
    div { class: "cellindicator",
       "R{cx.props.row+1}C{cx.props.column+1}"
    }
    })
}
