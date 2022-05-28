// Imports
use crate::format::{constant, matrix::Matrix};
use std::f32::consts::PI;

/// Function that adds edges representing a circle to a matrix of edges
pub fn add_circle(edge: &mut Matrix, c: (f32, f32, f32), r: f32, itr: u32) {
    // Error checking
    if (r < 0.0) {
        eprintln!("Circle can not have negative radius, no changes made");
        return;
    }

    // Getting initial values
    let mut px: f32 = c.0 + r;
    let mut py: f32 = c.1;

    // Iterating through t values
    for i in 1..itr + 1 {
        // Getting new coordinates
        let t: f32 = (i as f32) / (itr as f32);
        let nx: f32 = c.0 + r * f32::cos(2.0 * PI * t);
        let ny: f32 = c.1 + r * f32::sin(2.0 * PI * t);

        // Adding to matrix
        edge.add_edge((px, py, c.2), (nx, ny, c.2));

        // Preparing for next loop
        px = nx;
        py = ny;
    }
}

/// Function that adds edges representing a bezier curve to a matrix of edges
pub fn add_bezier(
    edge: &mut Matrix,
    s: (f32, f32),
    e: (f32, f32),
    m1: (f32, f32),
    m2: (f32, f32),
    itr: u32,
) {
    // Getting x coefficients
    let mut xmat: Matrix = Matrix::new_matrix();
    xmat.add_col(&[s.0, e.0, m1.0, m2.0]);
    let xret: Matrix = Matrix::multiply_matrices(&constant::BEZIER, &xmat);
    let (ax, bx, cx, dx) = (
        xret.data[0][0],
        xret.data[0][1],
        xret.data[0][2],
        xret.data[0][3],
    );

    // Getting y coefficients
    let mut ymat: Matrix = Matrix::new_matrix();
    ymat.add_col(&[s.1, e.1, m1.1, m2.1]);
    let yret: Matrix = Matrix::multiply_matrices(&constant::BEZIER, &ymat);
    let (ay, by, cy, dy) = (
        yret.data[0][0],
        yret.data[0][1],
        yret.data[0][2],
        yret.data[0][3],
    );

    // Getting initial values
    let mut px: f32 = s.0;
    let mut py: f32 = s.1;

    // Iterating through t values
    for i in 1..itr + 1 {
        // Getting new coordinates
        let t: f32 = (i as f32) / (itr as f32);
        let nx: f32 = (ax * t * t * t) + (bx * t * t) + (cx * t) + dx;
        let ny: f32 = (ay * t * t * t) + (by * t * t) + (cy * t) + dy;

        // Adding to matrix
        edge.add_edge((px, py, 0.0), (nx, ny, 0.0));

        // Preparing for next loop
        px = nx;
        py = ny;
    }
}

/// Function that adds edges representing a hermite curve to a matrix of edges
pub fn add_hermite(
    edge: &mut Matrix,
    s: (f32, f32),
    e: (f32, f32),
    sr: (f32, f32),
    er: (f32, f32),
    itr: u32,
) {
    // Getting x coefficients
    let mut xmat: Matrix = Matrix::new_matrix();
    xmat.add_col(&[s.0, e.0, sr.0, er.0]);
    let xret: Matrix = Matrix::multiply_matrices(&constant::HERMITE, &xmat);
    let (ax, bx, cx, dx) = (
        xret.data[0][0],
        xret.data[0][1],
        xret.data[0][2],
        xret.data[0][3],
    );

    // Getting y coefficients
    let mut ymat: Matrix = Matrix::new_matrix();
    ymat.add_col(&[s.1, e.1, sr.1, er.1]);
    let yret: Matrix = Matrix::multiply_matrices(&constant::HERMITE, &ymat);
    let (ay, by, cy, dy) = (
        yret.data[0][0],
        yret.data[0][1],
        yret.data[0][2],
        yret.data[0][3],
    );

    // Getting initial values
    let mut px: f32 = s.0;
    let mut py: f32 = s.1;

    // Iterating through t values
    for i in 1..itr + 1 {
        // Getting new coordinates
        let t: f32 = (i as f32) / (itr as f32);
        let nx: f32 = (ax * t * t * t) + (bx * t * t) + (cx * t) + dx;
        let ny: f32 = (ay * t * t * t) + (by * t * t) + (cy * t) + dy;

        // Adding to matrix
        edge.add_edge((px, py, 0.0), (nx, ny, 0.0));

        // Preparing for next loop
        px = nx;
        py = ny;
    }
}
