use crate::ff61::FiniteField;
use crate::ff61::FiniteField61Point;

use bitvec::prelude::*;

trait EllipticCurve {
    type ValueType: FiniteField;

    fn base_point(&self) -> (Self::ValueType, Self::ValueType);
    fn eval_at(&self, x: Self::ValueType) -> Option<(Self::ValueType, Self::ValueType)>;
    fn point_add(
        &self,
        p: &(Self::ValueType, Self::ValueType),
        q: &(Self::ValueType, Self::ValueType),
    ) -> (Self::ValueType, Self::ValueType);
    fn point_mul(
        &self,
        p: (Self::ValueType, Self::ValueType),
        s: Self::ValueType,
    ) -> Option<(Self::ValueType, Self::ValueType)>;
}

struct Curve61 {
    base_point: (FiniteField61Point, FiniteField61Point),
}

impl EllipticCurve for Curve61 {
    type ValueType = FiniteField61Point;

    fn base_point(&self) -> (Self::ValueType, Self::ValueType) {
        self.base_point.clone()
    }

    fn eval_at(&self, x: FiniteField61Point) -> Option<(FiniteField61Point, FiniteField61Point)> {
        // Curve equation is: y^2 = x^3 + 9*x + 1
        let x_pow_2 = &x * &x;
        let x_pow_3 = &x_pow_2 * &x;
        let x_mul_9 = &x * &FiniteField61Point::new(9);
        let y_pow_2 = x_pow_3 + x_mul_9 + FiniteField61Point::new(1);
        y_pow_2.square_root()
    }

    // FIXME: should handle add with infinity
    fn point_add(
        &self,
        p: &(FiniteField61Point, FiniteField61Point),
        q: &(FiniteField61Point, FiniteField61Point),
    ) -> (FiniteField61Point, FiniteField61Point) {
        // From: https://en.wikipedia.org/wiki/Elliptic_curve_point_multiplication#Point_addition

        let lambda = match p == q {
            true => {
                let px_pow_2 = &p.0 * &p.0;
                let num = &FiniteField61Point::new(3) * &px_pow_2 + FiniteField61Point::new(9);
                let denom = &FiniteField61Point::new(2) * &p.1;
                num * denom.mul_inverse()
            }
            false => {
                let num = &q.1 - &p.1;
                let denom = &q.0 - &p.0;
                num * denom.mul_inverse()
            }
        };

        let lambda_pow_2 = &lambda * &lambda;
        let x3 = &(&lambda_pow_2 - &p.0) - &q.0;
        let y3 = &(lambda * (&p.0 - &x3)) - &p.1;

        (x3, y3)
    }

    fn point_mul(
        &self,
        p: (FiniteField61Point, FiniteField61Point),
        s: FiniteField61Point,
    ) -> Option<(FiniteField61Point, FiniteField61Point)> {
        // From: https://en.wikipedia.org/wiki/Elliptic_curve_point_multiplication#Double-and-add

        // Vector of bits
        let num = s.value();
        let raw = num.view_bits::<Lsb0>();
        let bits = raw
            .iter_ones()
            .last()
            .unwrap_or(bitvec::mem::bits_of::<u8>() - 1);
        let bv = raw[..=bits].to_bitvec();

        // res is set to point at infinity (aka None)
        // Note that: P + INF = P & P_INF + P_INF = P_INF
        let mut res: Option<(FiniteField61Point, FiniteField61Point)> = None;
        let mut temp = p;

        for bit in bv {
            if bit {
                // FIXME: remove this when point_add handles add with INF
                res = if let Some(res_) = res {
                    Some(self.point_add(&res_, &temp))
                } else {
                    Some(temp.clone())
                }
            }
            temp = self.point_add(&temp, &temp);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_at() {
        // From: https://curves.xargs.org/#elliptic-curves-and-finite-fields

        let c61 = Curve61 {
            base_point: (FiniteField61Point::new(5), FiniteField61Point::new(7)),
        };

        assert_eq!(
            c61.eval_at(FiniteField61Point::new(0)),
            Some((FiniteField61Point::new(1), FiniteField61Point::new(60)))
        );
        assert_eq!(c61.eval_at(FiniteField61Point::new(1)), None);
        assert_eq!(
            c61.eval_at(FiniteField61Point::new(2)),
            Some((FiniteField61Point::new(24), FiniteField61Point::new(37)))
        );
        assert_eq!(c61.eval_at(FiniteField61Point::new(3)), None);
        assert_eq!(c61.eval_at(FiniteField61Point::new(4)), None);
        assert_eq!(
            c61.eval_at(FiniteField61Point::new(5)),
            Some((FiniteField61Point::new(7), FiniteField61Point::new(54)))
        );
    }

    #[test]
    fn test_point_add() {
        // From: https://curves.xargs.org/#point-addition

        let c61 = Curve61 {
            base_point: (FiniteField61Point::new(5), FiniteField61Point::new(7)),
        };

        let p = (FiniteField61Point::new(5), FiniteField61Point::new(7));
        // 2P
        let p2 = c61.point_add(&p, &p);
        assert_eq!(
            p2,
            (FiniteField61Point::new(26), FiniteField61Point::new(50))
        );

        // 3P
        let p3 = c61.point_add(&p2, &p);
        assert_eq!(
            p3,
            (FiniteField61Point::new(27), FiniteField61Point::new(38))
        );
    }

    #[test]
    fn test_point_mul() {
        // From: https://curves.xargs.org/#efficient-point-multiplication

        let c61 = Curve61 {
            base_point: (FiniteField61Point::new(5), FiniteField61Point::new(7)),
        };

        let p = (FiniteField61Point::new(5), FiniteField61Point::new(7));
        let p2 = c61.point_mul(p, FiniteField61Point::new(2));
        assert_eq!(
            p2.unwrap(),
            (FiniteField61Point::new(26), FiniteField61Point::new(50))
        );

        let p = (FiniteField61Point::new(5), FiniteField61Point::new(7));
        let p3 = c61.point_mul(p, FiniteField61Point::new(3));
        assert_eq!(
            p3.unwrap(),
            (FiniteField61Point::new(27), FiniteField61Point::new(38))
        );
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_key_exchange() {
        // https://curves.xargs.org/#key-exchange
        /*
         * Alice and Bob agree to use Curve61
         * Alice picks a random number: kA
         * Alice computes the coordinates of kA*P and sends it to Bob as A
         * Bob picks a random number: kB
         * Bob computes the coordinates of kB*P and sends it to Alice as B
         * Alice computes the coordinates of kA*B, which is: kA*(kB*P)
         * Bob computes the coordinates of kB*A, which is: kB*(kA*P)
         */

        let c61 = Curve61 {
            base_point: (FiniteField61Point::new(5), FiniteField61Point::new(7)),
        };
        let p = c61.base_point();

        // Alice
        let kA = FiniteField61Point::new(12); // A very random number :-)
        let A = c61.point_mul(p.clone(), kA.clone()).unwrap();

        // Bob
        let kB = FiniteField61Point::new(7);
        let B = c61.point_mul(p, kB.clone()).unwrap();

        // Alice
        let kA_B = c61.point_mul(B, kA).unwrap();

        // Bob
        let kb_A = c61.point_mul(A, kB).unwrap();

        assert_eq!(kA_B, kb_A);
    }
}
