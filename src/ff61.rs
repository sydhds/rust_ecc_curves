use num::Integer;
use std::ops::{Add, Mul, Neg, Sub};

pub(crate) trait FiniteField: Add + Sub + Mul + Neg + PartialEq + Sized {
    const P: u8;
    // const P_MINUS_1: u8;
    // Nightly required (1.77.0 nightly): https://github.com/rust-lang/rust/issues/88674
    const SIGNED_P: i8 = Self::P as i8;
    const P_U16: u16 = Self::P as u16;

    type ValueType;

    fn new(value: Self::ValueType) -> Self;

    fn min() -> Self;
    fn max() -> Self;

    fn value(&self) -> Self::ValueType;

    // fn mul_inverse_0(&self) -> Self;
    fn mul_inverse(&self) -> Self;
    fn square_root(&self) -> Option<(Self, Self)>;
}

/// finite field 𝔽61 is the list of numbers 0 through 60
#[derive(Debug, Clone, PartialEq)]
pub struct FiniteField61Point {
    value: <FiniteField61Point as FiniteField>::ValueType,
}

impl FiniteField for FiniteField61Point {
    const P: u8 = 61;

    type ValueType = u8;

    fn new(value: Self::ValueType) -> Self {
        if value < Self::P {
            Self { value }
        } else {
            panic!("")
        }
    }

    fn min() -> Self {
        Self { value: 0 }
    }

    fn max() -> Self {
        Self { value: Self::P - 1 }
    }

    fn value(&self) -> Self::ValueType {
        self.value
    }

    fn mul_inverse(&self) -> Self {
        // Definition for: mul_inv(n) (== 1/n)
        //                 n * mul_inv(n) = 1

        // from: https://curves.xargs.org/inverse61.html
        let res: u8 = match self.value {
            1 => 1,   // 1 × 1 = 1 mod 61 = 1
            2 => 31,  // 2 × 31 = 62 mod 61 = 1
            3 => 41,  // 3 × 41 = 123 mod 61 = 1
            4 => 46,  // 4 × 46 = 184 mod 61 = 1
            5 => 49,  // 5 × 49 = 245 mod 61 = 1
            6 => 51,  // 6 × 51 = 306 mod 61 = 1
            7 => 35,  // 7 × 35 = 245 mod 61 = 1
            8 => 23,  // 8 × 23 = 184 mod 61 = 1
            9 => 34,  // 9 × 34 = 306 mod 61 = 1
            10 => 55, // 10 × 55 = 550 mod 61 = 1
            11 => 50, // 11 × 50 = 550 mod 61 = 1
            12 => 56, // 12 × 56 = 672 mod 61 = 1
            13 => 47, // 13 × 47 = 611 mod 61 = 1
            14 => 48, // 14 × 48 = 672 mod 61 = 1
            15 => 57, // 15 × 57 = 855 mod 61 = 1
            16 => 42, // 16 × 42 = 672 mod 61 = 1
            17 => 18, // 17 × 18 = 306 mod 61 = 1
            18 => 17, // 18 × 17 = 306 mod 61 = 1
            19 => 45, // 19 × 45 = 855 mod 61 = 1
            20 => 58, // 20 × 58 = 1160 mod 61 = 1
            21 => 32, // 21 × 32 = 672 mod 61 = 1
            22 => 25, // 22 × 25 = 550 mod 61 = 1
            23 => 8,  // 23 × 8 = 184 mod 61 = 1
            24 => 28, // 24 × 28 = 672 mod 61 = 1
            25 => 22, // 25 × 22 = 550 mod 61 = 1
            26 => 54, // 26 × 54 = 1404 mod 61 = 1
            27 => 52, // 27 × 52 = 1404 mod 61 = 1
            28 => 24, // 28 × 24 = 672 mod 61 = 1
            29 => 40, // 29 × 40 = 1160 mod 61 = 1
            30 => 59, // 30 × 59 = 1770 mod 61 = 1
            31 => 2,  // 31 × 2 = 62 mod 61 = 1
            32 => 21, // 32 × 21 = 672 mod 61 = 1
            33 => 37, // 33 × 37 = 1221 mod 61 = 1
            34 => 9,  // 34 × 9 = 306 mod 61 = 1
            35 => 7,  // 35 × 7 = 245 mod 61 = 1
            36 => 39, // 36 × 39 = 1404 mod 61 = 1
            37 => 33, // 37 × 33 = 1221 mod 61 = 1
            38 => 53, // 38 × 53 = 2014 mod 61 = 1
            39 => 36, // 39 × 36 = 1404 mod 61 = 1
            40 => 29, // 40 × 29 = 1160 mod 61 = 1
            41 => 3,  // 41 × 3 = 123 mod 61 = 1
            42 => 16, // 42 × 16 = 672 mod 61 = 1
            43 => 44, // 43 × 44 = 1892 mod 61 = 1
            44 => 43, // 44 × 43 = 1892 mod 61 = 1
            45 => 19, // 45 × 19 = 855 mod 61 = 1
            46 => 4,  // 46 × 4 = 184 mod 61 = 1
            47 => 13, // 47 × 13 = 611 mod 61 = 1
            48 => 14, // 48 × 14 = 672 mod 61 = 1
            49 => 5,  // 49 × 5 = 245 mod 61 = 1
            50 => 11, // 50 × 11 = 550 mod 61 = 1
            51 => 6,  // 51 × 6 = 306 mod 61 = 1
            52 => 27, // 52 × 27 = 1404 mod 61 = 1
            53 => 38, // 53 × 38 = 2014 mod 61 = 1
            54 => 26, // 54 × 26 = 1404 mod 61 = 1
            55 => 10, // 55 × 10 = 550 mod 61 = 1
            56 => 12, // 56 × 12 = 672 mod 61 = 1
            57 => 15, // 57 × 15 = 855 mod 61 = 1
            58 => 20, // 58 × 20 = 1160 mod 61 = 1
            59 => 30, // 59 × 30 = 1770 mod 61 = 1
            60 => 60, // 60 × 60 = 3600 mod 61 = 1
            _ => unreachable!(),
        };

        Self::new(res)
    }

    fn square_root(&self) -> Option<(Self, Self)> {
        // Definition for: sqrt(n)
        //                 sqrt(n) * sqrt(n) = n
        // from: https://curves.xargs.org/sqrt61.html

        let res: Option<(u8, u8)> = match self.value {
            0 => Some((0, 0)),
            1 => Some((1, 60)),
            2 => None,
            3 => Some((8, 53)),
            4 => Some((2, 59)),
            5 => Some((26, 35)),
            6 => None,
            7 => None,
            8 => None,
            9 => Some((3, 58)),
            10 => None,
            11 => None,
            12 => Some((16, 45)),
            13 => Some((14, 47)),
            14 => Some((21, 40)),
            15 => Some((25, 36)),
            16 => Some((4, 57)),
            17 => None,
            18 => None,
            19 => Some((18, 43)),
            20 => Some((9, 52)),
            21 => None,
            22 => Some((12, 49)),
            23 => None,
            24 => None,
            25 => Some((5, 56)),
            26 => None,
            27 => Some((24, 37)),
            28 => None,
            29 => None,
            30 => None,
            31 => None,
            32 => None,
            33 => None,
            34 => Some((20, 41)),
            35 => None,
            36 => Some((6, 55)),
            37 => None,
            38 => None,
            39 => Some((10, 51)),
            40 => None,
            41 => Some((23, 38)),
            42 => Some((15, 46)),
            43 => None,
            44 => None,
            45 => Some((17, 44)),
            46 => Some((30, 31)),
            47 => Some((13, 48)),
            48 => Some((29, 32)),
            49 => Some((7, 54)),
            50 => None,
            51 => None,
            52 => Some((28, 33)),
            53 => None,
            54 => None,
            55 => None,
            56 => Some((19, 42)),
            57 => Some((22, 39)),
            58 => Some((27, 34)),
            59 => None,
            60 => Some((11, 50)),
            _ => unreachable!(),
        };

        res.map(|(v1, v2)| (FiniteField61Point::new(v1), FiniteField61Point::new(v2)))
    }
}

impl Add for FiniteField61Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            value: (self.value + other.value).mod_floor(&Self::P),
        }
    }
}

impl Sub for FiniteField61Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        // safe to unwrap -> [0; P - 1] where P: u8
        let v1 = i8::try_from(self.value).unwrap();
        // safe to unwrap -> [0; P - 1]
        let v2 = i8::try_from(other.value).unwrap();
        let v = (v1 - v2).mod_floor(&Self::SIGNED_P);

        Self {
            // Safe to unwrap as the result of the subtract is modulo P
            value: u8::try_from(v).unwrap(),
        }
    }
}

impl Sub for &FiniteField61Point {
    type Output = FiniteField61Point;

    fn sub(self, other: &Self::Output) -> Self::Output {
        // safe to unwrap -> [0; P - 1] where P: u8
        let v1 = i8::try_from(self.value).unwrap();
        // safe to unwrap -> [0; P - 1]
        let v2 = i8::try_from(other.value).unwrap();
        let v = (v1 - v2).mod_floor(&FiniteField61Point::SIGNED_P);

        Self::Output {
            // Safe to unwrap as the result of the subtract is modulo P
            value: u8::try_from(v).unwrap(),
        }
    }
}

impl Mul for FiniteField61Point {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        // Need to use u16 here
        let v1 = u16::from(self.value);
        let v2 = u16::from(other.value);
        let v = (v1 * v2).mod_floor(&Self::P_U16);

        Self {
            // Safe to unwrap as the result of the multiplication is module 23
            value: u8::try_from(v).unwrap(),
        }
    }
}

impl Mul for &FiniteField61Point {
    type Output = FiniteField61Point;

    fn mul(self, other: &Self::Output) -> Self::Output {
        // Need to use u16 here
        let v1 = u16::from(self.value);
        let v2 = u16::from(other.value);
        let v = (v1 * v2).mod_floor(&FiniteField61Point::P_U16);

        FiniteField61Point {
            // Safe to unwrap as the result of the multiplication is module 23
            value: u8::try_from(v).unwrap(),
        }
    }
}

impl Neg for FiniteField61Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        // Definition for: -n
        // n + (-n) = 0

        // safe to unwrap -> [0; - 1] where P: u8
        let v1 = i8::try_from(self.value).unwrap();
        let v2 = Self::SIGNED_P;
        let v = (v2 - v1).mod_floor(&Self::SIGNED_P);

        Self {
            // Safe to unwrap as the result of the subtract is modulo 23
            value: u8::try_from(v).unwrap(),
        }
    }
}

impl Neg for &FiniteField61Point {
    type Output = FiniteField61Point;

    fn neg(self) -> Self::Output {
        // Definition for: -n
        // n + (-n) = 0

        // safe to unwrap -> [0; - 1] where P: u8
        let v1 = i8::try_from(self.value).unwrap();
        let v2 = Self::Output::SIGNED_P;
        let v = (v2 - v1).mod_floor(&Self::Output::SIGNED_P);

        Self::Output {
            // Safe to unwrap as the result of the subtract is modulo 23
            value: u8::try_from(v).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let ff_min = FiniteField61Point::min();
        assert_eq!(ff_min, FiniteField61Point::new(0));

        let ff_max = FiniteField61Point::max();
        assert_eq!(ff_max, FiniteField61Point::new(60));
    }

    #[test]
    #[should_panic]
    fn test_basic_invalid_1() {
        FiniteField61Point::new(61);
    }

    #[test]
    #[should_panic]
    fn test_basic_invalid_2() {
        FiniteField61Point::new(70);
    }

    #[test]
    fn test_add() {
        assert_eq!(
            FiniteField61Point::new(41) + FiniteField61Point::new(1),
            FiniteField61Point::new(42)
        );
        assert_eq!(
            FiniteField61Point::new(0) + FiniteField61Point::new(0),
            FiniteField61Point::new(0)
        );
        assert_eq!(
            FiniteField61Point::max() + FiniteField61Point::new(1),
            FiniteField61Point::new(0)
        );
        assert_eq!(
            FiniteField61Point::max() + FiniteField61Point::new(2),
            FiniteField61Point::new(1)
        );
        assert_eq!(
            FiniteField61Point::max() + FiniteField61Point::max(),
            FiniteField61Point::new(FiniteField61Point::P - 2)
        );
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            FiniteField61Point::new(43) - FiniteField61Point::new(1),
            FiniteField61Point::new(42)
        );
        assert_eq!(
            FiniteField61Point::new(0) - FiniteField61Point::new(0),
            FiniteField61Point::new(0)
        );
    }

    #[test]
    fn test_neg() {
        for i in 0..FiniteField61Point::P {
            let v = FiniteField61Point::new(i);
            let nv = (&v).neg();
            assert_eq!(v + nv, FiniteField61Point::new(0));
        }
    }

    #[test]
    fn test_multiplication_inverse() {
        for i in 1..FiniteField61Point::P {
            let v = FiniteField61Point::new(i);
            let iv = v.mul_inverse();
            assert_eq!(v * iv, FiniteField61Point::new(1));
        }
    }

    #[test]
    fn test_square_roots() {
        for i in 1..FiniteField61Point::P {
            let v = FiniteField61Point::new(i);
            if let Some((v_sqr_1, v_sqr_2)) = v.square_root() {
                assert_eq!(&v_sqr_1 * &v_sqr_1, v);
                assert_eq!(&v_sqr_2 * &v_sqr_2, v);
            }
        }
    }
}
