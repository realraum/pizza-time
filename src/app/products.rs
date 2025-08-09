use leptos::{either::Either, html::Dialog, prelude::*, task::spawn_local};

#[cfg(feature = "ssr")]
use crate::server::get_user_id_and_create_if_required;
use crate::{
    app::{components::ProductCard, order::get_users},
    common::{Pizza, SusPizza},
};

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
    #[derive(Clone, PartialEq)]
    enum DialogState {
        AddProduct(Pizza),
        Closed,
    }

    // let count = RwSignal::new(Vec::new());
    // let on_click = move |_| *count.write() += 1;

    let pizza_types = Resource::new(move || (), move |_| get_pizza_types());

    let dialog_ref = NodeRef::<Dialog>::new();
    let dialog_state = RwSignal::new(DialogState::Closed);
    let close = move |_| {
        dialog_state.set(DialogState::Closed);
    };
    Effect::new(move |_| {
        if let Some(dialog) = dialog_ref.get() {
            match dialog_state.get() {
                DialogState::AddProduct(_) => {
                    dialog.show_modal().unwrap();
                }
                DialogState::Closed => {
                    dialog.close();
                }
            }
        }
    });

    let users = Resource::new(dialog_state, async move |_| {
        let users = get_users().await;
        users.map(|users| (users.0.values().cloned().collect::<Vec<_>>(), users.1))
    });

    view! {
        <dialog
            node_ref=dialog_ref
        >
            <div class="fixed inset-0 flex items-center justify-center bg-gray-800 bg-opacity-50">
                <div class="bg-white rounded-lg p-4 flex flex-col">
                    <h2 class="text-xl">"Add Pizza"</h2>
                    <p>"Select a user to add the pizza for"</p>
                    <Suspense
                        fallback=|| view! { <p>"Loading users..."</p> }
                    >
                        {move || {
                            let Some(pizza) = (match dialog_state.get() {
                                DialogState::AddProduct(pizza) => Some(pizza),
                                DialogState::Closed => None,
                            }) else {
                                return Either::Left(view! { <p>"Failed to load pizza types"</p> })
                            };

                            Either::Right(users.get()
                                .map(move |users| {
                                    let (users, my_uid)= users.unwrap();

                                    let me = users. iter()
                                        .find(|user| user.id == my_uid)
                                        .unwrap().clone();

                                    let others = users.iter()
                                        // .filter(|user| user.id != my_uid)
                                        .cloned()
                                        .collect::<Vec<_>>();

                                    others.into_iter()
                                        .map(|user| {
                                            let user2 = user.clone();
                                            let pt = pizza.clone();
                                            Either::Left(view! {
                                                <div class="flex flex-col items-center gap-2 w-full">
                                                    <button
                                                        class="bg-green-500 text-white rounded-md p-1 ml-2"
                                                        on:click=move |_| {
                                                            let pt = pt.clone();
                                                            spawn_local(async move {
                                                                add_pizza_for_me(pt).await.unwrap();
                                                            });
                                                        }
                                                    >
                                                        {format!("{} ({})", user.name, user.id)}
                                                    </button>
                                                </div>
                                            })
                                        })
                                        .collect::<Vec<_>>()
                                })
                                .unwrap_or_else(|| {
                                    vec![Either::Right(view! {
                                        <p>"Failed to load users"</p>
                                    })]
                                }))
                        }}
                    </Suspense>
                    <button on:click=close>"Close"</button>
                </div>
            </div>
        </dialog>
        // {
        //     move || if dialog_state.get() {
        //         Some(view! {
        //             // <dialog>
        //             //     <div class="fixed inset-0 flex items-center justify-center bg-gray-800 bg-opacity-50">
        //             //         <div class="bg-white rounded-lg p-4">
        //             //             <h2 class="text-xl">"Add Pizza"</h2>
        //             //             <button on:click=close>"Close"</button>
        //
        //             //         </div>
        //             //     </div>
        //             // </dialog>
        //         })
        //     } else {
        //         None
        //     }
        // }
        // <button on:click=on_click>"Click Me: " {count}</button>
        <Suspense
            fallback=|| view! { <p>"Loading pizza types..."</p> }
        >
            <h2 class="text-2xl">"Pizza types"</h2>
            <p>"Click on a pizza to add it to your order"</p>
            // <div class="grid grid-cols-4 gap-4">
            //     <span>"Name"</span>
            //     <span>"Description"</span>
            //     <span>"Price"</span>
            //     <span>"Action"</span>
            //     {move || {
            //         let Some(pizza_types) = pizza_types.get() else {
            //             return Either::Left(view! { <p>"Failed to load pizza types"</p> })
            //         };
            //
            //         Either::Right(pizza_types.unwrap().into_iter()
            //             .map(|pt| {
            //                 let pt2 = pt.clone();
            //                 view! {
            //                         <span>{pt.name.clone()}</span>
            //                         <span>{pt.description.clone()}</span>
            //                         <span>{pt.price.to_string()}</span>
            //                         <div class="flex flex-col gap-2">
            //                             <button
            //                                 class="bg-green-500 text-white rounded-md p-1 ml-2"
            //                                 on:click=move |_| {
            //                                     let pt = pt.clone();
            //                                     spawn_local(async move {
            //                                         add_pizza_for_me(pt).await.unwrap();
            //                                     });
            //                                 }
            //                             >
            //                                 "For me"
            //                             </button>
            //                             <button
            //                                 class="bg-green-500 text-white rounded-md p-1 ml-2"
            //                                 on:click=move |_| {
            //                                     dialog_state.set(DialogState::AddProduct(pt2.clone()));
            //                                 }
            //                             >
            //                                 "Select person"
            //                             </button>
            //                         </div>
            //                 }
            //             })
            //             .collect::<Vec<_>>())
            //     }}
            // </div>
            <div class="grid grid-cols-1 gap-4">
                {move || {
                    let Some(pizza_types) = pizza_types.get() else {
                        return Either::Left(view! { <p>"Failed to load pizza types"</p> })
                    };

                    Either::Right(pizza_types.unwrap().into_iter()
                        .map(|pt| {
                            let pt2 = pt.clone();
                            view! {
                                <ProductCard pizza=pt2 />
                            }
                        })
                        .collect::<Vec<_>>())
                }}
            </div>
        </Suspense>
    }
}
