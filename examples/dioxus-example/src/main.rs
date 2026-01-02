use dioxus::prelude::*;
use scoped_css::{ScopedStyles, scoped_css};

const CSS_ASSET: Asset = asset!("/assets/main.generated.css");
const CSS: ScopedStyles = scoped_css!("main.css");

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { class: CSS["scoped"],
            p { class: CSS["p"], "hello css" }
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        Hero {}
    }
}

#[component]
fn Navbar() -> Element {
    rsx! {
        div { id: "navbar",
            Link { to: Route::Home {}, "Home" }
        }

        Outlet::<Route> {}
    }
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: CSS_ASSET }
        Router::<Route> {}
    }
}

fn main() {
    dioxus::launch(App);
}
