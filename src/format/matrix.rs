// Imports
use crate::algorithm::{line, shape};
use crate::format::{constant, image::Image, pixel::Pixel};
use std::f32::consts::PI;
use std::fmt;

/// Matrix struct
#[derive(Clone)]
pub struct Matrix {
    pub row_num: i32,
    pub col_num: i32,
    pub data: Vec<Vec<f32>>,
}

// Implementing Constructors
impl Matrix {
    /// New default matrix of size 4 x 0
    pub fn new_matrix() -> Matrix {
        return Matrix {
            row_num: 4,
            col_num: 0,
            data: vec![vec![0.00; 4 as usize]; 0 as usize],
        };
    }

    /// New matrix of size row_num x col_num
    pub fn new_dimension(row_num: i32, col_num: i32) -> Matrix {
        if (row_num < 0 || col_num < 0) {
            eprintln!("Matrix dimensions are out of range, using default matrix");
            return Matrix::new_matrix();
        }
        return Matrix {
            row_num: row_num,
            col_num: col_num,
            data: vec![vec![0.00; row_num as usize]; col_num as usize],
        };
    }

    /// New transformation matrix of size 4 x 4
    pub fn new_transformation() -> Matrix {
        return Matrix {
            row_num: 4,
            col_num: 4,
            data: vec![
                vec![1.00, 0.00, 0.00, 0.00],
                vec![0.00, 1.00, 0.00, 0.00],
                vec![0.00, 0.00, 1.00, 0.00],
                vec![0.00, 0.00, 0.00, 1.00],
            ],
        };
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
    pub fn update_row(&mut self, ind: i32, row: &Vec<f32>) {
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
    pub fn update_col(&mut self, ind: i32, col: &Vec<f32>) {
        if (ind < 0 || ind >= self.col_num) {
            eprintln!("Matrix column index is out of range, no changes made");
            return;
        }
        if (col.len() != self.row_num as usize) {
            eprintln!("Inputted column does not match matrix column size, no changes made");
            return;
        }
        self.data[ind as usize] = col.clone();
    }

    /// Adding a row to the matrix
    pub fn add_row(&mut self, row: &Vec<f32>) {
        if (row.len() != self.col_num as usize) {
            eprintln!("Inputted row does not match matrix row size, no changes made");
            return;
        }
        self.row_num = self.row_num + 1;
        for i in 0..row.len() {
            self.data[i as usize].push(row[i as usize]);
        }
    }

    /// Adding a column to the matrix
    pub fn add_col(&mut self, col: &Vec<f32>) {
        if (col.len() != self.row_num as usize) {
            eprintln!("Inputted column does not match matrix column size, no changes made");
            return;
        }
        self.col_num = self.col_num + 1;
        self.data.push(col.clone());
    }
}

// Implementing formatting for the struct
impl fmt::Display for Matrix {
    /// Function for formatted printing
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // If nothing to print
        if (self.row_num == 0 || self.col_num == 0) {
            return write!(f, "\n");
        }

        // Iterating through matrix
        for i in 0..self.row_num {
            for q in 0..self.col_num {
                // Printing value
                write!(f, "{:.2} ", self.data[q as usize][i as usize])?;
            }
            // Printing new line
            write!(f, "\n")?;
        }

        // Printing final new line
        return write!(f, "\n");
    }
}

// Implementing utility functions for the struct
impl Matrix {
    /// Function clears all edges from a matrix
    pub fn clear(&mut self) {
        self.data
            .resize_with(0 as usize, || vec![0.0, 0.0, 0.0, 0.0]);
        self.col_num = 0;
    }

    /// Function that copies all edges from another matrix to the current one
    pub fn copy(&mut self, mat: &Matrix) {
        self.clear();
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

        return mat3;
    }
}

// Implementing transformation functions for struct
impl Matrix {
    /// Function for dilating a transformation matrix
    pub fn dilate(&mut self, dx: f32, dy: f32, dz: f32) {
        // Making sure current matrix is a tranformation matrix
        if (self.row_num != 4 || self.col_num != 4) {
            eprintln!("Matrix undergoing dilation is not a transformation matrix, no changes made");
            return;
        }

        // Making new transformation matrix
        let mut mat: Matrix = Matrix::new_transformation();
        mat.data[0][0] = dx;
        mat.data[1][1] = dy;
        mat.data[2][2] = dz;

        // Updating transformation matrix
        *self = Matrix::multiply_matrices(&mat, self);
    }

    /// Function for translating a transformation matrix
    pub fn translate(&mut self, dx: f32, dy: f32, dz: f32) {
        // Making sure current matrix is a tranformation matrix
        if (self.row_num != 4 || self.col_num != 4) {
            eprintln!(
                "Matrix undergoing translation is not a transformation matrix, no changes made"
            );
            return;
        }

        // Making new transformation matrix
        let mut mat: Matrix = Matrix::new_transformation();
        mat.data[3][0] = dx;
        mat.data[3][1] = dy;
        mat.data[3][2] = dz;

        // Updating transformation matrix
        *self = Matrix::multiply_matrices(&mat, self);
    }

    /// Function for rotating a transformation matrix in radians
    pub fn rotate_radian(&mut self, angle: f32, axis: &str) {
        // Making sure current matrix is a tranformation matrix
        if (self.row_num != 4 || self.col_num != 4) {
            eprintln!("Matrix undergoing rotation is not a transformation matrix, no changes made");
            return;
        }

        // Casework on axes
        if (axis.eq("x")) {
            // Making new transformation matrix
            let mut mat: Matrix = Matrix::new_transformation();
            mat.data[1][1] = f32::cos(angle);
            mat.data[1][2] = f32::sin(angle);
            mat.data[2][1] = f32::sin(angle) * -1.0;
            mat.data[2][2] = f32::cos(angle);

            // Updating transformation matrix
            *self = Matrix::multiply_matrices(&mat, self);
        } else if (axis.eq("y")) {
            // Making new transformation matrix
            let mut mat: Matrix = Matrix::new_transformation();
            mat.data[0][0] = f32::cos(angle);
            mat.data[0][2] = f32::sin(angle) * -1.0;
            mat.data[2][0] = f32::sin(angle);
            mat.data[2][2] = f32::cos(angle);

            // Updating transformation matrix
            *self = Matrix::multiply_matrices(&mat, self);
        } else if (axis.eq("z")) {
            // Making new transformation matrix
            let mut mat: Matrix = Matrix::new_transformation();
            mat.data[0][0] = f32::cos(angle);
            mat.data[0][1] = f32::sin(angle);
            mat.data[1][0] = f32::sin(angle) * -1.0;
            mat.data[1][1] = f32::cos(angle);

            // Updating transformation matrix
            *self = Matrix::multiply_matrices(&mat, self);
        } else {
            eprintln!("Axis of rotation is not valid, no changes made");
            return;
        }
    }

    /// Function for rotating a transformation matrix in degrees
    pub fn rotate_degree(&mut self, angle: f32, axis: &str) {
        self.rotate_radian(angle * PI / 180.0, axis);
    }

    /// Implements tranformation of a matrix given the original matrix and a transformation matrix
    pub fn matrix_transform(&mut self, trans: &Matrix) {
        // Making sure current matrix is a tranformation matrix
        if (trans.row_num != 4 || trans.col_num != 4) {
            eprintln!("Transformation matrix given is not the right size, no changes made");
            return;
        }

        // Multiplying and copying things over
        *self = Matrix::multiply_matrices(trans, self);
    }
}

// Implementing drawing function for the struct
impl Matrix {
    /// Function for adding a point (x, y, z) to a matrix
    pub fn add_point(&mut self, x: f32, y: f32, z: f32) {
        if (self.row_num != 4) {
            eprintln!("Matrix row number does not equal four, no changes made");
            return;
        }
        self.add_col(&vec![x, y, z, 1.0]);
    }

    /// Function for adding an edge (x0, y0, z0) (x1, y1, z1) to a matrix
    pub fn add_edge(&mut self, x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32) {
        if (self.row_num != 4) {
            eprintln!("Matrix row number does not equal four, no changes made");
            return;
        }
        self.add_point(x0, y0, z0);
        self.add_point(x1, y1, z1);
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
                    self.data[(i - 1) as usize][0] as i32,
                    self.data[(i - 1) as usize][1] as i32,
                    self.data[i as usize][0] as i32,
                    self.data[i as usize][1] as i32,
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
                    self.data[(i - 1) as usize][0] as i32,
                    img.get_height() - self.data[(i - 1) as usize][1] as i32,
                    self.data[i as usize][0] as i32,
                    img.get_height() - self.data[i as usize][1] as i32,
                    img,
                    pix,
                )
            }
        }
    }

    /// Function for adding a triangle to a matrix
    pub fn add_triangle(
        &mut self,
        x0: f32,
        y0: f32,
        z0: f32,
        x1: f32,
        y1: f32,
        z1: f32,
        x2: f32,
        y2: f32,
        z2: f32,
    ) {
        if (self.row_num != 4) {
            eprintln!("Matrix row number does not equal four, no changes made");
            return;
        }
        self.add_point(x0, y0, z0);
        self.add_point(x1, y1, z1);
        self.add_point(x2, y2, z2);
    }

    /// Function for drawing the triangles found in a matrix on an image using xy orientation
    pub fn draw_triangles_xy(&mut self, img: &mut Image, pix: Pixel) {
        if (self.row_num != 4) {
            eprintln!("Matrix row number does not equal four, no changes made");
            return;
        }
        for i in 0..self.col_num {
            if (i % 3 == 2) {
                let normal = shape::normal(
                    &(self.data[(i - 2) as usize]),
                    &(self.data[(i - 1) as usize]),
                    &(self.data[(i - 0) as usize]),
                );
                if (shape::dot(&normal, &constant::ZVIEW) <= 0.0) {
                    continue;
                }
                line::draw_line(
                    self.data[(i - 2) as usize][0] as i32,
                    self.data[(i - 2) as usize][1] as i32,
                    self.data[(i - 1) as usize][0] as i32,
                    self.data[(i - 1) as usize][1] as i32,
                    img,
                    pix,
                );

                line::draw_line(
                    self.data[(i - 1) as usize][0] as i32,
                    self.data[(i - 1) as usize][1] as i32,
                    self.data[(i - 0) as usize][0] as i32,
                    self.data[(i - 0) as usize][1] as i32,
                    img,
                    pix,
                );

                line::draw_line(
                    self.data[(i - 0) as usize][0] as i32,
                    self.data[(i - 0) as usize][1] as i32,
                    self.data[(i - 2) as usize][0] as i32,
                    self.data[(i - 2) as usize][1] as i32,
                    img,
                    pix,
                );
            }
        }
    }

    /// Function for drawing the triangles found in a matrix on an image using rc orientation
    pub fn draw_triangles_rc(&mut self, img: &mut Image, pix: Pixel) {
        if (self.row_num != 4) {
            eprintln!("Matrix row number does not equal four, no changes made");
            return;
        }
        for i in 0..self.col_num {
            if (i % 3 == 2) {
                let normal = shape::normal(
                    &vec![
                        self.data[(i - 2) as usize][0],
                        img.get_height() as f32 - self.data[(i - 2) as usize][1],
                        self.data[(i - 2) as usize][1],
                    ],
                    &vec![
                        self.data[(i - 1) as usize][0],
                        img.get_height() as f32 - self.data[(i - 1) as usize][1],
                        self.data[(i - 1) as usize][1],
                    ],
                    &vec![
                        self.data[(i - 0) as usize][0],
                        img.get_height() as f32 - self.data[(i - 0) as usize][1],
                        self.data[(i - 0) as usize][1],
                    ],
                );
                if (shape::dot(&normal, &constant::ZVIEW) <= 0.0) {
                    continue;
                }
                line::draw_line(
                    self.data[(i - 2) as usize][0] as i32,
                    img.get_height() - self.data[(i - 2) as usize][1] as i32,
                    self.data[(i - 1) as usize][0] as i32,
                    img.get_height() - self.data[(i - 1) as usize][1] as i32,
                    img,
                    pix,
                );

                line::draw_line(
                    self.data[(i - 1) as usize][0] as i32,
                    img.get_height() - self.data[(i - 1) as usize][1] as i32,
                    self.data[(i - 0) as usize][0] as i32,
                    img.get_height() - self.data[(i - 0) as usize][1] as i32,
                    img,
                    pix,
                );

                line::draw_line(
                    self.data[(i - 0) as usize][0] as i32,
                    img.get_height() - self.data[(i - 0) as usize][1] as i32,
                    self.data[(i - 2) as usize][0] as i32,
                    img.get_height() - self.data[(i - 2) as usize][1] as i32,
                    img,
                    pix,
                );
            }
        }
    }
}
