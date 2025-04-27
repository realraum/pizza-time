mod money;

use serde::{Deserialize, Serialize};

use money::Money;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pizza {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: Money,
}

impl Pizza {
    pub fn from_sus_pizza(sus_pizza: SusPizza) -> Self {
        Pizza {
            id: sus_pizza.id.clone(),
            name: sus_pizza.name.clone(),
            description: sus_pizza.description.clone(),
            price: money_from_sus_str(&sus_pizza.price).unwrap(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SusPizza {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ParseError {
    InvalidFormat,
    InvalidInt,
}

fn money_from_sus_str(s: &str) -> Result<Money, ParseError> {
    // Assume the string is in the format "X,YY"
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return Err(ParseError::InvalidFormat);
    }
    let euros = parts[0]
        .parse::<i32>()
        .map_err(|_| ParseError::InvalidInt)?;
    let cents = parts[1]
        .parse::<i32>()
        .map_err(|_| ParseError::InvalidInt)?;

    Ok(Money::from_euros_and_cents(euros, cents))
}
