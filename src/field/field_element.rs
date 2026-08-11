use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FieldElement<const P: u64> {
    pub value: u64,
}

//During initialization we make sure the value falls into (0, p-1) through reaminder or euclidian
//remainder in case of negative values being a possibility
impl<const P: u64> FieldElement<P> {
    pub fn zero() -> Self {
        Self { value: 0 }
    }

    pub fn one() -> Self {
        Self { value: 1 }
    }

    pub fn from_u64(value: u64) -> Self {
        Self { value: value % P }
    }

    #[allow(dead_code)]
    pub fn from_i64(value: i64) -> Self {
        Self {
            value: value.rem_euclid(P as i64) as u64,
        }
    }
    pub fn inverse(self) -> Self {
        assert!(self.value != 0, "Zero has no inverse");

        match mod_inverse(self.value, P) {
            Some(x) => Self::from_u64(x),
            None => panic!("No inverse exists"),
        }
    }

    // Binary Exponentiation O(logn) instead of O(n)
    pub fn pow(self, mut exp: u64) -> Self {
        let mut base = self;
        let mut result = Self::one();

        while exp > 0 {
            // If the current bit of the exponent is 1, multiply the result by the base
            if exp % 2 == 1 {
                result = result * base;
            }
            // Square the base for the next bit
            base = base * base;
            // Shift exponent to the right by 1 bit (divide by 2)
            // Z13: 13 -> 6 -> 3 -> 1
            //       3¹ -> unchanged -> 3¹.3⁴ -> (3¹.3⁴).3⁸
            exp >>= 1;
        }
        result
    }
}

//Since we create a new FieldElement the range will always fall in (0, P-1) range.
//To avoid overflow we cast the sum to u128.
impl<const P: u64> Add for FieldElement<P> {
    type Output = Self;
    fn add(self, element: Self) -> Self::Output {
        let sum = (self.value as u128 + element.value as u128) % (P as u128);
        Self::from_u64(sum as u64)
    }
}

//Since we use Neg combined with Add methods we are sure it will fall in (0, P-1) range.
impl<const P: u64> Sub for FieldElement<P> {
    type Output = Self;
    fn sub(self, element: Self) -> Self::Output {
        self + (-element)
    }
}

// For learning purposes we use u64 and u128 to avoid multiplication overflow
#[allow(clippy::suspicious_arithmetic_impl)]
impl<const P: u64> Mul for FieldElement<P> {
    type Output = Self;
    fn mul(self, element: Self) -> Self::Output {
        let res = (self.value as u128 * element.value as u128) % P as u128;
        Self::from_u64(res as u64)
    }
}

/// We actually multiply by the inverse
impl<const P: u64> Div for FieldElement<P> {
    type Output = Self;
    fn div(self, element: Self) -> Self::Output {
        self * element.inverse()
    }
}

impl<const P: u64> Neg for FieldElement<P> {
    type Output = Self;

    fn neg(self) -> Self {
        if self.value == 0 {
            Self::zero()
        } else {
            Self::from_u64(P - self.value)
        }
    }
}

impl<const P: u64> Default for FieldElement<P> {
    fn default() -> Self {
        Self::zero()
    }
}

// Display
impl<const P: u64> Display for FieldElement<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// Extended Euclidian Algorithm
fn mod_inverse(e: u64, p: u64) -> Option<u64> {
    //coeficient of e
    let (mut t, mut new_t) = (0, 1);
    //GCD(p, e)
    let (mut r, mut new_r) = (p as i64, e as i64);

    while new_r != 0 {
        let quotient = r / new_r;

        // Update t
        let temp_t = t;
        t = new_t;
        new_t = temp_t - quotient * new_t;

        // Update r
        let temp_r = r;
        r = new_r;
        new_r = temp_r - quotient * new_r;
    }

    // If gcd > 1, inverse doesn't exist
    if r > 1 {
        return None;
    }

    if t < 0 {
        t += p as i64;
    }
    Some(t as u64)
}

#[cfg(test)]
mod tests {
    use proptest::proptest;

    use super::*;

    type F13 = FieldElement<13>;
    type F23 = FieldElement<23>;

    #[test]
    fn test_normalization_negative() {
        assert_eq!(F13::from_i64(-1), F13::from_u64(12));
        assert_eq!(F13::from_i64(-14), F13::from_u64(12));

        assert_eq!(F13::from_i64(0), F13::zero());
        assert_eq!(F13::from_i64(13), F13::zero());
        assert_eq!(F13::from_i64(26), F13::zero());
        assert_eq!(F13::from_i64(-26), F13::zero());
    }

    #[test]
    fn test_addition_closure() {
        let a = F13::from_u64(7);
        let b = F13::from_u64(8);
        let c = a + b;
        // 7 + 8 = 15 ≡ 2 mod 13
        assert_eq!(c, F13::from_u64(2));
    }

    proptest! {
        #[test]
        fn every_nonzero_element_has_an_inverse(a in 1u64..23) {
            let a = F23::from_u64(a);
            assert_eq!(a * a.inverse(), F23::one());
        }
    }

    #[test]
    #[should_panic(expected = "Zero has no inverse")]
    fn test_division_by_zero_behavior() {
        let a = F13::from_u64(5);
        let zero = F13::from_u64(0);

        let _ = a / zero;
    }

    #[test]
    fn test_division() {
        let a = F23::from_u64(15);
        let b = F23::from_u64(7);

        assert_eq!((a / b) * b, a);
    }
}
