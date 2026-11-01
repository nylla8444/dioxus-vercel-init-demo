use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(Nav)]
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
}

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[component]
fn Nav() -> Element {
    rsx! {
        nav {
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::About {}, "About" }
            a { href: "https://github.com/nylla8444/dioxus-vercel-init-demo", "GitHub" }
        }
        main { Outlet::<Route> {} }
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        h1 { "dioxus-vercel init demo" }
        p {
            "A Dioxus app that Vercel builds by itself on every push to its GitHub repo. "
            "The setup is one vercel.json, written by "
            a { href: "https://crates.io/crates/dioxus-vercel", "dioxus-vercel" }
            " init. No token, secret, workflow or local build."
        }
        p { "If the button counts, the WebAssembly loaded and is running." }
        p { "Every push to main redeploys this page; pull requests get their own preview." }
        button { onclick: move |_| count += 1, "Clicked {count} times" }
    }
}

#[component]
fn About() -> Element {
    rsx! {
        h1 { "About" }
        p {
            "This page is a client-side route. Reloading it shows that Vercel serves "
            "index.html for it and the app's router takes over."
        }
    }
}
