use crate::math::linear_combination;
use crate::vector::Vector;
use crate::complex::Complex;

fn c(re: f32, im: f32) -> Complex {
    Complex::new(re, im)
}

pub fn run() {
    println!("\n========== REAL TESTS ==========");
    real_tests();

    println!("\n======== COMPLEX TESTS =========");
    complex_tests();
}

fn complex_tests() {
    println!("\n--- Complex linear combination ---");

    let v1 = Vector::from([
        c(1.0, 0.0),
        c(2.0, 1.0),
        c(0.0, -1.0),
    ]);

    let v2 = Vector::from([
        c(0.0, 1.0),
        c(1.0, 0.0),
        c(2.0, 0.0),
    ]);

    let vectors = [v1, v2];

    let coefs = [
        c(2.0, 1.0),
        c(-1.0, 2.0),
    ];

    println!("coefficients:");
    println!("{}", coefs[0]);
    println!("{}\n", coefs[1]);

    let result = linear_combination(&vectors, &coefs);

    println!("result:");
    println!("{}\n", result);
}


fn real_tests() {
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

    println!("coefficients: {:?}\n", coefs);

    let result = linear_combination(&vectors, &coefs);

    println!("result:");
    println!("{}\n", result);
}

fn custom_vectors() {
    println!();
    println!("--- Arbitrary vectors ---");

    let v1 = Vector::from([1.0, 2.0, 3.0]);
    let v2 = Vector::from([0.0, 10.0, -100.0]);

    let vectors = [v1, v2];
    let coefs = [10.0, -2.0];

    println!("coefficients: {:?}\n", coefs);

    let result = linear_combination(&vectors, &coefs);

    println!("result:");
    println!("{}\n", result);
}