use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="text-lg fixed bottom-0 w-full">
            <ul class="flex gap-4 justify-center items-stretch h-12 bg-green-200">
                <a class="px-2 content-center" href="/"><li>Order</li></a>
                <a class="px-2 content-center" href="/products"><li>Products</li></a>
                <a class="px-2 content-center" href="/money"><li>Money</li></a>
                <a class="px-2 content-center" href="/about"><li>About</li></a>
            </ul>
        </nav>
    }
}
