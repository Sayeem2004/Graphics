// Imports
use crate::algorithm::{light, line, shape};
use crate::format::{image::Image, pixel::Pixel};
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;
use std::fmt;

/// Matrix struct
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Matrix {
    pub row_num: i32,
    pub col_num: i32,
    pub data: Vec<Vec<f32>>,
}

// Implementing Constructors
impl Matrix {
    /// New default matrix of size 4 x 0
    pub fn new_matrix() -> Matrix {
        Matrix {
            row_num: 4,
            col_num: 0,
            data: vec![vec![0.00; 4_usize]; 0_usize],
        }
    }

    /// New matrix of size row_num x col_num
    pub fn new_dimension(row_num: i32, col_num: i32) -> Matrix {
        if (row_num < 0 || col_num < 0) {
            eprintln!("Matrix dimensions are out of range, using default matrix");
            return Matrix::new_matrix();
        }
        Matrix {
            row_num,
            col_num,
            data: vec![vec![0.00; row_num as usize]; col_num as usize],
        }
    }

    /// New transformation matrix of size 4 x 4
    pub fn new_transformation() -> Matrix {
        Matrix {
            row_num: 4,
            col_num: 4,
            data: vec![
                vec![1.00, 0.00, 0.00, 0.00],
                vec![0.00, 1.00, 0.00, 0.00],
                vec![0.00, 0.00, 1.00, 0.00],
                vec![0.00, 0.00, 0.00, 1.00],
            ],
        }
    }
}

// Implementing mutators
impl Matrix {
    /// Changing number of rows
    pub fn update_row_num(&mut self, row_num: i32) {
        if (row_num < 0) {
            eprintln!("Matrix row number is out of range, no changes made");
            return;
        }
        self.row_num = row_num;
        for col in self.data.iter_mut() {
            col.resize_with(self.row_num as usize, || 0.00);
        }
    }

    /// Changing number of columns
    pub fn update_col_num(&mut self, col_num: i32) {
        if (col_num < 0) {
            eprintln!("Matrix column number is out of range, no changes made");
            return;
        }
        self.col_num = col_num;
        self.data
            .resize_with(self.col_num as usize, || vec![0.00; self.row_num as usize]);
    }

    /// Changing a specific row in the matrix
    pub fn update_row(&mut self, ind: i32, row: &[f32]) {
        if (ind < 0 || ind >= self.row_num) {
            eprintln!("Matrix row index is out of range, no changes made");
            return;
        }
        if (row.len() != self.col_num as usize) {
            eprintln!("Inputted row does not match matrix row size, no changes made");
            return;
        }
        for i in 0..row.len() {
            self.data[i as usize][ind as usize] = row[i as usize];
        }
    }

    /// Changing a specific column in the matrix
    pub fn update_col(&mut self, ind: i32, col: &[f32]) {
        if (ind < 0 || ind >= self.col_num) {
            eprintln!("Matrix column index is out of range, no changes made");
            return;
        }
        if (col.len() != self.row_num as usize) {
            eprintln!("Inputted column does not match matrix column size, no changes made");
            return;
        }
        self.data[ind as usize] = col.to_vec();
    }

    /// Adding a row to the matrix
    pub fn add_row(&mut self, row: &[f32]) {
        if (row.len() != self.col_num as usize) {
            eprintln!("Inputted row does not match matrix row size, no changes made");
            return;
        }
        self.row_num += 1;
        for i in 0..row.len() {
            self.data[i as usize].push(row[i as usize]);
        }
    }

    /// Adding a column to the matrix
    pub fn add_col(&mut self, col: &[f32]) {
        if (col.len() != self.row_num as usize) {
            eprintln!("Inputted column does not match matrix column size, no changes made");
            return;
        }
        self.col_num += 1;
        self.data.push(col.to_vec());
    }
}

// Implementing formatting for the struct
impl fmt::Display for Matrix {
    /// Function for formatted printing
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // If nothing to print
        if (self.row_num == 0 || self.col_num == 0) {
            return writeln!(f);
        }

        // Iterating through matrix
        for i in 0..self.row_num {
            for q in 0..self.col_num {
                // Printing value
                write!(f, "{:.2} ", self.data[q as usize][i as usize])?;
            }
            // Printing new line
            writeln!(f)?;
        }

        // Printing final new line
        return writeln!(f);
    }
}

// Implementing utility functions for the struct
impl Matrix {
    /// Function clears all edges from a matrix
    pub fn clear(&mut self) {
        self.data.resize_with(0_usize, || vec![0.0, 0.0, 0.0, 0.0]);
        self.col_num = 0;
    }

    /// Function that copies all edges from another matrix to the current one
    pub fn copy(&mut self, mat: &Matrix) {
        self.clear();
        for col in &mat.data {
            self.add_col(col);
        }
    }

    /// Function that appends all edges from another matrix to the current one
    pub fn append(&mut self, mat: &Matrix) {
        for col in &mat.data {
            self.add_col(col);
        }
    }

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

        mat3
    }
}

// Implementing transformation functions for struct
impl Matrix {
    /// Function for making a dilation matrix
    pub fn dilate(dx: f32, dy: f32, dz: f32) -> Matrix {
        // Making new transformation matrix
        let mut mat: Matrix = Matrix::new_transformation();
        mat.data[0][0] = dx;
        mat.data[1][1] = dy;
        mat.data[2][2] = dz;

        // Returning transformation matrix
        mat
    }

    /// Function for making a translation matrix
    pub fn translate(dx: f32, dy: f32, dz: f32) -> Matrix {
        // Making new transformation matrix
        let mut mat: Matrix = Matrix::new_transformation();
        mat.data[3][0] = dx;
        mat.data[3][1] = dy;
        mat.data[3][2] = dz;

        // Returning transformation matrix
        mat
    }

    /// Function for creating a rotation matrix in radians
    pub fn rotate_radian(angle: f32, axis: &str) -> Matrix {
        // Casework on axes
        if (axis.eq("x")) {
            // Making new transformation matrix
            let mut mat: Matrix = Matrix::new_transformation();
            mat.data[1][1] = f32::cos(angle);
            mat.data[1][2] = f32::sin(angle);
            mat.data[2][1] = f32::sin(angle) * -1.0;
            mat.data[2][2] = f32::cos(angle);

            // Returning transformation matrix
            mat
        } else if (axis.eq("y")) {
            // Making new transformation matrix
            let mut mat: Matrix = Matrix::new_transformation();
            mat.data[0][0] = f32::cos(angle);
            mat.data[0][2] = f32::sin(angle) * -1.0;
            mat.data[2][0] = f32::sin(angle);
            mat.data[2][2] = f32::cos(angle);

            // Returning transformation matrix
            mat
        } else if (axis.eq("z")) {
            // Making new transformation matrix
            let mut mat: Matrix = Matrix::new_transformation();
            mat.data[0][0] = f32::cos(angle);
            mat.data[0][1] = f32::sin(angle);
            mat.data[1][0] = f32::sin(angle) * -1.0;
            mat.data[1][1] = f32::cos(angle);

            // Returning transformation matrix
            mat
        } else {
            eprintln!("Axis of rotation is not valid, returning default value");
            Matrix::new_transformation()
        }
    }

    /// Function for creating a rotation matrix in degrees
    pub fn rotate_degree(angle: f32, axis: &str) -> Matrix {
        Matrix::rotate_radian(angle * PI / 180.0, axis)
    }

    /// Implements the right tranformation of a matrix given the original matrix and a transformation matrix
    pub fn right_transform(&mut self, trans: &Matrix) {
        // Making sure current matrix is a tranformation matrix
        if (trans.row_num != 4 || trans.col_num != 4) {
            eprintln!("Transformation matrix given is not the right size, no changes made");
            return;
        }

        // Multiplying and copying things over
        *self = Matrix::multiply_matrices(self, trans);
    }

    /// Implments the left transformation of a matrix given the original matrix and a transformation matrix
    pub fn left_transform(&mut self, trans: &Matrix) {
        // Making sure current matrix is a transformation matrix
        if (trans.row_num != 4 || trans.col_num != 4) {
            eprintln!("Transformation amtrix given is not the right size, no changes made");
            return;
        }

        // Multiplying and copying things over
        *self = Matrix::multiply_matrices(trans, self);
    }
}

// Implementing drawing function for the struct
impl Matrix {
    /// Function for adding a point (x, y, z) to a matrix
    pub fn add_point(&mut self, p: (f32, f32, f32)) {
        if (self.row_num != 4) {
            eprintln!("Matrix row number does not equal four, no changes made");
            return;
        }
        self.add_col(&[p.0, p.1, p.2, 1.0]);
    }

    /// Function for adding an edge to a matrix
    pub fn add_edge(&mut self, p0: (f32, f32, f32), p1: (f32, f32, f32)) {
        if (self.row_num != 4) {
            eprintln!("Matrix row number does not equal four, no changes made");
            return;
        }
        self.add_point(p0);
        self.add_point(p1);
    }

    /// Function for drawing the edges found in a matrix on an image using xy orientation
    pub fn draw_lines_xy(&mut self, img: &mut Image, pix: Pixel) {
        if (self.row_num != 4) {
            eprintln!("Matrix row number does not equal four, no changes made");
            return;
        }
        for i in 0..self.col_num {
            if (i % 2 == 1) {
                line::draw_line(
                    (
                        self.data[(i - 1) as usize][0] as i32,
                        self.data[(i - 1) as usize][1] as i32,
                        self.data[(i - 1) as usize][2],
                    ),
                    (
                        self.data[i as usize][0] as i32,
                        self.data[i as usize][1] as i32,
                        self.data[i as usize][2],
                    ),
                    img,
                    pix,
                )
            }
        }
    }

    /// Function for drawing the edges found in a matrix on an image using rc orientation
    pub fn draw_lines_rc(&mut self, img: &mut Image, pix: Pixel) {
        if (self.row_num != 4) {
            eprintln!("Matrix row number does not equal four, no changes made");
            return;
        }
        for i in 0..self.col_num {
            if (i % 2 == 1) {
                line::draw_line(
                    (
                        self.data[(i - 1) as usize][0] as i32,
                        img.height - self.data[(i - 1) as usize][1] as i32,
                        self.data[(i - 1) as usize][2],
                    ),
                    (
                        self.data[i as usize][0] as i32,
                        img.height - self.data[i as usize][1] as i32,
                        self.data[i as usize][2],
                    ),
                    img,
                    pix,
                )
            }
        }
    }

    /// Function for adding a triangle to a matrix
    pub fn add_triangle(&mut self, p0: (f32, f32, f32), p1: (f32, f32, f32), p2: (f32, f32, f32)) {
        if (self.row_num != 4) {
            eprintln!("Matrix row number does not equal four, no changes made");
            return;
        }
        self.add_point(p0);
        self.add_point(p1);
        self.add_point(p2);
    }

    /// Function for drawing the triangles found in a matrix on an image using xy orientation
    pub fn draw_triangles_xy(
        &mut self,
        img: &mut Image,
        amb: Pixel,
        pnts: &[(Pixel, f32, f32, f32)],
        view: (f32, f32, f32),
        div: (f32, f32, f32, f32, f32, f32, f32, f32, f32),
    ) {
        // Error checking
        if (self.row_num != 4) {
            eprintln!("Matrix row number does not equal four, no changes made");
            return;
        }

        // Drawing triangles
        for i in 0..self.col_num {
            if (i % 3 == 2) {
                // Backface culling
                let normal = shape::normal(self, i as usize);
                if (light::dot(normal, view) <= 0.0) {
                    continue;
                }

                // Getting lighting value
                let surf = (
                    self.data[(i - 2) as usize][0],
                    self.data[(i - 2) as usize][1],
                    self.data[(i - 2) as usize][2],
                );
                let color: Pixel = light::calculate(amb, pnts, surf, view, normal, div);

                // Drawing polygon with color
                line::scanline(self, i, img, color);
            }
        }
    }
}
