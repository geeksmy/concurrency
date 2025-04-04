use crate::{Vector, dot_product};
use anyhow::Result;
use std::sync::mpsc;
use std::{
    fmt,
    ops::{Add, AddAssign, Mul},
    thread,
};

const NUN_THREADS: usize = 4;

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

impl<T> Mul for Matrix<T>
where
    T: Copy + Default + AddAssign + Add<Output = T> + Mul<Output = T> + Send + 'static,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        matrix_mul(&self, &rhs).expect("矩阵乘法错误!!!")
    }
}

pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

impl<T> MsgInput<T> {
    pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
    }
}

pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
}

impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
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
    T: Copy + Default + AddAssign + Add<Output = T> + Mul<Output = T> + Send + 'static,
{
    if a.col != b.row {
        return Err(anyhow::anyhow!("矩阵乘法错误: a.col != b.row"));
    }
    let senders = (0..NUN_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput {
                        idx: msg.input.idx,
                        value,
                    }) {
                        eprintln!("发送错误: {:?}", e);
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();

    let matrix_len = a.row * b.col;
    let mut data = vec![T::default(); matrix_len];
    let mut receivers = Vec::with_capacity(matrix_len);
    for i in 0..a.row {
        for j in 0..b.col {
            let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let col_data = b.data[j..]
                .iter()
                .step_by(b.col)
                .copied()
                .collect::<Vec<_>>();
            let col = Vector::new(col_data);
            let idx = i * b.col + j;
            let input = MsgInput::new(idx, row, col);
            let (tx, rx) = oneshot::channel();
            let msg = Msg::new(input, tx);
            if let Err(e) = senders[idx % NUN_THREADS].send(msg) {
                eprintln!("发送错误: {}", e)
            }
            receivers.push(rx);
        }
    }

    for rx in receivers {
        let ret = rx.recv()?;
        data[ret.idx] = ret.value;
    }

    Ok(Matrix {
        data,
        row: a.row,
        col: b.col,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_mul() -> Result<()> {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4, 5, 6], 3, 2);
        let c = a * b;
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
