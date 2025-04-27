use std::{
    fmt::{self, Display, Formatter},
    ops::{Add, Neg, Sub},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Money {
    amount_cents: i32,
}

impl Money {
    pub fn from_cents(cents: i32) -> Self {
        Money {
            amount_cents: cents,
        }
    }

    pub fn from_euros_and_cents(euros: i32, cents: i32) -> Self {
        Money {
            amount_cents: euros * 100 + cents,
        }
    }

    pub fn from_euros_lossy(euros: f32) -> Self {
        Money {
            amount_cents: (euros * 100.0).round() as i32,
        }
    }

    pub fn to_euros_and_cents(&self) -> (i32, i32) {
        let euros = self.amount_cents / 100;
        let cents = self.amount_cents % 100;
        (euros, cents)
    }
    pub fn to_euros_lossy(&self) -> f32 {
        self.amount_cents as f32 / 100.0
    }
}

impl Display for Money {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (euros, cents) = self.to_euros_and_cents();
        write!(f, "{euros}.{cents:02}")
    }
}

impl Add for Money {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Money {
            amount_cents: self.amount_cents + other.amount_cents,
        }
    }
}

impl Sub for Money {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Money {
            amount_cents: self.amount_cents - other.amount_cents,
        }
    }
}

impl Neg for Money {
    type Output = Self;

    fn neg(self) -> Self {
        Money {
            amount_cents: -self.amount_cents,
        }
    }
}
