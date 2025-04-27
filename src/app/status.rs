use leptos::prelude::*;

#[component]
pub fn Status() -> impl IntoView {
    view! {
        <div class="sticky top-3 bg-green-200 p-1 w-fit mx-auto my-2 rounded">
            <p>"Remaining time: 1 hour"</p>
            <p>"Your balance: 10€"</p>
        </div>
    }
}
