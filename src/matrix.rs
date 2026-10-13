use std::ops::{Add, Sub, Mul, Div};
use std::fmt;
use std::convert::From;

use crate::matrix;
use crate::scalar::Field;
use crate::vector::Vector;

#[derive(Debug, Clone)]
pub struct Matrix<K> {
    pub data: Vec<K>,
    pub rows: usize,
    pub cols: usize,
}

struct EliminationResult<K> {
    matrix: Matrix<K>,
    augmented: Option<Matrix<K>>,
    pivot_product: K,
    swaps: usize,
    rank: usize,
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

    pub fn print_column_major(&self)
    where
        K: std::fmt::Display,
    {
        for col in 0..self.cols {
            for row in 0..self.rows {
                if row > 0 {
                    print!(", ");
                }

                print!(
                    "{}",
                    self.data[
                        row * self.cols + col
                    ]
                );
            }

            println!();
        }
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

    pub fn mul_mat(&self, w: &Matrix<K>) -> Matrix<K>
    {
        if self.cols != w.rows {
            panic!("Martrix shape mismatch");
        }

        let mut result = vec![K::zero(); self.rows * w.cols];

        for row in 0..self.rows {
            for col in 0..w.cols {
                let mut sum = K::zero();
                for k in 0..self.cols {
                    sum = sum + self.data[row * self.cols + k].clone()  * w.data[k * w.cols + col].clone();
                }
                result[row * w.cols + col] = sum;
            }
        }

        Matrix {
            data: result,
            rows: self.rows,
            cols: w.cols,
        }
    }

    pub fn trace(&self) -> K
    {
        if self.rows != self.cols {
            panic!("Trace is only defined for square matrices");
        }

        let mut sum = K::zero();
        for i in 0..self.rows {
            sum = sum + self.data[i * self.cols + i].clone();
        }
        sum
    }

    pub fn transpose(&self) -> Matrix<K>
    {
        let mut result = vec![K::zero(); self.rows * self.cols];

        for row in 0..self.rows {
            for col in 0..self.cols {
                result[col * self.rows + row] = self.data[row * self.cols + col].conj();
            }
        }

        Matrix {
            data: result,
            rows: self.cols,
            cols: self.rows,
        }
    }

    fn swap_rows(&mut self, row1: usize, row2: usize) {
        for col in 0..self.cols {
            self.data.swap(row1 * self.cols + col, row2 * self.cols + col);
        }
    }

    fn gauss_jordan(&self, mut augmented: Option<Matrix<K>>) -> Result<EliminationResult<K>, &'static str> {
        if let Some(ref aug) = augmented {
            if aug.rows != self.rows {
                return Err("Augmented matrix must have the same number of rows as the original matrix");
            }
        }

        let mut w = self.clone();

        let mut pivot_row = 0;
        let mut pivot_product = K::one();
        let mut swaps = 0;
        let mut rank = 0;

        for pivot_col in 0..w.cols {
            if pivot_row >= w.rows {
                break;
            }

            let mut selected_row = pivot_row;

            while selected_row < w.rows && w.data[selected_row * w.cols + pivot_col].is_zero() {
                selected_row += 1;
            }

            if selected_row == w.rows {
                continue;
            }

            if selected_row != pivot_row {
                w.swap_rows(selected_row, pivot_row);

                if let Some(ref mut aug) = augmented {
                    aug.swap_rows(selected_row, pivot_row);
                }

                swaps += 1;
            }

            //save pivot product
            let pivot = w.data[pivot_row * w.cols + pivot_col].clone();

            pivot_product = pivot_product * pivot.clone();

            //normalisation
            for col in 0..w.cols {
                let index = pivot_row * w.cols + col;
                w.data[index] = w.data[index].clone() / pivot.clone();
            }

            if let Some(ref mut aug) = augmented {
                for col in 0..aug.cols {
                    let index = pivot_row * aug.cols + col;
                    aug.data[index] = aug.data[index].clone() / pivot.clone();
                }
            }

            for row in 0..w.rows {
                if row == pivot_row {
                    continue;
                }

                let factor = w.data[row * w.cols + pivot_col].clone();

                if factor.is_zero() {
                    continue;
                }

                for col in 0..w.cols {
                    let index = row * w.cols + col;
                    let pivot_index = pivot_row * w.cols + col;

                    w.data[index] = w.data[index].clone() - factor.clone() * w.data[pivot_index].clone();
                }

                if let Some(ref mut aug) = augmented {
                    for col in 0..aug.cols {
                        let index = row * aug.cols + col;
                        let pivot_index = pivot_row * aug.cols + col;

                        aug.data[index] = aug.data[index].clone() - factor.clone() * aug.data[pivot_index].clone();
                    }
                }
            }

            pivot_row += 1;
            rank += 1;
        }

        Ok(EliminationResult {
            matrix: w,
            augmented,
            pivot_product,
            swaps,
            rank,
        })
    }

    pub fn row_echelon(&self) -> Matrix<K> {
        let result = self.gauss_jordan(None).unwrap();
        result.matrix
    }

    pub fn determinant(&self) -> K {
        if self.rows != self.cols {
            panic!("Determinant is only defined for square matrices");
        }

        if self.rows > 4 {
            panic!("Determinant calculation is only implemented for matrices up to 4x4");
        }

        let result = self.gauss_jordan(None).unwrap();

        if result.rank < self.rows {
            return K::zero();
        }

        if result.swaps % 2 != 0 {
            return K::zero() - result.pivot_product;
        }
        return result.pivot_product;
    }

    pub fn identity(size: usize) -> Matrix<K> {
        let mut data = vec![K::zero(); size * size];

        for i in 0..size {
            data[i * size + i] = K::one();
        }

        Matrix {
            data,
            rows: size,
            cols: size,
        }
    }

    pub fn inverse(&self) -> Result<Matrix<K>, &'static str>
    {
        if self.rows != self.cols {
            return Err("Inverse is only defined for square matrices");
        }

        let identity = Matrix::<K>::identity(self.rows);

        let result = self.gauss_jordan(Some(identity))?;

        if result.rank < self.rows {
            return Err("Matrix is singular and cannot be inverted");
        }

        result.augmented.ok_or("Augmented matrix not found after elimination")
    }

    pub fn rank(&self) -> usize {
        let result = self.gauss_jordan(None).unwrap();
        result.rank
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

