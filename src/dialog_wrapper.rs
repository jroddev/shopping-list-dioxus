use dioxus::prelude::*;

#[component()]
pub fn DialogWrapper(is_open: SyncSignal<bool>, children: Element) -> Element {
    if !is_open() {
        return None;
    }

    rsx! {
        div {
            position: "absolute",
            top: 0,
            left: 0,
            background_color: "rgba(0,0,0,0.5)",
            width: "100%",
            height: "100%",
            onclick: move |_|{
                is_open.set(false);
            },
            dialog {
                open: true,
                onclick: move |e| {
                    e.stop_propagation();
                },
                {&children}
            }
        }
    }
}
