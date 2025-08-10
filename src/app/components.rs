use leptos::prelude::*;

use crate::common::Pizza;

/// A full-with card without borders
#[component]
pub fn ProductCard(pizza: Pizza) -> impl IntoView {
    view! {
        // <div class="block bg-white dark:bg-gray-700 after:w-full after:border-b-8 dark:after:border-b-gray-600">
        <div class="block bg-white dark:bg-gray-700 p-4 border-b dark:border-gray-600">
            <header class="flex items-center justify-between">
                <h2 class="font-bold">{pizza.name}</h2>
                <p class="text-gray-500 dark:text-gray-400">nyaa :3</p>
            </header>
            <p class="text-gray-600 dark:text-gray-300">{pizza.price.to_string()}" €"</p>
            <p class="mt-2">{pizza.description}</p>
            <div class="mt-4 flex justify-start gap-2">
                <button
                    class="bg-green-400 dark:bg-green-500 text-white px-2 py-1 rounded hover:bg-green-600"
                    on:click=move |_| {
                        // Logic to add pizza to order goes here
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
