use serde::{Deserialize, Serialize};

use super::{money::Money, Pizza};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u16,
    pub name: String,
    pub order: Vec<Pizza>,
    pub paid_amount: Money,
    pub received_amount: Money,
}

impl User {
    pub fn new(id: u16, name: String) -> Self {
        User {
            id,
            name,
            order: Vec::new(),
            paid_amount: Money::from_cents(0),
            received_amount: Money::from_cents(0),
        }
    }
}
