use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FieldElement<const P: u64> {
    pub value: u64,
}

impl<const P: u64> FieldElement<P> {
    pub fn from_u64(value: u64) -> Self {
        Self { value: value % P }
    }
    pub fn from_i64(value: i64) -> Self {
        Self {
            value: value.rem_euclid(P as i64) as u64,
        }
    }
}

impl<const P: u64> Add for FieldElement<P> {
    type Output = Self;
    fn add(self, element: Self) -> Self::Output {
        Self::from_u64(self.value + element.value)
    }
}

impl<const P: u64> Sub for FieldElement<P> {
    type Output = Self;
    fn sub(self, element: Self) -> Self::Output {
        Self::from_i64((self.value as i64 - element.value as i64) as i64)
    }
}
