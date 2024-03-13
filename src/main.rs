mod curve61;
mod ff23;
mod ff61;

use crate::ff23::FiniteField23Point;

fn main() {
    println!("== Finite Field 23 ==");

    let pa3 = FiniteField23Point::try_new(2).unwrap();
    let pb3 = FiniteField23Point::max();
    let pr3 = pa3 + pb3;

    println!("pr3: {:?}", pr3);
}
