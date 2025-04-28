use std::{collections::BTreeMap, sync::Mutex};

// TODO consider: use std::cell::OnceCell;
use once_cell::sync::OnceCell;

use crate::common::users::User;

pub static USERS: OnceCell<&'static Mutex<BTreeMap<u16, User>>> = OnceCell::new();

macro_rules! get_user_id_and_create_if_required {
    () => {{
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

        uid
    }};
}

pub(crate) use get_user_id_and_create_if_required;
