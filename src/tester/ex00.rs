use crate::matrix::Matrix;
use crate::vector::Vector;

pub fn run() {
    vector_add();
    vector_sub();
    vector_scale();

    matrix_add();
    matrix_sub();
    matrix_scale();
}

fn vector_add() {
    println!();
    println!("--- Vector addition ---");

    let mut u = Vector::from([2.0, 3.0]);
    let v = Vector::from([5.0, 7.0]);

    println!("u:");
    println!("{}", u);

    println!("v:");
    println!("{}", v);

    u.add(&v);

    println!("result:");
    println!("{}", u);
}

fn vector_sub() {
    println!();
    println!("--- Vector subtraction ---");

    let mut u = Vector::from([2.0, 3.0]);
    let v = Vector::from([5.0, 7.0]);

    println!("u:");
    println!("{}", u);

    println!("v:");
    println!("{}", v);

    u.sub(&v);

    println!("result:");
    println!("{}", u);
}

fn vector_scale() {
    println!();
    println!("--- Vector scaling ---");

    let mut u = Vector::from([2.0, 3.0]);
    let scalar = 2.0;

    println!("u:");
    println!("{}", u);

    println!("scalar: {}", scalar);

    u.scl(scalar);

    println!("result:");
    println!("{}", u);
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
    println!("{}", u);

    println!("v:");
    println!("{}", v);

    u.add(&v);

    println!("result:");
    println!("{}", u);
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
    println!("{}", u);

    println!("v:");
    println!("{}", v);

    u.sub(&v);

    println!("result:");
    println!("{}", u);
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
    println!("{}", u);

    println!("scalar: {}", scalar);

    u.scl(scalar);

    println!("result:");
    println!("{}", u);
}