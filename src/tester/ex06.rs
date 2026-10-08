use crate::math::cross_product;
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
    println!("\n--- Complex cross product ---");

    let u = Vector::from([
        c(1.0, 1.0),
        c(2.0, 0.0),
        c(0.0, -1.0),
    ]);

    let v = Vector::from([
        c(0.0, 1.0),
        c(1.0, -1.0),
        c(2.0, 0.0),
    ]);

    println!("u:\n{}\n", u);
    println!("v:\n{}\n", v);

    let result = cross_product(&u, &v);

    println!("cross product:");
    println!("{}", result);
}

fn real_tests() {
    test_1();
    test_2();
    test_3();
}

fn print_test(u: Vector<f32>, v: Vector<f32>) {
    println!("u:");
    println!("{}\n", u);

    println!("v:");
    println!("{}\n", v);

    let result = cross_product(&u, &v);

    println!("cross product:");
    println!("{}\n", result);
}

fn test_1() {
    println!();
    println!("--- Test 1 ---");

    let u = Vector::from([0.0, 0.0, 1.0]);
    let v = Vector::from([1.0, 0.0, 0.0]);

    print_test(u, v);
}

fn test_2() {
    println!();
    println!("--- Test 2 ---");

    let u = Vector::from([1.0, 2.0, 3.0]);
    let v = Vector::from([4.0, 5.0, 6.0]);

    print_test(u, v);
}

fn test_3() {
    println!();
    println!("--- Test 3 ---");

    let u = Vector::from([4.0, 2.0, -3.0]);
    let v = Vector::from([-2.0, -5.0, 16.0]);

    print_test(u, v);
}