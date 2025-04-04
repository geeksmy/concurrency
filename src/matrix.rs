use crate::Vector;
use anyhow::Result;
use std::{
    fmt,
    ops::{Add, AddAssign, Mul},
};

pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

impl<T> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Self {
            data: data.into(),
            row,
            col,
        }
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T>
where
    T: fmt::Display,
{
    // display a 2x3 as [[1,2],[2,3], [3,4]], 3x2 as [[1,2,3],[2,3,4]]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..self.row {
            write!(f, "[")?;
            for j in 0..self.col {
                write!(f, "{}", self.data[i * self.col + j])?;
                if j < self.col - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, "]")?;
            if i < self.row - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix(row {}, col {}, {})", self.row, self.col, self)
    }
}

pub fn matrix_mul<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Copy + Default + AddAssign + Add<Output = T> + Mul<Output = T>,
{
    if a.col != b.row {
        return Err(anyhow::anyhow!("矩阵乘法错误: a.col != b.row"));
    }
    let mut data = vec![T::default(); a.row * b.col];
    for i in 0..a.row {
        for j in 0..b.col {
            let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let col_data = b.data[j..]
                .iter()
                .step_by(b.col)
                .copied()
                .collect::<Vec<_>>();
            let col = Vector::new(col_data);
            data[i * b.col + j] += dot_product(row, col)?;
        }
    }
    Ok(Matrix {
        data,
        row: a.row,
        col: b.col,
    })
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Copy + Default + AddAssign + Add<Output = T> + Mul<Output = T>,
{
    if a.len() != b.len() {
        return Err(anyhow::anyhow!("点乘错误: a.len() != b.len()"));
    }
    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_mul() -> Result<()> {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4, 5, 6], 3, 2);
        let c = matrix_mul(&a, &b)?;
        assert_eq!(c.row, 2);
        assert_eq!(c.col, 2);
        assert_eq!(c.data, [22, 28, 49, 64]);
        assert_eq!(
            format!("{:?}", c),
            "Matrix(row 2, col 2, [[22, 28], [49, 64]])"
        );

        Ok(())
    }

    #[test]
    fn test_dot_product() -> Result<()> {
        let a = Vector::new([1, 2, 3]);
        let b = Vector::new([4, 5, 6]);
        let c = dot_product(a, b)?;
        assert_eq!(c, 32);

        Ok(())
    }
}
