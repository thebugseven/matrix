use std::ops::{Add, Sub, Mul,Div};
use std::convert::From;
use std::fmt;
use crate::scalar::Field;

#[derive(Debug, Clone)]
pub struct Vector<K> {
    pub data: Vec<K>,
}

impl<K> From<Vec<K>> for Vector<K> {
    fn from(vec: Vec<K>) -> Self {
        Vector { data: vec }
    }
}

impl<K, const N: usize> From<[K; N]> for Vector<K> {
    fn from(data: [K; N]) -> Self {
        Vector { data: Vec::from(data) }
    }
}

impl <K> Vector<K>
where K: Field,
{
    pub fn add(&mut self, other: &Vector<K>)
    {
        if self.data.len() != other.data.len() {
            panic!("Vectors must be of the same length");
        }
        for i in 0..self.data.len() {
            self.data[i] = self.data[i].clone() + other.data[i].clone();
        }
    }

    pub fn sub(&mut self, other: &Vector<K>)
    {
        if self.data.len() != other.data.len() {
            panic!("Vectors must be of the same length");
        }
        for i in 0..self.data.len() {
            self.data[i] = self.data[i].clone() - other.data[i].clone();
        }
    }

    pub fn scl(&mut self, scalar: K)
    {
        for i in 0..self.data.len() {
            self.data[i] = self.data[i].clone() * scalar.clone();
        }
    }

    pub fn dot(&self, v: Vector<K>) -> K
    {
        if self.data.len() != v.data.len() {
            panic!("Vectors must be of the same length");
        }

        if self.data.is_empty() {
            panic!("Vectors cannot be empty");
        }

        let mut result = K::zero();

        for i in 0..self.data.len() {
            result = result + self.data[i].conj() * v.data[i].clone();
        }

        result
    }

    pub fn norm_1(&self) -> f32
    {
        let mut sum = 0.0;
        for value in &self.data {
            sum += value.magnitude();
        }
        sum
    }

    pub fn norm_2(&self) -> f32
    {
        let mut sum = 0.0;
        for value in &self.data {
            sum += value.abs_square();
        }

        sum.sqrt()
    }

    pub fn norm_inf(&self) -> f32
    {
        let mut max = 0.0;
        for value in &self.data {
            let magnitude = value.magnitude();
            if magnitude > max {
                max = magnitude;
            }
        }

        max
    }
}

impl<K> Add for Vector<K>
where K: Add<Output = K>,
{
    type Output = Vector<K>;

    fn add(self, other: Vector<K>) -> Vector<K> {
        if self.data.len() != other.data.len() {
            panic!("Vectors must be of the same length");
        }

        Vector {
            data: self.data.into_iter().zip(other.data.into_iter()).map(|(a, b)| a + b).collect()
        }
    }
}

impl<K> Mul<f32> for Vector<K>
where K: Field,
{
    type Output = Vector<K>;

    fn mul(self, scalar: f32) -> Vector<K> {
        Vector {
            data: self.data.into_iter().map(|value| value * scalar).collect(),
        }
    }
}

impl<K> Div<f32> for Vector<K>
where K: Field,
{
    type Output = Vector<K>;

    fn div(self, scalar: f32) -> Vector<K> {
        Vector {
            data: self.data.into_iter().map(|value| value / scalar).collect(),
        }
    }
}

impl<K> fmt::Display for Vector<K>
where K: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (i, value) in self.data.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", value)?;
        }
        write!(f, "]")
    }
}
