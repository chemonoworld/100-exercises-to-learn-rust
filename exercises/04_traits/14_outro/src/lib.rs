// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::{fmt::Display, ops::Add, u16};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SaturatingU16 {
    inner: u16,
}

impl SaturatingU16 {
    pub fn new(from: u16) -> Self {
        SaturatingU16 { inner: from }
    }
}

impl Display for SaturatingU16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.inner.to_string().as_str())
    }
}

impl Add<SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: SaturatingU16) -> Self::Output {
        let added = self.inner.saturating_add(rhs.inner);
        SaturatingU16 { inner: added }
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        let added = self.inner.saturating_add(rhs.inner);
        SaturatingU16 { inner: added }
    }
}

impl Add<SaturatingU16> for u16 {
    type Output = SaturatingU16;

    fn add(self, rhs: SaturatingU16) -> Self::Output {
        let added = self.saturating_add(rhs.inner);
        SaturatingU16 { inner: added }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        SaturatingU16 {
            inner: value.into(),
        }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        SaturatingU16 {
            inner: (*value).into(),
        }
    }
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16 { inner: value }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        SaturatingU16 { inner: *value }
    }
}

// impl From<SaturatingU16> for u16 {
//     fn from(value: SaturatingU16) -> Self {
//         value.inner
//     }
// }

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.inner == *other
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: u16) -> Self::Output {
        SaturatingU16 {
            inner: self.inner + rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s16 = SaturatingU16::new(111);
        let s16_clone = s16.clone();
        print!("{}", s16);
        assert_eq!(111, s16_clone);
    }
}
