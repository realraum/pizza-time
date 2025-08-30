use leptos::{prelude::*, task::spawn_local};

use crate::{
    app::{order::rm_pizza, products::add_pizza_for_me},
    common::{dedup_and_count, Pizza},
};

/// A full-with card without borders
#[component]
pub fn ProductCard(pizza: Pizza) -> impl IntoView {
    let pt = pizza.clone();
    view! {
        // <div class="block bg-white dark:bg-gray-700 after:w-full after:border-b-8 dark:after:border-b-gray-600">
        <div class="block bg-white dark:bg-gray-700 p-4 border-b dark:border-gray-600">
            <header class="flex items-center justify-between">
                <h2 class="font-bold">{pizza.name}</h2>
                <p class="text-gray-500 dark:text-gray-400">nyaa :3</p>
            </header>
            <p class="text-gray-600 dark:text-gray-300">{pizza.price.to_string()}" €"</p>
            <p class="mt-2">{pizza.description}</p>
            <div class="mt-2 flex justify-start gap-2">
                <button
                    class="bg-green-400 dark:bg-green-500 text-white px-2 py-1 rounded hover:bg-green-600"
                    on:click=move |_| {
                        let value = pt.clone();
                        spawn_local(async move {
                            add_pizza_for_me(value).await.unwrap();
                        });
                    }
                >
                    "Add for me"
                </button>
                <button
                    class="outline-1 outline-green-500 bg-green-200 text-green-500 px-2 py-1 rounded hover:bg-green-600"
                    on:click=move |_| {
                        // Logic to add pizza to order goes here
                    }
                >
                    "Add for other"
                </button>
            </div>
        </div>
    }
}

/// A card that displays a person's name and the pizzas they ordered
///
/// There's a `<header>` for the person's name & the count & sum of their pizzas,
/// and a `<div>` that contains a textual description of the pizzas they ordered.
#[component]
pub fn PersonCard(
    name: String,
    pizzas: Vec<Pizza>,
    #[prop(default = false)] is_me: bool,
) -> impl IntoView {
    let mut pizzas = dedup_and_count(pizzas);
    pizzas.sort_by(|(a, _), (b, _)| a.name.cmp(&b.name));
    pizzas.sort_by_key(|(_, count)| *count);
    view! {
        <div class="block bg-white dark:bg-gray-700 p-4 border-b dark:border-gray-600">
            <header class="flex items-center justify-between">
                <h2 class="font-bold">{name}</h2>
                <p class="text-gray-500 dark:text-gray-400">
                    {pizzas.len()}
                    { if pizzas.len() == 1 {" pizza"} else {" pizzas"} }
                </p>
            </header>
            <div class="mt-2">
                {
                    pizzas.into_iter().map(|(pizza, count)| {
                        view! {
                            <span>
                            <p>
                                {count} "x " {pizza.name} " - " {pizza.price.to_string()} " €"
                            </p>
                            {if !is_me {
                                Some(view! {
                                        <button
                                            class="bg-red-500 text-white rounded-md p-1 ml-2"
                                            on:click=move |_| {
                                                let pid = pizza.id.clone();
                                                spawn_local(async move {
                                                    rm_pizza(pid).await.unwrap();
                                                });
                                                // update_signal.set(!update_signal.get());
                                            }
                                        >
                                            "Remove"
                                        </button>
                                })
                            } else {
                                None
                            }}
                            </span>
                        }
                    }).collect::<Vec<_>>()
                }
            </div>
        </div>
    }
}
