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
    test_1();
    test_2();
    test_3();
}

fn complex_tests() {
    complex_test_1();
    complex_test_2();
    complex_test_3();
}

/* ========================================================= */
/*                         REAL                              */
/* ========================================================= */

fn test_1() {
    println!();
    println!("--- Test 1 : Identity matrix ---");

    let  u = Matrix::new(
        vec![
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ],
        3,
        3,
    )
    .unwrap();

    println!("matrix:");
    println!("{}", u);

    let result = u.rank();

    println!("rank:");
    println!("{}", result);
}

fn test_2() {
    println!();
    println!("--- Test 2 : Dependent rows ---");

    let  u = Matrix::new(
        vec![
             1.0, 2.0, 0.0, 0.0,
             2.0, 4.0, 0.0, 0.0,
            -1.0, 2.0, 1.0, 1.0,
        ],
        3,
        4,
    )
    .unwrap();

    println!("matrix:");
    println!("{}", u);

    let result = u.rank();

    println!("rank:");
    println!("{}", result);
}

fn test_3() {
    println!();
    println!("--- Test 3 : Rectangular matrix ---");

    let  u = Matrix::new(
        vec![
             8.0,  5.0, -2.0,
             4.0,  7.0, 20.0,
             7.0,  6.0,  1.0,
            21.0, 18.0,  7.0,
        ],
        4,
        3,
    )
    .unwrap();

    println!("matrix:");
    println!("{}", u);

    let result = u.rank();

    println!("rank:");
    println!("{}", result);
}

/* ========================================================= */
/*                        COMPLEX                            */
/* ========================================================= */

fn complex_test_1() {
    println!();
    println!("--- Complex Test 1 : Full rank ---");

    let  u = Matrix::new(
        vec![
            c(1.0, 1.0), c(0.0, 0.0), c(0.0, 0.0),
            c(0.0, 0.0), c(2.0, -1.0), c(0.0, 0.0),
            c(0.0, 0.0), c(0.0, 0.0), c(-1.0, 2.0),
        ],
        3,
        3,
    )
    .unwrap();

    println!("matrix:");
    println!("{}", u);

    let result = u.rank();

    println!("rank:");
    println!("{}", result);
}

fn complex_test_2() {
    println!();
    println!("--- Complex Test 2 : Dependent rows ---");

    let  u = Matrix::new(
        vec![
            c(1.0, 1.0), c(2.0, 0.0),
            c(2.0, 2.0), c(4.0, 0.0),
        ],
        2,
        2,
    )
    .unwrap();

    println!("matrix:");
    println!("{}", u);

    let result = u.rank();

    println!("rank:");
    println!("{}", result);
}

fn complex_test_3() {
    println!();
    println!("--- Complex Test 3 : Rectangular dependent matrix ---");

    let  u = Matrix::new(
        vec![
            c(1.0, 0.0), c(0.0, 0.0), c(0.0, 1.0), c(2.0, 0.0),
            c(0.0, 0.0), c(1.0, 0.0), c(1.0, 0.0), c(0.0, 0.0),
            c(1.0, 0.0), c(1.0, 0.0), c(1.0, 1.0), c(2.0, 0.0),
        ],
        3,
        4,
    )
    .unwrap();

    println!("matrix:");
    println!("{}", u);

    let result = u.rank();

    println!("rank:");
    println!("{}", result);
}