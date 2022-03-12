// Imports
use crate::format::matrix::Matrix;

/// Implements matrix multiplication between two matrices and returns a third one
pub fn multiply_matrices(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    // Making sure matrices are of the same size
    if (mat1.col_num != mat2.row_num) {
        eprintln!(
            "Matrix 1 column number does not match matrix 2 row number, using default matrix"
        );
        return Matrix::new_matrix();
    }

    // Actually multiplying
    let mut mat3: Matrix = Matrix::new_dimension(mat1.row_num, mat2.col_num);
    for i in 0..mat1.row_num {
        for q in 0..mat2.col_num {
            let mut sum: f32 = 0.0;
            for r in 0..mat1.col_num {
                sum += mat1.data[r as usize][i as usize] * mat2.data[q as usize][r as usize];
            }
            mat3.data[q as usize][i as usize] = sum;
        }
    }

    return mat3;
}
