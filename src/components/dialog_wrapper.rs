use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DialogWrapperProps {
    is_open: Signal<bool>,
    children: Element,
}

#[component]
pub fn DialogWrapper(mut props: DialogWrapperProps) -> Element {
    if !*props.is_open.read() {
        return rsx! {};
    }

    rsx! {
        div {
            style: "position: absolute; top: 0; left: 0; background-color: rgba(0,0,0,0.5); width: 100%; height: 100%;",
            onclick: move |_| {
                *props.is_open.write() = false;
            },
            dialog {
                open: true,
                onclick: move |e| {
                    e.stop_propagation();
                },
                {props.children}
            }
        }
    }
}
