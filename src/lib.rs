mod matrix;
mod metrics;
mod vector;

pub use matrix::{Matrix, matrix_mul};
pub use metrics::{AmapMetrics, CmapMetrics};
pub use vector::{Vector, dot_product};
