use std::collections::BTreeMap;

use leptos::{ev::SubmitEvent, prelude::*, task::spawn_local};

use crate::common::{dedup_and_count, money::Money, users::User};
#[cfg(feature = "ssr")]
use crate::server::{get_user_id_and_create_if_required, USERS};

#[server(GetUsers, endpoint = "get_users")]
async fn get_users() -> Result<(BTreeMap<u16, User>, u16), ServerFnError> {
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
    Effect::new(move || {
        let name = my_name.get();
        println!("my_name: {:?}", name);
    });

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

                        view! {
                            // <p>
                            //     {format!("{:?}", users)}
                            // </p>

                            <div class="flex flex-col items-center gap-2 w-full">
                                <form
                                    on:submit=set_name
                                    class="flex flex-row w-full gap-2"
                                >
                                    // <p>"Your name"</p>
                                    <input
                                        type="text"
                                        class="border-2 border-gray-300 rounded-md p-1 w-full"
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

                                <div class="flex flex-col items-center bg-green-50 rounded p-1">
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
                                                                <td><button class="bg-red-500 text-white rounded-md p-1 ml-2">"Remove"</button></td>
                                                            </tr>
                                                        }
                                                    })
                                                    .collect::<Vec<_>>()
                                            }
                                        </tbody>
                                    </table>
                                </div>

                                <div class="flex flex-col items-center bg-green-50 rounded p-1">
                                    <h2 class="text-xl">"Other's selection"</h2>
                                    <h4 class="text-lg">"Alice"</h4>
                                    <table>
                                        <tbody>
                                            <tr>
                                                <td>"1x"</td>
                                                <td>"QuadFor"</td>
                                                <td>"@ 10€"</td>
                                            </tr>
                                            <tr>
                                                <td>"1x"</td>
                                                <td>"Veggi"</td>
                                                <td>"@ 12€"</td>
                                            </tr>
                                        </tbody>
                                    </table>
                                    <h4 class="text-lg">"Bob"</h4>
                                    <table>
                                        <tbody>
                                            <tr>
                                                <td>"1x"</td>
                                                <td>"QuadFor"</td>
                                                <td>"@ 10€"</td>
                                            </tr>
                                            <tr>
                                                <td>"1x"</td>
                                                <td>"Veggi"</td>
                                                <td>"@ 12€"</td>
                                            </tr>
                                        </tbody>
                                    </table>
                                </div>
                            </div>
                        }
                    })
                }
            }
        </ Suspense>
    }
}
