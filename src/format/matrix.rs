// Imports
use crate::format::{image::Image, pixel::Pixel};
use crate::algorithm::line;
use std::fmt;

/// Matrix struct
#[derive(Clone)]
pub struct Matrix {
    pub row_num : i32,
    pub col_num : i32,
    pub data : Vec<Vec<f32>>
}

// Implementing Constructors
impl Matrix {
    /// New default matrix of size 4 x 0
    pub fn new_matrix() -> Matrix {
        return Matrix {
            row_num : 4,
            col_num : 0,
            data : vec![vec![0.00; 4 as usize]; 0 as usize]
        };
    }

    /// New matrix of size row_num x col_num
    pub fn new_dimension(row_num : i32, col_num : i32) -> Matrix {
        if (row_num < 0 || col_num < 0) {
            println!("Matrix dimensions are out of range, using default matrix");
            return Matrix::new_matrix();
        }
        return Matrix {
            row_num : row_num,
            col_num : col_num,
            data : vec![vec![0.00; row_num as usize]; col_num as usize]
        }
    }

    /// New identity matrix of size 4 x 4
    pub fn new_identity() -> Matrix {
        return Matrix {
            row_num : 4,
            col_num : 4,
            data : vec![
                vec![1.00, 0.00, 0.00, 0.00],
                vec![0.00, 1.00, 0.00, 0.00],
                vec![0.00, 0.00, 1.00, 0.00],
                vec![0.00, 0.00, 0.00, 1.00]
            ]
        }
    }
}

// Implementing mutators
impl Matrix {
    /// Changing number of rows
    pub fn update_row_num(&mut self, row_num : i32) {
        if (row_num < 0) {
            println!("Matrix row number is out of range, no changes made");
            return;
        }
        self.row_num = row_num;
        for col in self.data.iter_mut() {
            col.resize_with(self.row_num as usize, || 0.00);
        }
    }

    /// Changing number of columns
    pub fn update_col_num(&mut self, col_num : i32) {
        if (col_num < 0) {
            println!("Matrix column number is out of range, no changes made");
            return;
        }
        self.col_num = col_num;
        self.data.resize_with(self.col_num as usize, || vec![0.00; self.row_num as usize]);
    }

    /// Changing a specific row in the matrix
    pub fn update_row(&mut self, ind : i32, row : Vec<f32>) {
        if (ind < 0 || ind >= self.row_num) {
            println!("Matrix row index is out of range, no changes made");
            return;
        }
        if (row.len() != self.col_num as usize) {
            println!("Inputted row does not match matrix row size, no changes made");
            return;
        }
        for i in 0..row.len() {
            self.data[i as usize][ind as usize] = row[i as usize];
        }
    }

    /// Changing a specific column in the matrix
    pub fn update_col(&mut self, ind : i32, col : Vec<f32>) {
        if (ind < 0 || ind >= self.col_num) {
            println!("Matrix column index is out of range, no changes made");
            return;
        }
        if (col.len() != self.row_num as usize) {
            println!("Inputted column does not match matrix column size, no changes made");
            return;
        }
        self.data[ind as usize] = col;
    }

    /// Adding a row to the matrix
    pub fn add_row(&mut self, row : Vec<f32>) {
        if (row.len() != self.col_num as usize) {
            println!("Inputted row does not match matrix row size, no changes made");
            return;
        }
        self.row_num = self.row_num + 1;
        for i in 0..row.len() {
            self.data[i as usize].push(row[i as usize]);
        }
    }

    /// Adding a column to the matrix
    pub fn add_col(&mut self, col : Vec<f32>) {
        if (col.len() != self.row_num as usize) {
            println!("Inputted column does not match matrix column size, no changes made");
            return;
        }
        self.col_num = self.col_num + 1;
        self.data.push(col);
    }
}

// Implementing formatting for the struct
impl fmt::Display for Matrix {
    /// Function for formatted printing
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
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
    /// Function for adding a point (x, y, z) to a matrix
    pub fn add_point(&mut self, x : f32, y : f32, z : f32) {
        if (self.row_num != 4) {
            println!("Matrix row number does not equal four, no changes made");
            return;
        }
        self.add_col(vec![x, y, z, 1.0]);
    }

    /// Function for adding an edge (x0, y0, z0) (x1, y1, z1) to a matrix
    pub fn add_edge(&mut self, x0 : f32, y0 : f32, z0 : f32, x1 : f32, y1 : f32, z1 : f32) {
        if (self.row_num != 4) {
            println!("Matrix row number does not equal four, no changes made");
            return;
        }
        self.add_point(x0, y0, z0);
        self.add_point(x1, y1, z1);
    }

    /// Function for drawing the edges found in a matrix on an image
    pub fn draw_lines(&mut self, img : &mut Image, pix : Pixel) {
        if (self.row_num != 4) {
            println!("Matrix row number does not equal four, no changes made");
            return;
        }
        for i in 0..self.col_num {
            if (i % 2 == 1) {
                line::draw_line(
                    self.data[(i-1) as usize][0] as i32,
                    self.data[(i-1) as usize][1] as i32,
                    self.data[i as usize][0] as i32,
                    self.data[i as usize][1] as i32,
                    img,
                    pix
                )
            }
        }
    }

    /// Function clears all edges from a matrix
    pub fn clear(&mut self) {
        self.data.resize_with(0 as usize, || vec![0.0, 0.0, 0.0, 0.0]);
        self.col_num = 0;
    }
}
