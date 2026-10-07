use crate::math::lerp;
use crate::matrix::Matrix;
use crate::vector::Vector;

pub fn run() {
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
    println!("{}", u);

    println!("v:");
    println!("{}", v);

    println!("t = {}", t);

    let result = lerp(u, v, t);

    println!("result:");
    println!("{}", result);
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
    println!("{}", u);

    println!("v:");
    println!("{}", v);

    println!("t = {}", t);

    let result = lerp(u, v, t);

    println!("result:");
    println!("{}", result);
}