use leptos::prelude::*;

#[component]
pub fn Money() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center">
            <h2 class="text-2xl">"Money"</h2>
            <p>"Your balance: 10€"</p>
            <p>"Remaining time: 1 hour"</p>
        </div>
    }
}
