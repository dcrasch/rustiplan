use dioxus::prelude::*;    
use crate::formula::eval;

#[derive(Props,PartialEq)]
pub struct FormulaBarProps {
    pub formula: String,
}

pub fn FormulaBar(cx: Scope<FormulaBarProps>) -> Element {
    cx.render(rsx! {
    div { class: "formulabar",
	  input {
            value: "{cx.props.formula}",
      }
    }
    })
}
