use leptos::prelude::*;

#[component]
pub fn Status() -> impl IntoView {
    view! {
        <div class="sticky top-3 bg-green-200 p-2 w-fit mx-auto my-2 rounded-md dark:bg-gray-900 dark:text-gray-200">
            <p>"Remaining time: 1 hour"</p>
            <p>"Your balance: 10€"</p>
        </div>
    }
}
