use crate::math::angle_cos;
use crate::vector::Vector;

pub fn run() {
    test_1();
    test_2();
    test_3();
    test_4();
    test_5();
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