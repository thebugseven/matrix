use crate::math::lerp;
use crate::matrix::Matrix;
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

fn complex_tests(){
    complex_vector_test();
    complex_matrix_test();

}

fn real_tests() {
    scalar_tests();
    vector_test();
    matrix_test();
}

fn scalar_tests() {
    println!();
    println!("--- Scalar interpolation ---");

    println!(
        "lerp(0, 1, 0) = {}",
        lerp(0.0, 1.0, 0.0)
    );

    println!(
        "lerp(0, 1, 1) = {}",
        lerp(0.0, 1.0, 1.0)
    );

    println!(
        "lerp(0, 1, 0.5) = {}",
        lerp(0.0, 1.0, 0.5)
    );

    println!(
        "lerp(21, 42, 0.3) = {}",
        lerp(21.0, 42.0, 0.3)
    );
}

fn vector_test() {
    println!();
    println!("--- Vector interpolation ---");

    let u = Vector::from([2.0, 1.0]);
    let v = Vector::from([4.0, 2.0]);
    let t = 0.3;

    println!("u:");
    println!("{}\n", u);

    println!("v:");
    println!("{}\n", v);

    println!("t = {}\n", t);

    let result = lerp(u, v, t);

    println!("result:");
    println!("{}\n", result);
}

fn matrix_test() {
    println!();
    println!("--- Matrix interpolation ---");

    let u = Matrix::from([
        [2.0, 1.0],
        [3.0, 4.0],
    ]);

    let v = Matrix::from([
        [20.0, 10.0],
        [30.0, 40.0],
    ]);

    let t = 0.5;

    println!("u:");
    println!("{}\n", u);

    println!("v:");
    println!("{}\n", v);

    println!("t = {}\n", t);

    let result = lerp(u, v, t);

    println!("result:");
    println!("{}", result);
}

fn complex_vector_test() {
    println!("\n--- Complex vector interpolation ---");

    let u = Vector::from([
        c(2.0, 1.0),
        c(1.0, -2.0),
    ]);

    let v = Vector::from([
        c(4.0, 3.0),
        c(2.0, 2.0),
    ]);

    let t = 0.5;

    println!("u:\n{}\n", u);
    println!("v:\n{}\n", v);
    println!("t = {}\n", t);

    let result = lerp(u, v, t);

    println!("result:\n{}\n", result);
}

fn complex_matrix_test() {
    println!("\n--- Complex matrix interpolation ---");

    let u = Matrix::from([
        [c(2.0, 1.0), c(1.0, 0.0)],
        [c(3.0, -1.0), c(4.0, 2.0)],
    ]);

    let v = Matrix::from([
        [c(20.0, 3.0), c(10.0, 2.0)],
        [c(30.0, 1.0), c(40.0, -2.0)],
    ]);

    let t = 0.5;

    println!("u:\n{}\n", u);
    println!("v:\n{}\n", v);
    println!("t = {}\n", t);

    let result = lerp(u, v, t);

    println!("result:\n{}\n", result);
}