use crate::math::linear_combination;
use crate::vector::Vector;

pub fn run() {
    canonical_basis();
    custom_vectors();
}

fn canonical_basis() {
    println!();
    println!("--- Canonical basis ---");

    let e1 = Vector::from([1.0, 0.0, 0.0]);
    let e2 = Vector::from([0.0, 1.0, 0.0]);
    let e3 = Vector::from([0.0, 0.0, 1.0]);

    let vectors = [e1, e2, e3];
    let coefs = [10.0, -2.0, 0.5];

    println!("coefficients: {:?}", coefs);

    let result = linear_combination(&vectors, &coefs);

    println!("result:");
    println!("{}", result);
}

fn custom_vectors() {
    println!();
    println!("--- Arbitrary vectors ---");

    let v1 = Vector::from([1.0, 2.0, 3.0]);
    let v2 = Vector::from([0.0, 10.0, -100.0]);

    let vectors = [v1, v2];
    let coefs = [10.0, -2.0];

    println!("coefficients: {:?}", coefs);

    let result = linear_combination(&vectors, &coefs);

    println!("result:");
    println!("{}", result);
}