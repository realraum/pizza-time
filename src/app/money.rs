use leptos::{ev::SubmitEvent, prelude::*, task::spawn_local};

#[cfg(feature = "ssr")]
use crate::server::get_user_id_and_create_if_required;
use crate::{app::order::get_users, common::money::Money};

#[server(SetPaid, endpoint = "set_paid")]
async fn set_paid(paid: Money) -> Result<(), ServerFnError> {
    let (mut users, uid) = get_user_id_and_create_if_required!();

    if let Some(user) = users.get_mut(&uid) {
        user.paid_amount = paid;
    } else {
        return Err(ServerFnError::ServerError("User not found".to_string()));
    }

    Ok(())
}

#[component]
pub fn Money() -> impl IntoView {
    let update_signal = RwSignal::new(true);
    let users = Resource::new(
        move || update_signal.get(),
        async move |_| {
            let users = get_users().await;

            users.map(|users| (users.0.values().cloned().collect::<Vec<_>>(), users.1))
        },
    );

    let paid_amount = RwSignal::new(String::new());

    view! {
        <div class="flex flex-col items-center gap-2 w-full">
            <table>
                <tbody>
                    <tr>
                        <td>"You paid"</td>
                        <td>"10€"</td>
                    </tr>
                    <tr>
                        <td>"You received"</td>
                        <td>"0€"</td>
                    </tr>
                    <tr>
                        <td>"Your orders cost"</td>
                        <td>"8€"</td>
                    </tr>
                    <tr>
                        <td>"Current delta"</td>
                        <td>"2€"</td>
                    </tr>
                </tbody>
            </table>
            <form
                class="flex flex-row gap-2"
                on:submit=move |e: SubmitEvent| {
                    e.prevent_default();
                    // HACK this is all just cursed af
                    let mut paid = paid_amount.get();
                    if !paid.contains('.')  {
                        paid.push_str(".0");
                    }
                    let Some((eur, cent)) = paid.split_once('.') else {
                        return;
                    };
                    let mut cent = cent.to_string();
                    if cent.len() == 1 {
                        cent.push('0');
                    }
                    let paid = Money::from_euros_and_cents(
                        eur.parse::<i32>().unwrap(),
                        cent.parse::<i32>().unwrap(),
                    );
                    spawn_local(async move {
                        set_paid(paid).await.unwrap();
                    });
                }
            >
                <input
                    type="text"
                    class="border-2 border-gray-300 rounded-md p-1"
                    placeholder="Update paid amount"
                    bind:value=paid_amount
                />
                <button
                    type="submit"
                    class="bg-green-500 text-white rounded-md p-2"
                >
                    "Submit"
                </button>
            </form>
            // <form class="flex flex-row gap-2">
            //     <input type="text" class="border-2 border-gray-300 rounded-md p-1" placeholder="Update received amount"/>
            //     <button type="submit" class="bg-green-500 text-white rounded-md p-2">"Submit"</button>
            // </form>

            <h2 class="text-xl">"Other's balance"</h2>
            <table class="w-full">
                <thead>
                    <tr>
                        <th>"Name"</th>
                        <th>"Paid"</th>
                        <th>"Order"</th>
                        <th>"Delta"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>"Alice"</td>
                        <td>"10€"</td>
                        <td>"8€"</td>
                        <td>"2€"</td>
                        <td><button class="bg-blue-500 text-white rounded-md p-1 ml-2">"Paid"</button></td>
                    </tr>
                    <tr>
                        <td>"Bob"</td>
                        <td>"10€"</td>
                        <td>"8€"</td>
                        <td>"2€"</td>
                        <td><button class="bg-blue-500 text-white rounded-md p-1 ml-2">"Paid"</button></td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}
