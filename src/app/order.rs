use leptos::prelude::*;

#[component]
pub fn Summary() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center">
            <h2 class="text-2xl">"Summary"</h2>
            <p>"Your order is being prepared!"</p>
            <p>"Estimated delivery time: 30 minutes"</p>
        </div>
    }
}
