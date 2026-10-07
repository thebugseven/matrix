use crate::vector::Vector;

pub fn run() {
    test_1();
    test_2();
    test_3();
}

fn test_1() {
    println!();
    println!("--- Test 1 ---");

    let u = Vector::from([0.0, 0.0]);
    let v = Vector::from([1.0, 1.0]);

    println!("u:");
    println!("{}", u);

    println!("v:");
    println!("{}", v);

    let result = u.dot(v);

    println!("dot product:");
    println!("{}", result);
}

fn test_2() {
    println!();
    println!("--- Test 2 ---");

    let u = Vector::from([1.0, 1.0]);
    let v = Vector::from([1.0, 1.0]);

    println!("u:");
    println!("{}", u);

    println!("v:");
    println!("{}", v);

    let result = u.dot(v);

    println!("dot product:");
    println!("{}", result);
}

fn test_3() {
    println!();
    println!("--- Test 3 ---");

    let u = Vector::from([-1.0, 6.0]);
    let v = Vector::from([3.0, 2.0]);

    println!("u:");
    println!("{}", u);

    println!("v:");
    println!("{}", v);

    let result = u.dot(v);

    println!("dot product:");
    println!("{}", result);
}