use bnum::types::I512;

mod finite_field;
mod finite_field_elliptic_curve;
mod secp256k1;

fn main() {
    println!("Hello, world!");
    println!("I256: {}", I512::MAX % I512::from(2));
}
