use crate::complex::Complex;
use crate::matrix::Matrix;
use crate::vector::Vector;

fn c(re: f32, im: f32) -> Complex {
    Complex::new(re, im)
}

pub fn run() {
    println!("\n========== REAL TESTS ==========");
    real_tests();

    println!("\n======== COMPLEX TESTS =========");
    complex_tests();
}

fn real_tests() {
    vector_add();
    vector_sub();
    vector_scale();

    matrix_add();
    matrix_sub();
    matrix_scale();
}

fn complex_tests() {
    complex_vector_add();
    complex_vector_sub();
    complex_vector_scale();

    complex_matrix_add();
    complex_matrix_sub();
    complex_matrix_scale();
}

fn vector_add() {
    println!();
    println!("--- Vector addition ---");

    let mut u = Vector::from([2.0, 3.0]);
    let v = Vector::from([5.0, 7.0]);

    println!("u:");
    println!("{}\n", u);

    println!("v:");
    println!("{}\n", v);

    u.add(&v);

    println!("result:");
    println!("{}\n", u);
}

fn vector_sub() {
    println!();
    println!("--- Vector subtraction ---");

    let mut u = Vector::from([2.0, 3.0]);
    let v = Vector::from([5.0, 7.0]);

    println!("u:");
    println!("{}\n", u);

    println!("v:");
    println!("{}\n", v);

    u.sub(&v);

    println!("result:");
    println!("{}\n", u);
}

fn vector_scale() {
    println!();
    println!("--- Vector scaling ---");

    let mut u = Vector::from([2.0, 3.0]);
    let scalar = 2.0;

    println!("u:");
    println!("{}\n", u);

    println!("scalar: {}\n", scalar);

    u.scl(scalar);

    println!("result:");
    println!("{}\n", u);
}

fn matrix_add() {
    println!();
    println!("--- Matrix addition ---");

    let mut u = Matrix::from([
        [1.0, 2.0],
        [3.0, 4.0],
    ]);

    let v = Matrix::from([
        [7.0, 4.0],
        [-2.0, 2.0],
    ]);

    println!("u:");
    println!("{}\n", u);

    println!("v:");
    println!("{}\n", v);

    u.add(&v);

    println!("result:");
    println!("{}\n", u);
}

fn matrix_sub() {
    println!();
    println!("--- Matrix subtraction ---");

    let mut u = Matrix::from([
        [1.0, 2.0],
        [3.0, 4.0],
    ]);

    let v = Matrix::from([
        [7.0, 4.0],
        [-2.0, 2.0],
    ]);

    println!("u:");
    println!("{}\n", u);

    println!("v:");
    println!("{}\n", v);

    u.sub(&v);

    println!("result:");
    println!("{}\n", u);
}

fn matrix_scale() {
    println!();
    println!("--- Matrix scaling ---");

    let mut u = Matrix::from([
        [1.0, 2.0],
        [3.0, 4.0],
    ]);

    let scalar = 2.0;

    println!("u:");
    println!("{}\n", u);

    println!("scalar: {}\n", scalar);

    u.scl(scalar);

    println!("result:");
    println!("{}\n", u);
}

fn complex_vector_add() {
    println!("\n--- Complex vector addition ---");

    let mut u = Vector::from([
        c(2.0, 1.0),
        c(3.0, -2.0),
    ]);

    let v = Vector::from([
        c(5.0, -1.0),
        c(7.0, 4.0),
    ]);

    println!("u:\n{}\n", u);
    println!("v:\n{}\n", v);

    u.add(&v);

    println!("result:\n{}\n", u);
}

fn complex_vector_sub() {
    println!("\n--- Complex vector subtraction ---");

    let mut u = Vector::from([
        c(2.0, 1.0),
        c(3.0, -2.0),
    ]);

    let v = Vector::from([
        c(5.0, -1.0),
        c(7.0, 4.0),
    ]);

    println!("u:\n{}\n", u);
    println!("v:\n{}\n", v);

    u.sub(&v);

    println!("result:\n{}\n", u);
}

fn complex_vector_scale() {
    println!("\n--- Complex vector scaling ---");

    let mut u = Vector::from([
        c(2.0, 1.0),
        c(3.0, -2.0),
    ]);

    let scalar = c(2.0, 1.0);

    println!("u:\n{}\n", u);
    println!("scalar: {}", scalar);

    u.scl(scalar);

    println!("result:\n{}\n", u);
}


fn complex_matrix_add() {
    println!("\n--- Complex matrix addition ---");

    let mut u = Matrix::from([
        [c(1.0, 1.0), c(2.0, -1.0)],
        [c(3.0, 2.0), c(4.0, 0.0)],
    ]);

    let v = Matrix::from([
        [c(2.0, -1.0), c(1.0, 1.0)],
        [c(-1.0, 3.0), c(2.0, -2.0)],
    ]);

    println!("u:\n{}\n", u);
    println!("v:\n{}\n", v);

    u.add(&v);

    println!("result:\n{}\n", u);
}

fn complex_matrix_sub() {
    println!("\n--- Complex matrix subtraction ---");

    let mut u = Matrix::from([
        [c(1.0, 1.0), c(2.0, -1.0)],
        [c(3.0, 2.0), c(4.0, 0.0)],
    ]);

    let v = Matrix::from([
        [c(2.0, -1.0), c(1.0, 1.0)],
        [c(-1.0, 3.0), c(2.0, -2.0)],
    ]);

    println!("u:\n{}\n", u);
    println!("v:\n{}\n", v);

    u.sub(&v);

    println!("result:\n{}\n", u);
}

fn complex_matrix_scale() {
    println!("\n--- Complex matrix scaling ---");

    let mut u = Matrix::from([
        [c(1.0, 1.0), c(2.0, 0.0)],
        [c(0.0, -1.0), c(3.0, 2.0)],
    ]);

    let scalar = c(2.0, 1.0);

    println!("u:\n{}\n", u);
    println!("scalar: {}", scalar);

    u.scl(scalar);

    println!("result:\n{}\n", u);
}