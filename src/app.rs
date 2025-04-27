// Stop cargo from complaining about uppercase letters in function names
#![allow(non_snake_case)]

mod header;
mod money;
mod nav;
mod order;
mod products;
mod status;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub static PRODUCT_JSON_STR: &str = include_str!("../private/products.json");

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body class="bg-green-100 h-full">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/r3-pizza-time.css"/>

        // sets the document title
        <Title text="r3 pizza time"/>

        // content for this welcome page
        <Router>
            <header::Header/>
            <status::Status/>
            <main class="w-11/12 mx-auto max-w-3xl">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=order::Summary/>
                    <Route path=StaticSegment("/products") view=products::PizzaList/>
                    <Route path=StaticSegment("/money") view=money::Money/>
                    <Route path=StaticSegment("/about") view=About/>
                </Routes>
            </main>
            <nav::Nav/>
        </Router>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <div class="flex flex-col items-stretch">
            <h2 class="text-2xl">"About"</h2>
            <p>"A simple client-server web app to coordinate ordering several pizzas in a group setting, where either most if not all people have access to a web client, or just one person manages everything."</p>
            <p>"It is built with Leptos and Rust."</p>

            <p class="mt-2">
                "Licensed under the AGPL-3.0 License." <br/>
                "See its source code "
                <a class="underline text-blue-500" href="https://github.com/realraum/pizza-time" rel="external" target="_blank">"here"</a>
            </p>
        </div>
    }
}
