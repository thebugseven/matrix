use crate::{matrix::Matrix, vector::Vector};
use crate::scalar::Field;
use std::ops::{Add, Mul, Div};

pub fn linear_combination<K>(u: &[Vector<K>], coefs: &[K]) -> Vector<K> 
where K: Field,
{
    if u.len() != coefs.len() {
        panic!("Vectors and coefficients must have the same length");
    }

    if u.is_empty() {
        panic!("Input vectors cannot be empty");
    }

    let mut result = Vector {
        data: vec![K::zero(); u[0].data.len()],
    };

    for i in 0..u.len() {
        let mut scaled_vector = u[i].clone();
        scaled_vector.scl(coefs[i].clone());
        Vector::add(&mut result, &scaled_vector);
    }
    result
}

pub fn lerp<V>(u: V, v: V, t: f32) -> V
where V: Add<Output = V> + Mul<f32, Output = V>,
{
    u * (1.0 - t) + v * t
}

pub fn angle_cos<K>(u: &Vector<K>, v: &Vector<K>) -> f32
where K: Field + std::fmt::Display,
{
    if u.data.len() != v.data.len() {
        panic!("Vectors must be of the same length");
    }

    let dot_product = u.dot(v.clone());

    let norm_u = u.norm_2();
    let norm_v = v.norm_2();


    if norm_u == 0.0 || norm_v == 0.0 {
        panic!("Cannot compute angle with zero-length vector");
    }

    dot_product.angle_numerator() / (norm_u * norm_v)
}

pub fn cross_product<K>(u: &Vector<K>, v: &Vector<K>) -> Vector<K>
where K: Field,
{
    if u.data.len() != 3 || v.data.len() != 3 {
        panic!("Cross product is only defined for 3D vectors");
    }

    let x = u.data[1].clone() * v.data[2].clone() - u.data[2].clone() * v.data[1].clone();
    let y = u.data[2].clone() * v.data[0].clone() - u.data[0].clone() * v.data[2].clone();
    let z = u.data[0].clone() * v.data[1].clone() - u.data[1].clone() * v.data[0].clone();

    Vector {
        data: vec![x, y, z],
    }

}


pub fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix<f32> {
    if ratio <= 0.0 {
        panic!("Aspect ratio must be positive");
    }

    if near <= 0.0 || far <= near {
        panic!("Expected 0 < near < far");
    }

    let scale = 1.0 / (fov / 2.0).tan();

    let x_sscale = scale / ratio;
    let y_scale = scale;

    let z_scale = far / (near - far);
    let z_offset = (far * near) / (near - far);

    Matrix::from([
        [x_sscale, 0.0, 0.0, 0.0],
        [0.0, y_scale, 0.0, 0.0],
        [0.0, 0.0, z_scale, z_offset],
        [0.0, 0.0, -1.0, 0.0],
    ])

}