use std::ops::{Add, Sub, Mul};

pub struct Vector<K> {
    pub data: Vec<K>,
}

impl <K> Vector<K> {
    pub fn add(&mut self, other: &Vector<K>)
    where K: Add<Output = K> + Clone,
    {
        if self.data.len() != other.data.len() {
            panic!("Vectors must be of the same length");
        }
        for i in 0..self.data.len() {
            self.data[i] = self.data[i].clone() + other.data[i].clone();
        }
    }

    pub fn sub(&mut self, other: &Vector<K>)
    where K: Sub<Output = K> + Clone,
    {
        if self.data.len() != other.data.len() {
            panic!("Vectors must be of the same length");
        }
        for i in 0..self.data.len() {
            self.data[i] = self.data[i].clone() - other.data[i].clone();
        }
    }

    pub fn scl(&mut self, scalar: K)
    where K: Mul<Output = K> + Clone,
    {
        for i in 0..self.data.len() {
            self.data[i] = self.data[i].clone() * scalar.clone();
        }
    }
}