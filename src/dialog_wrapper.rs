use dioxus::prelude::*;

#[derive(Props)]
pub struct DialogWrapperProps<'a> {
    is_open: &'a UseState<bool>,
    children: Element<'a>,
}

pub fn DialogWrapper<'a>(cx: Scope<'a, DialogWrapperProps<'a>>) -> Element {
    if !*cx.props.is_open.get() {
        return None;
    }

    render! {
        div {
            position: "absolute",
            top: 0,
            left: 0,
            background_color: "rgba(0,0,0,0.5)",
            width: "100%",
            height: "100%",
            onclick: |_|{
                cx.props.is_open.set(false);
            },

            dialog {
                open: true,
                onclick: |e| {
                    e.stop_propagation();
                },
                &cx.props.children
            }
        }
    }
}
