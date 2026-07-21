use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FieldElement<const P: u64> {
    pub value: u64,
}

//During initialization we make sure the value falls into (0, p-1) through reaminder or euclidian
//remainder in case of negative values being a possibility
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

//Since we create a new FieldElement the range will always fall in (0, P-1)
impl<const P: u64> Add for FieldElement<P> {
    type Output = Self;
    fn add(self, element: Self) -> Self::Output {
        Self::from_u64(self.value + element.value)
    }
}

//Since we create a new FieldElement the range will always fall in (0, P-1)
impl<const P: u64> Sub for FieldElement<P> {
    type Output = Self;
    fn sub(self, element: Self) -> Self::Output {
        Self::from_i64((self.value as i64 - element.value as i64) as i64)
    }
}

//For learning purposes we use u64 and u128 to avoid multiplication overflwow
impl<const P: u64> Mul for FieldElement<P> {
    type Output = Self;
    fn mul(self, element: Self) -> Self::Output {
        let res = (self.value as u128 * element.value as u128) % P as u128;
        Self::from_u64(res as u64)
    }
}

/// We actually multiply by one of the inverse
impl<const P: u64> Div for FieldElement<P> {
    type Output = Self;
    fn div(self, element: Self) -> Self::Output {
        if element.value == 0 {
            panic!("Atempting to divide by zero");
        } else {
            match mod_inverse(element.value, P) {
                Some(x) => {
                    let res = x * self.value;
                    return FieldElement::from_u64(res);
                }
                None => panic!("Error calculating euclidian inverse"),
            };
        }
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
        t = t + (p as i64);
    }
    Some(t as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Define a concrete field for testing: F_13
    type F13 = FieldElement<13>;
    type F23 = FieldElement<23>;

    #[test]
    fn test_normalization_negative() {
        // -1 mod 13 should be 12
        let a = F13::from_i64(-1);
        assert_eq!(a.value, 12);

        // -14 mod 13 should be 12
        let b = F13::from_i64(-14);
        assert_eq!(b.value, 12);
    }

    #[test]
    fn test_addition_closure() {
        let a = F13::from_u64(7);
        let b = F13::from_u64(8);
        let c = a + b;
        // 7 + 8 = 15 ≡ 2 mod 13
        assert_eq!(c.value, 2);
    }

    #[test]
    fn test_multiplication_and_inverse() {
        let a = F23::from_u64(7);
        let b = F23::from_u64(10);
        let product = a * b;

        // Verify inverse: (a * b) * (a * b)^-1 = 1
        if product.value != 0 {
            let inv = F23::from_u64(mod_inverse(product.value, 23).unwrap());
            let one = product * inv;
            assert_eq!(one.value, 1);
        }
    }

    #[test]
    #[should_panic(expected = "Atempting to divide by zero")]
    fn test_division_by_zero_behavior() {
        let a = F13::from_u64(5);
        let zero = F13::from_u64(0);

        let _ = a / zero;
    }
}
