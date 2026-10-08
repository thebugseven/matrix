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

fn complex_tests() {
    complex_mul_vec();
    complex_mul_mat();
}

fn real_tests() {
    println!();
    println!("--- Matrix x Vector ---");

    mul_vec_test_1();
    mul_vec_test_2();
    mul_vec_test_3();

    println!();
    println!("--- Matrix x Matrix ---");

    mul_mat_test_1();
    mul_mat_test_2();
    mul_mat_test_3();
}

fn mul_vec_test_1() {
    println!();
    println!("--- Test 1 ---");

    let u = Matrix::from([
        [1.0, 0.0],
        [0.0, 1.0],
    ]);

    let v = Vector::from([4.0, 2.0]);

    println!("matrix:");
    println!("{}\n", u);

    println!("vector:");
    println!("{}\n", v);

    let result = u.mul_vec(&v);

    println!("result:");
    println!("{}\n", result);
}

fn mul_vec_test_2() {
    println!();
    println!("--- Test 2 ---");

    let u = Matrix::from([
        [2.0, 0.0],
        [0.0, 2.0],
    ]);

    let v = Vector::from([4.0, 2.0]);

    println!("matrix:");
    println!("{}\n", u);

    println!("vector:");
    println!("{}\n", v);

    let result = u.mul_vec(&v);

    println!("result:");
    println!("{}\n", result);
}

fn mul_vec_test_3() {
    println!();
    println!("--- Test 3 ---");

    let u = Matrix::from([
        [2.0, -2.0],
        [-2.0, 2.0],
    ]);

    let v = Vector::from([4.0, 2.0]);

    println!("matrix:");
    println!("{}\n", u);

    println!("vector:");
    println!("{}\n", v);

    let result = u.mul_vec(&v);

    println!("result:");
    println!("{}\n", result);
}

fn mul_mat_test_1() {
    println!();
    println!("--- Test 1 ---");

    let u = Matrix::from([
        [1.0, 0.0],
        [0.0, 1.0],
    ]);

    let v = Matrix::from([
        [1.0, 0.0],
        [0.0, 1.0],
    ]);

    println!("matrix u:");
    println!("{}\n", u);

    println!("matrix v:");
    println!("{}\n", v);

    let result = u.mul_mat(&v);

    println!("result:");
    println!("{}\n", result);
}

fn mul_mat_test_2() {
    println!();
    println!("--- Test 2 ---");

    let u = Matrix::from([
        [1.0, 0.0],
        [0.0, 1.0],
    ]);

    let v = Matrix::from([
        [2.0, 1.0],
        [4.0, 2.0],
    ]);

    println!("matrix u:");
    println!("{}\n", u);

    println!("matrix v:");
    println!("{}\n", v);

    let result = u.mul_mat(&v);

    println!("result:");
    println!("{}\n", result);
}

fn mul_mat_test_3() {
    println!();
    println!("--- Test 3 ---");

    let u = Matrix::from([
        [3.0, -5.0],
        [6.0, 8.0],
    ]);

    let v = Matrix::from([
        [2.0, 1.0],
        [4.0, 2.0],
    ]);

    println!("matrix u:");
    println!("{}\n", u);

    println!("matrix v:");
    println!("{}\n", v);

    let result = u.mul_mat(&v);

    println!("result:");
    println!("{}\n", result);
}

fn complex_mul_vec() {
    println!("\n--- Complex matrix x vector ---");

    let matrix = Matrix::from([
        [c(1.0, 1.0), c(2.0, 0.0)],
        [c(0.0, -1.0), c(3.0, 2.0)],
    ]);

    let vector = Vector::from([
        c(2.0, 1.0),
        c(1.0, -1.0),
    ]);

    println!("matrix:");
    println!("{}\n", matrix);

    println!("vector:");
    println!("{}\n", vector);

    let result = matrix.mul_vec(&vector);

    println!("result:");
    println!("{}\n", result);
}

fn complex_mul_mat() {
    println!("\n--- Complex matrix x matrix ---");

    let a = Matrix::from([
        [c(1.0, 1.0), c(2.0, 0.0)],
        [c(0.0, -1.0), c(3.0, 1.0)],
    ]);

    let b = Matrix::from([
        [c(2.0, 0.0), c(1.0, -1.0)],
        [c(1.0, 2.0), c(0.0, 1.0)],
    ]);

    println!("A:");
    println!("{}\n", a);

    println!("B:");
    println!("{}\n", b);

    let result = a.mul_mat(&b);

    println!("result:");
    println!("{}\n", result);
}