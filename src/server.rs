use std::{collections::BTreeMap, sync::Mutex};

// TODO consider: use std::cell::OnceCell;
use once_cell::sync::OnceCell;

use crate::common::users::User;

pub static USERS: OnceCell<&'static Mutex<BTreeMap<u16, User>>> = OnceCell::new();
