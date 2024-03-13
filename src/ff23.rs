use num::Integer;
use std::ops::{Add, Mul, Neg, Sub};

/// finite field 𝔽23 is the list of numbers 0 through 22
/// Note: this is a very simple and naive implementation
#[derive(Debug)]
pub struct FiniteField23Point {
    value: u8,
}

impl FiniteField23Point {
    pub fn min() -> Self {
        Self { value: 0 }
    }

    pub fn max() -> Self {
        Self { value: 22 }
    }

    pub(crate) fn try_new(value: u8) -> Result<Self, &'static str> {
        if value < 23 {
            Ok(Self { value })
        } else {
            Err("Higher than 23")
        }
    }

    fn new(value: u8) -> Self {
        if value < 23 {
            Self { value }
        } else {
            panic!("Cannot init a Finite Field 23 Point with value higher than 22");
        }
    }

    fn multiplication_inverse(&self) -> Self {
        // from: https://curves.xargs.org/inverse23.html
        let res: u8 = match self.value {
            1 => 1,   // 1 × 1 = 1 mod 23 = 1
            2 => 12,  // 2 × 12 = 24 mod 23 = 1
            3 => 8,   // 3 × 8 = 24 mod 23 = 1
            4 => 6,   // 4 × 6 = 24 mod 23 = 1
            5 => 14,  // 5 × 14 = 70 mod 23 = 1
            6 => 4,   // 6 × 4 = 24 mod 23 = 1
            7 => 10,  // 7 × 10 = 70 mod 23 = 1
            8 => 3,   // 8 × 3 = 24 mod 23 = 1
            9 => 18,  // 9 × 18 = 162 mod 23 = 1
            10 => 7,  // 10 × 7 = 70 mod 23 = 1
            11 => 21, // 11 × 21 = 231 mod 23 = 1
            12 => 2,  // 12 × 2 = 24 mod 23 = 1
            13 => 16, // 13 × 16 = 208 mod 23 = 1
            14 => 5,  // 14 × 5 = 70 mod 23 = 1
            15 => 20, // 15 × 20 = 300 mod 23 = 1
            16 => 13, // 16 × 13 = 208 mod 23 = 1
            17 => 19, // 17 × 19 = 323 mod 23 = 1
            18 => 9,  // 18 × 9 = 162 mod 23 = 1
            19 => 17, // 19 × 17 = 323 mod 23 = 1
            20 => 15, // 20 × 15 = 300 mod 23 = 1
            21 => 11, // 21 × 11 = 231 mod 23 = 1
            22 => 22, // 22 × 22 = 484 mod 23 = 1
            _ => {
                unreachable!()
            }
        };

        FiniteField23Point::new(res)
    }

    fn square_root(&self) -> Option<(Self, Self)> {
        // Definition for: sqrt(n)
        //                 sqrt(n) * sqrt(n) = n
        let res: Option<(u8, u8)> = match self.value {
            0 => Some((0, 0)),
            1 => Some((1, 22)),
            2 => Some((5, 18)),
            3 => Some((7, 16)),
            4 => Some((2, 21)),
            5 => None,
            6 => Some((11, 12)),
            7 => None,
            8 => Some((10, 13)),
            9 => Some((3, 20)),
            10 => None,
            11 => None,
            12 => Some((9, 14)),
            13 => Some((6, 17)),
            14 => None,
            15 => None,
            16 => Some((4, 19)),
            17 => None,
            18 => Some((8, 15)),
            19 => None,
            20 => None,
            21 => None,
            22 => None,
            _ => unreachable!(),
        };

        res.map(|(v1, v2)| (FiniteField23Point::new(v1), FiniteField23Point::new(v2)))
    }
}

impl Add for FiniteField23Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            value: (self.value + other.value).mod_floor(&23),
        }
    }
}

impl Sub for FiniteField23Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        // safe to unwrap -> [0; 22]
        let v1 = i8::try_from(self.value).unwrap();
        // safe to unwrap -> [0; 22]
        let v2 = i8::try_from(other.value).unwrap();
        let v = (v1 - v2).mod_floor(&23);

        Self {
            // Safe to unwrap as the result of the subtract is modulo 23
            value: u8::try_from(v).unwrap(),
        }
    }
}

/*
impl Mul for &FiniteField23Point {
    type Output = Self;

    fn mul(self, other: &Self) -> Self {
        // Need to use u16 here as 22*22 = 484

        let v1 = u16::from(self.value);
        let v2 = u16::from(other.value);
        let v = (v1 * v2).mod_floor(&23);

        Self {
            // Safe to unwrap as the result of the multiplication is module 23
            value: u8::try_from(v).unwrap(),
        }
    }
}
*/

impl Mul for FiniteField23Point {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        // Need to use u16 here as 22*22 = 484

        let v1 = u16::from(self.value);
        let v2 = u16::from(other.value);
        let v = (v1 * v2).mod_floor(&23);

        Self {
            // Safe to unwrap as the result of the multiplication is module 23
            value: u8::try_from(v).unwrap(),
        }
    }
}

impl Mul for &FiniteField23Point {
    type Output = FiniteField23Point;

    fn mul(self, other: &Self::Output) -> Self::Output {
        // Need to use u16 here as 22*22 = 484

        let v1 = u16::from(self.value);
        let v2 = u16::from(other.value);
        let v = (v1 * v2).mod_floor(&23);

        FiniteField23Point {
            // Safe to unwrap as the result of the multiplication is module 23
            value: u8::try_from(v).unwrap(),
        }
    }
}

impl Neg for FiniteField23Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        // Definition for: -n
        // n + (-n) = 0

        // safe to unwrap -> [0; 22]
        let v1 = i8::try_from(self.value).unwrap();
        let v2 = 23;
        let v = (v2 - v1).mod_floor(&23);

        Self {
            // Safe to unwrap as the result of the subtract is modulo 23
            value: u8::try_from(v).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl PartialEq for FiniteField23Point {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value
        }
    }

    #[test]
    fn test_ff23_add() {
        assert_eq!(
            FiniteField23Point::new(22) + FiniteField23Point::new(12),
            FiniteField23Point::new(11)
        );
        assert_eq!(
            FiniteField23Point::new(22) + FiniteField23Point::new(19),
            FiniteField23Point::new(18)
        );
    }

    #[test]
    fn test_ff23_sub() {
        assert_eq!(
            FiniteField23Point::min() - FiniteField23Point::new(18),
            FiniteField23Point::new(5)
        );
        assert_eq!(
            FiniteField23Point::new(1) - FiniteField23Point::new(22),
            FiniteField23Point::new(2)
        );
        assert_eq!(
            FiniteField23Point::new(18) - FiniteField23Point::new(20),
            FiniteField23Point::new(21)
        );
    }

    #[test]
    fn test_ff23_mul() {
        assert_eq!(
            FiniteField23Point::new(21) * FiniteField23Point::new(14),
            FiniteField23Point::new(18)
        );
        assert_eq!(
            FiniteField23Point::new(16) * FiniteField23Point::new(22),
            FiniteField23Point::new(7)
        );
        assert_eq!(
            FiniteField23Point::new(15) * FiniteField23Point::new(8),
            FiniteField23Point::new(5)
        );
    }

    #[test]
    fn test_ff23_neg() {
        assert_eq!(-FiniteField23Point::new(20), FiniteField23Point::new(3));

        assert_eq!(-FiniteField23Point::new(6), FiniteField23Point::new(17));

        assert_eq!(-FiniteField23Point::new(8), FiniteField23Point::new(15));

        assert_eq!(-FiniteField23Point::new(10), FiniteField23Point::new(13));
    }

    #[test]
    fn test_multiplication_inverse() {
        for i in 1..22 {
            let v = FiniteField23Point::new(i);
            let iv = v.multiplication_inverse();
            assert_eq!(v * iv, FiniteField23Point::new(1));
        }
    }

    #[test]
    fn test_square_roots() {
        for i in 1..22 {
            let v = FiniteField23Point::new(i);
            if let Some((v_sqr_1, v_sqr_2)) = v.square_root() {
                assert_eq!(&v_sqr_1 * &v_sqr_1, v);
                assert_eq!(&v_sqr_2 * &v_sqr_2, v);
            }
        }
    }
}
