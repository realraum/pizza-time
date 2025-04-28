use std::collections::BTreeMap;

use leptos::{prelude::*, task::spawn_local};

use crate::common::{money::Money, users::User};
#[cfg(feature = "ssr")]
use crate::server::{get_user_id_and_create_if_required, USERS};

#[server(GetUsers, endpoint = "get_users")]
async fn get_users() -> Result<(BTreeMap<u16, User>, u16), ServerFnError> {
    let uid = get_user_id_and_create_if_required!();

    let users = *USERS.get().unwrap();
    let users = users.lock().unwrap();

    Ok((users.clone(), uid))
}

#[server(SetName, endpoint = "set_name")]
async fn set_name(name: String) -> Result<(), ServerFnError> {
    println!("Setting name to {name}");

    return Ok(()); // TODO implement the name setting fn
}

#[component]
pub fn Summary() -> impl IntoView {
    let users = Resource::new(
        move || (),
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
                            <p>
                                {format!("{:?}", users)}
                            </p>

                            <div class="flex flex-col items-center gap-2 w-full">
                                <form class="flex flex-row w-full gap-2">
                                    // <p>"Your name"</p>
                                    <input
                                        type="text"
                                        class="border-2 border-gray-300 rounded-md p-1 w-full"
                                        placeholder="Enter your name"
                                        bind:value=my_name
                                    />
                                    <button
                                        // type="submit"
                                        type="button"
                                        class="bg-green-500 text-white rounded-md p-2"
                                        on:click=move |_| {
                                            let name = my_name.get();
                                            spawn_local(async{set_name(name).await.unwrap();});
                                        }
                                    >
                                        "Submit"
                                    </button>
                                </form>

                                <div class="flex flex-col items-center bg-green-50 rounded p-1">
                                    <h2 class="text-xl">"Your selection"</h2>
                                    <table>
                                        <tbody>
                                            <tr>
                                                <td>"1x"</td>
                                                <td>"QuadFor"</td>
                                                <td>"@ 10€"</td>
                                                <td><button class="bg-red-500 text-white rounded-md p-1 ml-2">"Remove"</button></td>
                                            </tr>
                                            <tr>
                                                <td>"1x"</td>
                                                <td>"Veggi"</td>
                                                <td>"@ 12€"</td>
                                                <td><button class="bg-red-500 text-white rounded-md p-1 ml-2">"Remove"</button></td>
                                            </tr>
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
