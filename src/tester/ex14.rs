use crate::math::projection;
use crate::scalar::Field;
use crate::matrix::Matrix;

pub fn run(){
    let p = projection(
        std::f32::consts::FRAC_PI_2,
        16.0 / 9.0,
        0.1,
        100.0,
    );

    p.print_column_major();
}