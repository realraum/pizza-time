use std::collections::BTreeMap;

use leptos::{ev::SubmitEvent, prelude::*, task::spawn_local};

#[cfg(feature = "ssr")]
use crate::server::{get_user_id_and_create_if_required, USERS};
use crate::{
    app::components::PersonCard,
    common::{dedup_and_count, money::Money, users::User},
};

#[server(GetUsers, endpoint = "get_users")]
pub async fn get_users() -> Result<(BTreeMap<u16, User>, u16), ServerFnError> {
    let (users, uid) = get_user_id_and_create_if_required!();

    Ok((users.clone(), uid))
}

#[server(SetName, endpoint = "set_name")]
async fn set_name(name: String) -> Result<(), ServerFnError> {
    let (mut users, uid) = get_user_id_and_create_if_required!();

    if let Some(user) = users.get_mut(&uid) {
        user.name = name;
    } else {
        return Err(ServerFnError::ServerError("User not found".to_string()));
    }

    Ok(())
}

/// Removes a pizza by pid (pizza id)
#[server(RmPizza, endpoint = "rm_pizza")]
async fn rm_pizza(pid: String) -> Result<(), ServerFnError> {
    let (mut users, uid) = get_user_id_and_create_if_required!();

    if let Some(user) = users.get_mut(&uid) {
        user.order.retain(|p| p.id != pid);
    } else {
        return Err(ServerFnError::ServerError("User not found".to_string()));
    }

    Ok(())
}

#[component]
pub fn Summary() -> impl IntoView {
    let update_signal = RwSignal::new(true);
    let users = Resource::new(
        move || update_signal.get(),
        async move |_| {
            let users = get_users().await;

            users.map(|users| (users.0.values().cloned().collect::<Vec<_>>(), users.1))
        },
    );

    let my_name = RwSignal::new(String::new());

    let set_name = move |e: SubmitEvent| {
        e.prevent_default();
        let name = my_name.get();
        spawn_local(async {
            set_name(name).await.unwrap();
        });
        update_signal.set(!update_signal.get());
    };

    view! {
        <Suspense
            fallback=|| view! { <p>"Loading data..."</p> }
        >
            {move || {
                users.get()
                    .map(move |users| {
                        let (users, my_uid)= users.unwrap();

                        let me = users. iter()
                            .find(|user| user.id == my_uid)
                            .unwrap().clone();

                        my_name.set(me.name.clone());

                        let others = users.iter()
                            .filter(|user| user.id != my_uid)
                            .cloned()
                            .collect::<Vec<_>>();

                        let my_name_2 = me.name.clone();

                        view! {
                            // <p>
                            //     {format!("{:?}", users)}
                            // </p>

                            <div class="flex flex-col gap-2 w-full">
                                <form
                                    on:submit=set_name
                                    class="flex flex-row w-full gap-2"
                                >
                                    // <p>"Your name"</p>
                                    <input
                                        type="text"
                                        class="border-2 border-gray-300 rounded-md p-1 w-full dark:bg-gray-700 dark:text-gray-200 dark:border-gray-600"
                                        placeholder="Enter your name"
                                        bind:value=my_name
                                    />
                                    <button
                                        type="submit"
                                        // type="button"
                                        class="bg-green-500 text-white rounded-md p-2"
                                        // on:click=set_name
                                    >
                                        "Submit"
                                    </button>
                                </form>

                                <button>
                                    "New session"
                                </button>

                                // TODO replace this div with a (specialized?) `PersonCard` component
                                <div class="flex flex-col bg-green-50 rounded p-1 dark:bg-gray-700 dark:text-gray-200">
                                    <h2 class="text-xl">"Your selection"</h2>
                                    <p class="test-sm">"Your name: "{me.name}</p>
                                    <table>
                                        <tbody>
                                            {
                                                let order = dedup_and_count(me.order.clone());
                                                order.into_iter()
                                                    .map(|(pizza, count)| {
                                                        view! {
                                                            <tr>
                                                                <td>{format!("{}x", count)}</td>
                                                                <td>{pizza.name}</td>
                                                                <td>{format!("@ {}€", pizza.price.to_string())}</td>
                                                                <td>
                                                                    <button
                                                                        class="bg-red-500 text-white rounded-md p-1 ml-2"
                                                                        on:click=move |_| {
                                                                            let pid = pizza.id.clone();
                                                                            spawn_local(async move {
                                                                                rm_pizza(pid).await.unwrap();
                                                                            });
                                                                            update_signal.set(!update_signal.get());
                                                                        }
                                                                    >
                                                                        "Remove"
                                                                    </button>
                                                                </td>
                                                            </tr>
                                                        }
                                                    })
                                                    .collect::<Vec<_>>()
                                            }
                                        </tbody>
                                    </table>
                                </div>

                                <h2 class="text-xl">"Your selection"</h2>
                                <div class="sm:rounded-xl sm:overflow-clip mt-2 sm:mt-4">
                                    <div class="grid grid-cols-1">
                                        <PersonCard name=my_name_2.clone() pizzas=Vec::new() />
                                    </div>
                                </div>
                                <h2 class="text-xl">"Other's selection"</h2>
                                <div class="sm:rounded-xl sm:overflow-clip mt-2 sm:mt-4">
                                    <div class="grid grid-cols-1">
                                        <PersonCard name="Alice".into() pizzas=Vec::new() />
                                        <PersonCard name="Alice".into() pizzas=Vec::new() />
                                        <PersonCard name="Alice".into() pizzas=Vec::new() />
                                        <PersonCard name="Alice".into() pizzas=Vec::new() />
                                    </div>
                                </div>
                                // </div>
                            </div>
                        }
                    })
                }
            }
        </ Suspense>
    }
}
