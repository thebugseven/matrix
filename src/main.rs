mod vector;
mod matrix;

use vector::Vector;
// use matrix::Matrix;

fn main() {
    let mut v: Vector<i32> = Vector {
        data: vec![1, 2, 3]
    };
    let mut v2: Vector<i32> = Vector {
        data: vec![4, 5, 6]
    };
    v.add(&v2);
    v2.scl(5);
    println!("{}", v.data);
    println!("{}", v2.data);
}