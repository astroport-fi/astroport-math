use crate::astroport::cosmwasm_ext::AbsDiff;
use cosmwasm_std::{Decimal256, StdError};
use std::fmt::{Display, Formatter};
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignedDecimal256 {
    val: Decimal256,
    /// false - positive, true - negative
    neg: bool,
}

impl Display for SignedDecimal256 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let sign = if self.neg { "-" } else { "" };
        f.write_str(&format!("{sign}{}", self.val))
    }
}

impl SignedDecimal256 {
    pub fn new(val: Decimal256, neg: bool) -> Self {
        Self { val, neg }
    }
    pub fn pow(&self, exp: u32) -> Self {
        if self.val.is_zero() {
            Self::from(Decimal256::zero())
        } else {
            let neg = if exp % 2 == 0 { false } else { self.neg };
            Self {
                val: self.val.pow(exp),
                neg,
            }
        }
    }
    pub fn diff(self, other: SignedDecimal256) -> Decimal256 {
        if self.neg == other.neg {
            self.val.diff(other.val)
        } else {
            self.val + other.val
        }
    }
}

impl From<Decimal256> for SignedDecimal256 {
    fn from(val: Decimal256) -> Self {
        Self { val, neg: false }
    }
}

impl From<&Decimal256> for SignedDecimal256 {
    fn from(val: &Decimal256) -> Self {
        Self::from(*val)
    }
}

impl TryInto<Decimal256> for SignedDecimal256 {
    type Error = StdError;

    fn try_into(self) -> Result<Decimal256, Self::Error> {
        if !self.neg || self.val.is_zero() {
            Ok(self.val)
        } else {
            Err(StdError::generic_err(format!(
                "Unable to convert negative value, {}",
                self
            )))
        }
    }
}

impl ops::Add for SignedDecimal256 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.neg == rhs.neg {
            Self {
                val: self.val + rhs.val,
                ..self
            }
        } else if self.val > rhs.val {
            Self {
                val: self.val - rhs.val,
                ..self
            }
        } else {
            Self {
                val: rhs.val - self.val,
                ..rhs
            }
        }
    }
}

impl ops::Add<Decimal256> for SignedDecimal256 {
    type Output = SignedDecimal256;

    fn add(self, rhs: Decimal256) -> Self::Output {
        self + SignedDecimal256::from(rhs)
    }
}

impl ops::Add<SignedDecimal256> for Decimal256 {
    type Output = SignedDecimal256;

    fn add(self, rhs: SignedDecimal256) -> Self::Output {
        rhs + self
    }
}

impl ops::Sub for SignedDecimal256 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + Self {
            neg: !rhs.neg,
            ..rhs
        }
    }
}

impl ops::Sub<Decimal256> for SignedDecimal256 {
    type Output = SignedDecimal256;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn sub(self, rhs: Decimal256) -> Self::Output {
        self + Self {
            val: rhs,
            neg: true,
        }
    }
}

impl ops::Sub<SignedDecimal256> for Decimal256 {
    type Output = SignedDecimal256;

    fn sub(self, rhs: SignedDecimal256) -> Self::Output {
        SignedDecimal256::from(self) - rhs
    }
}

impl ops::Mul<Decimal256> for SignedDecimal256 {
    type Output = SignedDecimal256;

    fn mul(self, rhs: Decimal256) -> Self::Output {
        Self {
            val: self.val * rhs,
            ..self
        }
    }
}

impl ops::Mul for SignedDecimal256 {
    type Output = SignedDecimal256;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            val: self.val * rhs.val,
            neg: self.neg ^ rhs.neg,
        }
    }
}

impl ops::Mul<SignedDecimal256> for Decimal256 {
    type Output = SignedDecimal256;

    fn mul(self, rhs: SignedDecimal256) -> Self::Output {
        rhs * self
    }
}

impl ops::Div for SignedDecimal256 {
    type Output = SignedDecimal256;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            val: self.val / rhs.val,
            neg: self.neg ^ rhs.neg,
        }
    }
}

impl ops::Div<Decimal256> for SignedDecimal256 {
    type Output = SignedDecimal256;

    fn div(self, rhs: Decimal256) -> Self::Output {
        self / SignedDecimal256::from(rhs)
    }
}

impl ops::Div<SignedDecimal256> for Decimal256 {
    type Output = SignedDecimal256;

    fn div(self, rhs: SignedDecimal256) -> Self::Output {
        Self::Output {
            val: self / rhs.val,
            neg: rhs.neg,
        }
    }
}

impl ops::Neg for SignedDecimal256 {
    type Output = SignedDecimal256;

    fn neg(self) -> Self::Output {
        Self {
            neg: !self.neg,
            ..self
        }
    }
}
