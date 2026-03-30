use serde::{Deserialize, Serialize, Serializer};
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

/// A decimal number in fixed-point representation with 4 decimal digits.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Fixed4(i64);

impl From<f64> for Fixed4 {
    fn from(value: f64) -> Self {
        Self((value * 10_000.0).round() as i64)
    }
}

impl From<Fixed4> for f64 {
    fn from(value: Fixed4) -> Self {
        (value.0 as f64) / 10_000.0
    }
}

impl Neg for Fixed4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Add for Fixed4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Fixed4 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Fixed4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Fixed4 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Serialize for Fixed4 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Format number as f64 with at most 4 decimal digits.
        let number: f64 = (*self).into();
        let formatted = format!("{:.4}", number);
        serializer.serialize_str(&formatted)
    }
}

impl<'de> Deserialize<'de> for Fixed4 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Deserialize from a f64 or an empty string.
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            Ok(Self(0))
        } else {
            s.parse::<f64>()
                .map(Self::from)
                .map_err(serde::de::Error::custom)
        }
    }
}
