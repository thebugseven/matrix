use crate::math::angle_cos;
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

fn real_tests() {
    test_1();
    test_2();
    test_3();
    test_4();
    test_5();
}

fn complex_tests() {
    complex_cos_test_1();
    complex_cos_test_2();
}

fn complex_cos_test_1() {
    println!("\n--- Complex cosine 1 ---");

    let mut u = Vector::from([
        c(1.0, 0.0),
        c(0.0, 1.0),
    ]);

    let mut v = Vector::from([
        c(1.0, 0.0),
        c(0.0, 1.0),
    ]);

    println!("u:\n{}", u);
    println!("v:\n{}", v);

    let result = angle_cos(&mut u, &mut v);

    println!("cosine:");
    println!("{}", result);
}

fn complex_cos_test_2() {
    println!("\n--- Complex cosine 2 ---");

    let mut u = Vector::from([
        c(1.0, 1.0),
        c(2.0, 0.0),
    ]);

    let mut v = Vector::from([
        c(2.0, -1.0),
        c(0.0, 2.0),
    ]);

    println!("u:\n{}", u);
    println!("v:\n{}", v);

    let result = angle_cos(&mut u, &mut v);

    println!("cosine:");
    println!("{}", result);
}


fn print_test(u: Vector<f32>, v: Vector<f32>) {
    println!("u:");
    println!("{}", u);

    println!("v:");
    println!("{}", v);

    let result = angle_cos(&u, &v);

    println!("cosine:");
    println!("{}", result);
}

fn test_1() {
    println!();
    println!("--- Test 1 ---");

    let u = Vector::from([1.0, 0.0]);
    let v = Vector::from([1.0, 0.0]);

    print_test(u, v);
}

fn test_2() {
    println!();
    println!("--- Test 2 ---");

    let u = Vector::from([1.0, 0.0]);
    let v = Vector::from([0.0, 1.0]);

    print_test(u, v);
}

fn test_3() {
    println!();
    println!("--- Test 3 ---");

    let u = Vector::from([-1.0, 1.0]);
    let v = Vector::from([1.0, -1.0]);

    print_test(u, v);
}

fn test_4() {
    println!();
    println!("--- Test 4 ---");

    let u = Vector::from([2.0, 1.0]);
    let v = Vector::from([4.0, 2.0]);

    print_test(u, v);
}

fn test_5() {
    println!();
    println!("--- Test 5 ---");

    let u = Vector::from([1.0, 2.0, 3.0]);
    let v = Vector::from([4.0, 5.0, 6.0]);

    print_test(u, v);
}