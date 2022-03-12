// Imports
use crate::algorithm::matrix;
use crate::format::{constant, matrix::Matrix};
use std::f32::consts::PI;

/// Function that adds points representing a circle to a matrix of edges.
pub fn add_circle(edges: &mut Matrix, cx: f32, cy: f32, cz: f32, r: f32, step: f32) {
    // Getting initial values
    let mut px: f32 = cx + r;
    let mut py: f32 = cy;
    let mut t: f32 = step;

    // Iterating through t values
    while (t < 1.0) {
        // Getting new coordinates
        let nx: f32 = cx + r * f32::cos(2.0 * PI * t);
        let ny: f32 = cy + r * f32::sin(2.0 * PI * t);

        // Adding to matrix
        edges.add_edge(px, py, cz, nx, ny, cz);

        // Preparing for next loop
        px = nx;
        py = ny;
        t += step;
    }

    // Final iteration
    let nx: f32 = cx + r * f32::cos(2.0 * PI);
    let ny: f32 = cy + r * f32::sin(2.0 * PI);
    edges.add_edge(px, py, cz, nx, ny, cz);
}

/// Function that adds points representing a hermite curve to a matrix of edges
pub fn add_hermite(
    edges: &mut Matrix,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    rx0: f32,
    ry0: f32,
    rx1: f32,
    ry1: f32,
    step: f32,
) {
    // Getting x coefficients
    let mut xmat: Matrix = Matrix::new_matrix();
    xmat.add_col(&vec![x0, x1, rx0, rx1]);
    let xret: Matrix = matrix::multiply_matrices(&constant::HERMITE, &xmat);
    let (ax, bx, cx, dx) = (
        xret.data[0][0],
        xret.data[0][1],
        xret.data[0][2],
        xret.data[0][3],
    );

    // Getting y coefficients
    let mut ymat: Matrix = Matrix::new_matrix();
    ymat.add_col(&vec![y0, y1, ry0, ry1]);
    let yret: Matrix = matrix::multiply_matrices(&constant::HERMITE, &ymat);
    let (ay, by, cy, dy) = (
        yret.data[0][0],
        yret.data[0][1],
        yret.data[0][2],
        yret.data[0][3],
    );

    // Getting initial values
    let mut px: f32 = x0;
    let mut py: f32 = y0;
    let mut t: f32 = step;

    // Iterating through t values
    while (t < 1.0) {
        // Getting new coordinates
        let nx: f32 = (ax * t * t * t) + (bx * t * t) + (cx * t) + dx;
        let ny: f32 = (ay * t * t * t) + (by * t * t) + (cy * t) + dy;

        // Adding to matrix
        edges.add_edge(px, py, 0.0, nx, ny, 0.0);

        // Preparing for next loop
        px = nx;
        py = ny;
        t += step;
    }

    // Final iteration
    let nx: f32 = ax + bx + cx + dx;
    let ny: f32 = ay + by + cy + dy;
    edges.add_edge(px, py, 0.0, nx, ny, 0.0);
}

/// Function that adds points representing a bezier curve to a matrix of edges
pub fn add_bezier(
    edges: &mut Matrix,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
    step: f32,
) {
    // Getting x coefficients
    let mut xmat: Matrix = Matrix::new_matrix();
    xmat.add_col(&vec![x0, x1, x2, x3]);
    let xret: Matrix = matrix::multiply_matrices(&constant::BEZIER, &xmat);
    let (ax, bx, cx, dx) = (
        xret.data[0][0],
        xret.data[0][1],
        xret.data[0][2],
        xret.data[0][3],
    );

    // Getting y coefficients
    let mut ymat: Matrix = Matrix::new_matrix();
    ymat.add_col(&vec![y0, y1, y2, y3]);
    let yret: Matrix = matrix::multiply_matrices(&constant::BEZIER, &ymat);
    let (ay, by, cy, dy) = (
        yret.data[0][0],
        yret.data[0][1],
        yret.data[0][2],
        yret.data[0][3],
    );

    // Getting initial values
    let mut px: f32 = x0;
    let mut py: f32 = y0;
    let mut t: f32 = step;

    // Iterating through t values
    while (t < 1.0 + step) {
        // Getting new coordinates
        let nx: f32 = (ax * t * t * t) + (bx * t * t) + (cx * t) + dx;
        let ny: f32 = (ay * t * t * t) + (by * t * t) + (cy * t) + dy;

        // Adding to matrix
        edges.add_edge(px, py, 0.0, nx, ny, 0.0);

        // Preparing for next loop
        px = nx;
        py = ny;
        t += step;
    }

    // Final iteration
    let nx: f32 = ax + bx + cx + dx;
    let ny: f32 = ay + by + cy + dy;
    edges.add_edge(px, py, 0.0, nx, ny, 0.0);
}
