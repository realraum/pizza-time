use std::collections::BTreeMap;

use leptos::{prelude::*, task::spawn_local};

use crate::common::{money::Money, users::User};
#[cfg(feature = "ssr")]
use crate::server::USERS;

#[server(GetUsers, endpoint = "get_users")]
async fn get_users() -> Result<(BTreeMap<u16, User>, u16), ServerFnError> {
    use axum_extra::extract::cookie::{Cookie, SameSite};
    use http::{header::SET_COOKIE, HeaderMap};
    use leptos_axum::extract;

    let (headers,): (HeaderMap,) = extract().await?;

    let users = *USERS.get().unwrap();
    let mut users = users.lock().unwrap();

    let mut uid: Option<u16> = if let Some(cookie_header) = headers.get("cookie") {
        let mut uid = None;
        for cookie in Cookie::split_parse_encoded(cookie_header.to_str().unwrap()) {
            let cookie = cookie.unwrap();
            match cookie.name() {
                "r3-pizza-time_user-id" => {
                    uid = cookie.value().parse().ok();
                    break;
                }
                _ => (),
            }
        }
        uid
    } else {
        None
    };

    if uid.is_none() || !users.contains_key(&uid.unwrap()) {
        loop {
            let new_uid = rand::random::<u16>();
            if !users.contains_key(&new_uid) {
                let new_user = User {
                    id: new_uid,
                    name: String::new(),
                    order: Vec::new(),
                    paid_amount: Money::from_cents(0),
                    received_amount: Money::from_cents(0),
                };
                users.insert(new_uid, new_user);
                uid = Some(new_uid);
                break;
            }
        }
    }

    let uid = uid.unwrap();

    let opts = expect_context::<leptos_axum::ResponseOptions>();
    let cookie = Cookie::build(("r3-pizza-time_user-id", uid.to_string()))
        .path("/")
        .http_only(false)
        .secure(false)
        .same_site(SameSite::Lax);
    opts.insert_header(SET_COOKIE, cookie.to_string().try_into().unwrap());

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
