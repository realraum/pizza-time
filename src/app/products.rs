use leptos::{either::Either, html::Dialog, prelude::*, task::spawn_local};

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
pub async fn add_pizza_for_me(pizza: Pizza) -> Result<(), ServerFnError> {
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

    let dialog_ref = NodeRef::<Dialog>::new();
    let is_open = RwSignal::new(false);
    let toggle = move |_| {
        is_open.update(|v| *v = !*v);
    };
    let close = move |_| {
        is_open.set(false);
    };
    Effect::new(move |_| {
        if let Some(dialog) = dialog_ref.get() {
            if is_open.get() {
                dialog.show_modal().unwrap();
            } else {
                dialog.close();
            }
        }
    });

    view! {
        <dialog
            node_ref=dialog_ref
        >
            <div class="fixed inset-0 flex items-center justify-center bg-gray-800 bg-opacity-50">
                <div class="bg-white rounded-lg p-4">
                    <h2 class="text-xl">"Add Pizza"</h2>
                    <button on:click=close>"Close"</button>

                </div>
            </div>
        </dialog>
        <button
            class="bg-blue-500 text-white rounded-md p-1 ml-2"
            on:click=toggle
        >
            "Add Pizza"
        </button>
        {
            move || if is_open.get() {
                Some(view! {
                    // <dialog>
                    //     <div class="fixed inset-0 flex items-center justify-center bg-gray-800 bg-opacity-50">
                    //         <div class="bg-white rounded-lg p-4">
                    //             <h2 class="text-xl">"Add Pizza"</h2>
                    //             <button on:click=close>"Close"</button>

                    //         </div>
                    //     </div>
                    // </dialog>
                })
            } else {
                None
            }
        }
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
                                <tr class="my-2">
                                    <td>{pt.name.clone()}</td>
                                    <td>{pt.description.clone()}</td>
                                    <td>{pt.price.to_string()}</td>
                                    <td class="flex flex-col">
                                        <button
                                            class="bg-green-500 text-white rounded-md p-1 ml-2"
                                            on:click=move |_| {
                                                let pt = pt.clone();
                                                spawn_local(async move {
                                                    add_pizza_for_me(pt).await.unwrap();
                                                });
                                            }
                                        >
                                            "Add"
                                        </button>
                                        <button
                                            class="bg-green-500 text-white rounded-md p-1 ml-2"
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
