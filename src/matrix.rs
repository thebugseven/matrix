use std::ops::{Add, Sub, Mul, Div};
use std::fmt;
use std::convert::From;

use crate::scalar::Field;
use crate::vector::Vector;

#[derive(Debug, Clone)]
pub struct Matrix<K> {
    pub data: Vec<K>,
    pub rows: usize,
    pub cols: usize,
}

impl<K> From<Vec<K>> for Matrix<K> {
    fn from(vec: Vec<K>) -> Self {
        let rows = vec.len();
        let cols = 1;
        Matrix { data: vec, rows, cols }
    }
}

impl<K, const R: usize, const C: usize> From<[[K; C]; R]> for Matrix<K> {
    fn from(data: [[K; C]; R]) -> Self {
        let rows = R;
        let cols = C;
        Matrix { 
            data: data
                .into_iter()
                .flat_map(|row| row.into_iter())
                .collect(),
            rows,
            cols
        }
    }
}

impl<K> Matrix<K> {
    pub fn new(data: Vec<K>, rows: usize, cols: usize) -> Result<Self, &'static str> {
        if data.is_empty() {
            return Err("Matrix cannot be empty");
        }

        let total_elements = rows * cols;
        if data.len() != total_elements {
            return Err("Inconsistent matrix dimensions");
        }
        Ok(Self { data, rows, cols })
    }

    pub fn get(&self, row: usize, col:usize) -> Option<&K> {
        if row >= self.rows || col >= self.cols {
            return None;
        }
        Some(&self.data[row * self.cols + col])
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut K> {
        if row >= self.rows || col >= self.cols {
            return None;
        }
        Some(&mut self.data[row * self.cols + col])
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }
}

impl<K> Matrix<K>
where K: Field,
{
    pub fn add(&mut self, other: &Matrix<K>) -> Result<(), &'static str>
    {
        if self.shape() != other.shape() {
            return Err("Matrices must have the same dimensions for addition");
        }
        
        for i in 0..self.data.len(){
            self.data[i] = self.data[i].clone() + other.data[i].clone();
        }
        Ok(())
    }

    pub fn sub(&mut self, other: &Matrix<K>) -> Result<(), &'static str>
    {
        if self.shape() != other.shape() {
            return Err("Matrices must have the same dimensions for subtraction");
        }
        
        for i in 0..self.data.len(){
            self.data[i] = self.data[i].clone() - other.data[i].clone();
        }
        Ok(())
    }

    pub fn scl(&mut self, scalar: K) 
    {
        for i in 0..self.data.len() {
            self.data[i] = self.data[i].clone() * scalar.clone();
        }
    }

    pub fn mul_vec(&self, u: &Vector<K>) -> Vector<K>
    {
        if self.cols !=  u.data.len() {
            panic!("Matrix columns must match vector length for multiplication");
        }

        if self.rows == 0 || self.cols == 0 {
            panic!("Matrix cannot be empty");
        }

        let mut result = vec![K::zero(); self.rows];

        for row in 0..self.rows {
            let mut sum = K::zero();

            for col in 0..self.cols {
                sum = sum + self.data[row * self.cols + col].clone() * u.data[col].clone();
            }
            result[row] = sum;
        }
        Vector { data: result }
    }

    pub fn mul_mat(&self, A: &Matrix<K>) -> Matrix<K>
    {
        if self.cols != A.rows {
            panic!("Martrix shape mismatch");
        }

        let mut result = vec![K::zero(); self.rows * A.cols];

        for row in 0..self.rows {
            for col in 0..A.cols {
                let mut sum = K::zero();
                for k in 0..self.cols {
                    sum = sum + self.data[row * self.cols + k].clone()  * A.data[k * A.cols + col].clone();
                }
                result[row * A.cols + col] = sum;
            }
        }

        Matrix {
            data: result,
            rows: self.rows,
            cols: A.cols,
        }
    }
}


impl<K> Add for Matrix<K>
where K: Field,
{
    type Output = Matrix<K>;

    fn add(self, other: Matrix<K>) -> Matrix<K> {
        if self.shape() != other.shape() {
            panic!("Matrices must have the same dimensions for addition");
        }
        Matrix {
            data: self.data.into_iter().zip(other.data.into_iter()).map(|(a, b)| a + b).collect(),
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl<K> Mul<f32> for Matrix<K>
where K: Field,
{
    type Output = Matrix<K>;

    fn mul(self, scalar: f32) -> Matrix<K> {
        Matrix {
            data: self.data.into_iter().map(|value| value * scalar).collect(),
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl<K> Div<f32> for Matrix<K>
where K: Field,
{
    type Output = Matrix<K>;

    fn div(self, scalar: f32) -> Matrix<K> {
        Matrix {
            data: self.data.into_iter().map(|value| value / scalar).collect(),
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl<K> fmt::Display for Matrix<K>
where K: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        for row in 0..self.rows {
            write!(f, "[")?;
            for col in 0..self.cols {
                if col > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", self.data[row * self.cols + col])?;
            }
            if row != self.rows - 1 {
                writeln!(f, "]")?;
            } else {
                write!(f, "]")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}