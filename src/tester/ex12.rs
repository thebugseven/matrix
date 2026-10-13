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
    test_singular();
}

fn complex_tests() {
    complex_test_1();
    complex_test_2();
    complex_test_singular();
}

/* ========================================================= */
/*                         REAL                              */
/* ========================================================= */

fn test_1() {
    println!();
    println!("--- Test 1 : Identity matrix ---");

    let u = Matrix::new(
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

    match u.inverse() {
        Ok(result) => {
            println!("inverse:");
            println!("{}", result);
        }
        Err(err) => {
            println!("error: {:?}", err);
        }
    }
}

fn test_2() {
    println!();
    println!("--- Test 2 : Diagonal matrix ---");

    let u = Matrix::new(
        vec![
            2.0, 0.0, 0.0,
            0.0, 2.0, 0.0,
            0.0, 0.0, 2.0,
        ],
        3,
        3,
    )
    .unwrap();

    println!("matrix:");
    println!("{}", u);

    match u.inverse() {
        Ok(result) => {
            println!("inverse:");
            println!("{}", result);
        }
        Err(err) => {
            println!("error: {:?}", err);
        }
    }
}

fn test_3() {
    println!();
    println!("--- Test 3 : General 3x3 ---");

    let u = Matrix::new(
        vec![
            8.0, 5.0, -2.0,
            4.0, 7.0, 20.0,
            7.0, 6.0, 1.0,
        ],
        3,
        3,
    )
    .unwrap();

    println!("matrix:");
    println!("{}", u);

    match u.inverse() {
        Ok(result) => {
            println!("inverse:");
            println!("{}", result);
        }
        Err(err) => {
            println!("error: {:?}", err);
        }
    }
}

fn test_singular() {
    println!();
    println!("--- Test 4 : Singular matrix ---");

    let u = Matrix::new(
        vec![
            1.0, 2.0,
            2.0, 4.0,
        ],
        2,
        2,
    )
    .unwrap();

    println!("matrix:");
    println!("{}", u);

    match u.inverse() {
        Ok(result) => {
            println!("inverse:");
            println!("{}", result);
        }
        Err(err) => {
            println!("error:");
            println!("{:?}", err);
        }
    }
}

/* ========================================================= */
/*                        COMPLEX                            */
/* ========================================================= */

fn complex_test_1() {
    println!();
    println!("--- Complex Test 1 : Diagonal matrix ---");

    let u = Matrix::new(
        vec![
            c(1.0, 1.0), c(0.0, 0.0),
            c(0.0, 0.0), c(2.0, -1.0),
        ],
        2,
        2,
    )
    .unwrap();

    println!("matrix:");
    println!("{}", u);

    match u.inverse() {
        Ok(result) => {
            println!("inverse:");
            println!("{}", result);
        }
        Err(err) => {
            println!("error: {:?}", err);
        }
    }
}

fn complex_test_2() {
    println!();
    println!("--- Complex Test 2 : Upper triangular ---");

    let u = Matrix::new(
        vec![
            c(1.0, 0.0), c(0.0, 1.0),
            c(0.0, 0.0), c(1.0, 0.0),
        ],
        2,
        2,
    )
    .unwrap();

    println!("matrix:");
    println!("{}", u);

    match u.inverse() {
        Ok(result) => {
            println!("inverse:");
            println!("{}", result);
        }
        Err(err) => {
            println!("error: {:?}", err);
        }
    }
}

fn complex_test_singular() {
    println!();
    println!("--- Complex Test 3 : Singular matrix ---");

    let u = Matrix::new(
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

    match u.inverse() {
        Ok(result) => {
            println!("inverse:");
            println!("{}", result);
        }
        Err(err) => {
            println!("error:");
            println!("{:?}", err);
        }
    }
}