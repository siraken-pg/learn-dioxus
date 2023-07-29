#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::*;

fn main() {
    dioxus_web::launch(App);
}

/// App Component
///
fn App(cx: Scope) -> Element {

    cx.render(rsx! {
        Router {
            Route { to: "/", Home {} }
            Route { to: "/about", About {} }
            Redirect { from: "", to: "/" }
        }
    })
}

/// Home Component
///
fn Home(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    cx.render(rsx! {
        h1 {
            "Home"
        }
        div {
            display: "flex",
            flex_direction: "column",
            gap: "1rem",
            div {
                background_color: "#f8f9fa",
                padding: "1rem",
                border: "solid 1px #ccc",
                p {
                    a {
                        href: "https://dioxuslabs.com/docs/",
                        target: "_blank",
                        "Dioxus document is here!"
                    }
                }
                button {
                    onclick: move |event| println!("Clicked Event: {event:?}"),
                    "Click to get click event"
                }
            }
            div {
                background_color: "#f8f9fa",
                padding: "1rem",
                border: "solid 1px #ccc",
                p {
                    "Count: {count}"
                }
                button {
                    onclick: move |_| {
                        count += 1
                    },
                    "Increment"
                }
                button {
                    onclick: move |_| {
                        count -= 1
                    },
                    "Decrement"
                }
            }
            div {
                background_color: "#f8f9fa",
                padding: "1rem",
                border: "solid 1px #ccc",
                p {
                    "About page is "
                    Link {
                        to: "/about",
                        "here"
                    }
                    "!"
                }
            }
        }
    })
}

/// About Component
///
fn About(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 {
            "About"
        }
        div {
            Link {
                to: "/",
                "Go to home"
            }
        }
    })
}
