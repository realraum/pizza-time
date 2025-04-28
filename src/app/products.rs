use leptos::{either::Either, prelude::*, task::spawn_local};

use crate::common::{Pizza, SusPizza};
#[cfg(feature = "ssr")]
use crate::server::get_user_id_and_create_if_required;

use super::PRODUCT_JSON_STR;

#[server(GetPizzaTypes, endpoint = "get_pizza_types")]
pub async fn get_pizza_types() -> Result<Vec<Pizza>, ServerFnError> {
    let pizzas = serde_json::from_str::<Vec<SusPizza>>(PRODUCT_JSON_STR);

    match pizzas {
        Ok(pizzas) => Ok(pizzas.into_iter().map(Pizza::from_sus_pizza).collect()),
        Err(e) => Err(ServerFnError::ServerError(format!(
            "Failed to parse pizza types: {}",
            e
        ))),
    }
}

#[server(AddPizza, endpoint = "add_pizza")]
pub async fn add_pizza(pizza: Pizza) -> Result<(), ServerFnError> {
    let (mut users, uid) = get_user_id_and_create_if_required!();

    if let Some(user) = users.get_mut(&uid) {
        user.order.push(pizza);
    } else {
        return Err(ServerFnError::ServerError("User not found".to_string()));
    }
    Ok(())
}

/// Renders the home page of your application.
#[component]
pub fn PizzaList() -> impl IntoView {
    // let count = RwSignal::new(Vec::new());
    // let on_click = move |_| *count.write() += 1;

    let pizza_types = Resource::new(move || (), move |_| get_pizza_types());

    view! {
        // <button on:click=on_click>"Click Me: " {count}</button>
        <Suspense
            fallback=|| view! { <p>"Loading pizza types..."</p> }
        >
            <h2 class="text-2xl">"Pizza types"</h2>
            <p>"Click on a pizza to add it to your order"</p>
            <table>
                <thead>
                    <tr>
                        <th>"Name"</th>
                        <th>"Description"</th>
                        <th>"Price"</th>
                        // <th>"Action"</th>
                    </tr>
                </thead>
                <tbody>
                {move || {
                    let Some(pizza_types) = pizza_types.get() else {
                        return Either::Left(view! { <p>"Failed to load pizza types"</p> })
                    };

                    Either::Right(pizza_types.unwrap().into_iter()
                        .map(|pt| {
                            view! {
                                <tr>
                                    <td>{pt.name.clone()}</td>
                                    <td>{pt.description.clone()}</td>
                                    <td>{pt.price.to_string()}</td>
                                    <td>
                                        <button
                                            class="bg-green-500 text-white rounded-md p-1 ml-2"
                                            on:click=move |_| {
                                                let pt = pt.clone();
                                                spawn_local(async move {
                                                    add_pizza(pt).await.unwrap();
                                                });
                                            }
                                        >
                                            "Add"
                                        </button>
                                    </td>
                                </tr>
                            }
                        })
                        .collect::<Vec<_>>())
                }}
                </tbody>
            </table>
        </Suspense>
    }
}
