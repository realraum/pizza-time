use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav>
            <ul>
                <A href="/"><li>Order</li></A>
                <A href="/products"><li>Products</li></A>
                <A href="/money"><li>Money</li></A>
                <A href="/about"><li>About</li></A>
            </ul>
        </nav>
    }
}
