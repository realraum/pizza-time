use leptos::prelude::*;

use crate::common::Pizza;

#[server(GetPizzaTypes, endpoint = "get_pizza_types")]
pub async fn get_pizza_types() -> Result<Vec<Pizza>, ServerFnError> {
    Ok(Vec::new())
}

/// Renders the home page of your application.
#[component]
pub fn PizzaList() -> impl IntoView {
    // let count = RwSignal::new(Vec::new());
    // let on_click = move |_| *count.write() += 1;

    let pizza_types = Resource::new(move || (), move |_| get_pizza_types());

    view! {
        <h1>"🍕 pizza time 🍕"</h1>
        // <button on:click=on_click>"Click Me: " {count}</button>
    }
}
