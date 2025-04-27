mod money;

use serde::{Deserialize, Serialize};

use money::Money;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pizza {
    pub name: String,
    pub description: String,
    pub price: Money,
}
