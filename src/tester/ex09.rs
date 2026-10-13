use crate::complex::Complex;
use crate::matrix::Matrix;

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
    test_square();
    test_rectangular();
}

fn complex_tests() {
    complex_square();
    complex_rectangular();
}

/* ========================================================= */
/*                         REAL                              */
/* ========================================================= */

fn test_square() {
    println!();
    println!("--- Square matrix ---");

    let u = Matrix::new(
        vec![
            1.0, 2.0,
            3.0, 4.0,
        ],
        2,
        2,
    )
    .unwrap();

    println!("matrix:");
    println!("{}\n", u);

    let result = u.transpose();

    println!("transpose:");
    println!("{}\n", result);
}

fn test_rectangular() {
    println!();
    println!("--- Rectangular matrix ---");

    let u = Matrix::new(
        vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
        ],
        2,
        3,
    )
    .unwrap();

    println!("matrix:");
    println!("{}\n", u);

    let result = u.transpose();

    println!("transpose:");
    println!("{}\n", result);
}

/* ========================================================= */
/*                        COMPLEX                            */
/* ========================================================= */

fn complex_square() {
    println!();
    println!("--- Complex square matrix ---");

    let u = Matrix::new(
        vec![
            c(1.0, 2.0),  c(3.0, -1.0),
            c(2.0, 4.0),  c(-1.0, 5.0),
        ],
        2,
        2,
    )
    .unwrap();

    println!("matrix:");
    println!("{}\n", u);

    let result = u.transpose();

    println!("conjugate transpose:");
    println!("{}\n", result);
}

fn complex_rectangular() {
    println!();
    println!("--- Complex rectangular matrix ---");

    let u = Matrix::new(
        vec![
            c(1.0, 1.0), c(2.0, -2.0), c(3.0, 0.0),
            c(4.0, 3.0), c(-1.0, 1.0), c(0.0, -2.0),
        ],
        2,
        3,
    )
    .unwrap();

    println!("matrix:");
    println!("{}\n", u);

    let result = u.transpose();

    println!("conjugate transpose:");
    println!("{}\n", result);
}