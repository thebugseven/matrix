use crate::vector::Vector;

pub fn run() {
    test_1();
    test_2();
    test_3();
}

fn test_1() {
    println!();
    println!("--- Test 1 ---");

    let u = Vector::from([0.0, 0.0, 0.0]);

    println!("u:");
    println!("{}", u);

    println!("norm_1    = {}", u.norm_1());
    println!("norm_2    = {}", u.norm_2());
    println!("norm_inf  = {}", u.norm_inf());
}

fn test_2() {
    println!();
    println!("--- Test 2 ---");

    let u = Vector::from([1.0, 2.0, 3.0]);

    println!("u:");
    println!("{}", u);

    println!("norm_1    = {}", u.norm_1());
    println!("norm_2    = {}", u.norm_2());
    println!("norm_inf  = {}", u.norm_inf());
}

fn test_3() {
    println!();
    println!("--- Test 3 ---");

    let u = Vector::from([-1.0, -2.0]);

    println!("u:");
    println!("{}", u);

    println!("norm_1    = {}", u.norm_1());
    println!("norm_2    = {}", u.norm_2());
    println!("norm_inf  = {}", u.norm_inf());
}