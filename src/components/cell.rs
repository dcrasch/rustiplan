use dioxus::html::input_data::keyboard_types::Key;
use dioxus::prelude::*;

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
    let content = use_state(cx, || cx.props.children.clone());
    let editing = if **is_editing { "editing" } else { "" };

    let start_editing = move |_| {
        cx.props.onselected.call(());
        is_editing.set(true);
    };

    let blur = move |_| {
        cx.props.onupdate.call(content.get().to_string());
        is_editing.set(false);
    };

    let update_content = move |evt: Event<FormData>| content.set(evt.value.clone());

    let keypress = move |evt: Event<KeyboardData>| {
        if evt.key() == Key::Enter {
            cx.props.onupdate.call(content.get().to_string());
            is_editing.set(false);
        }
    };
    let keydown = move |evt: Event<KeyboardData>| {
        if evt.key() == Key::Escape {
            content.set(cx.props.children.to_string());
            is_editing.set(false);
        }
    };

    cx.render(rsx! {
    div {
        class: "cell {editing}",
        onclick: start_editing,
        label { "{content}"},
        is_editing.then(|| { rsx! {
        input {
                value: "{content}",
                oninput: update_content,
                onblur: blur,
                onkeypress: keypress,
        onkeydown: keydown,
        }
    }
        }),
    }
    })
}
