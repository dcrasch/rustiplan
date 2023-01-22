use dioxus::prelude::*;
use dioxus::html::input_data::keyboard_types::Key;

#[derive(Props)]
pub struct CellProps<'a> {
    pub row: usize,
    pub column: usize,
    pub children: String,
    pub onupdate: EventHandler<'a, String>,
    pub onselected: EventHandler<'a>,
}

pub fn Cell<'a>(cx: Scope<'a, CellProps<'a>>) -> Element {
    let is_editing = use_state(cx, || false);
    let contents = use_state(cx, || cx.props.children.clone());
    let editing = if **is_editing { "editing" } else { "" };

    cx.render(rsx! {
    div {
        class: "cell {editing}",
        onclick: move |_| {
            cx.props.onselected.call(());
            is_editing.set(true);
        },
        label { "{contents}"},
        is_editing.then(|| {
              rsx! {
                input {
                    value: "{contents.clone()}",
                    oninput: move |evt| {
                        contents.set(evt.value.clone());
                    },
            onblur: move |_| {
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
