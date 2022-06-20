// Imports
use crate::template::prev::ver07::format::matrix::Matrix;
use std::f32::consts::PI;

/// Function that adds triangles representing a rectangular prism to a matrix of triangles
pub fn add_box(poly: &mut Matrix, c: (f32, f32, f32), w: f32, h: f32, d: f32) {
    // Error checking
    if (w < 0.0 || h < 0.0 || d < 0.0) {
        eprintln!("Dimensions of box can not be negative, no changes made");
        return;
    }

    // Defining vertices
    let (x0, y0, z0) = (c.0 + 0.0, c.1 - 0.0, c.2 - 0.0);
    let (x1, y1, z1) = (c.0 + w, c.1 - 0.0, c.2 - 0.0);
    let (x2, y2, z2) = (c.0 + 0.0, c.1 - h, c.2 - 0.0);
    let (x3, y3, z3) = (c.0 + 0.0, c.1 - 0.0, c.2 - d);
    let (x4, y4, z4) = (c.0 + w, c.1 - h, c.2 - 0.0);
    let (x5, y5, z5) = (c.0 + w, c.1 - 0.0, c.2 - d);
    let (x6, y6, z6) = (c.0 + 0.0, c.1 - h, c.2 - d);
    let (x7, y7, z7) = (c.0 + w, c.1 - h, c.2 - d);

    // Adding triangles to matrix
    poly.add_triangle((x0, y0, z0), (x2, y2, z2), (x4, y4, z4));
    poly.add_triangle((x0, y0, z0), (x4, y4, z4), (x1, y1, z1));
    poly.add_triangle((x3, y3, z3), (x6, y6, z6), (x2, y2, z2));
    poly.add_triangle((x3, y3, z3), (x2, y2, z2), (x0, y0, z0));
    poly.add_triangle((x5, y5, z5), (x7, y7, z7), (x6, y6, z6));
    poly.add_triangle((x5, y5, z5), (x6, y6, z6), (x3, y3, z3));
    poly.add_triangle((x1, y1, z1), (x4, y4, z4), (x7, y7, z7));
    poly.add_triangle((x1, y1, z1), (x7, y7, z7), (x5, y5, z5));
    poly.add_triangle((x3, y3, z3), (x0, y0, z0), (x1, y1, z1));
    poly.add_triangle((x3, y3, z3), (x1, y1, z1), (x5, y5, z5));
    poly.add_triangle((x2, y2, z2), (x6, y6, z6), (x7, y7, z7));
    poly.add_triangle((x2, y2, z2), (x7, y7, z7), (x4, y4, z4));
}

/// Function that adds triangles representing a sphere to a matrix of triangles
pub fn add_sphere(poly: &mut Matrix, c: (f32, f32, f32), r: f32, itr: u32) {
    // Error checking
    if (r < 0.0) {
        eprintln!("Radius of sphere can not be negative, no changes made");
        return;
    }

    // Getting points on sphere
    let points: Matrix = gen_sphere(c, r, itr);

    // Adding triangles to poly matrix
    for i in 0..itr as usize {
        for q in 0..itr as usize {
            let mxi: usize = itr as usize;
            let mxq: usize = (itr + 1) as usize;
            let p: usize = (i * mxq + q);
            if (p % mxq < (mxq - 2)) {
                poly.add_triangle(
                    (points.data[p][0], points.data[p][1], points.data[p][2]),
                    (
                        points.data[p + 1][0],
                        points.data[p + 1][1],
                        points.data[p + 1][2],
                    ),
                    (
                        points.data[(p + mxq + 1) % (mxi * mxq)][0],
                        points.data[(p + mxq + 1) % (mxi * mxq)][1],
                        points.data[(p + mxq + 1) % (mxi * mxq)][2],
                    ),
                );
            }
            if (p % mxq > 0) {
                poly.add_triangle(
                    (points.data[p][0], points.data[p][1], points.data[p][2]),
                    (
                        points.data[(p + mxq + 1) % (mxi * mxq)][0],
                        points.data[(p + mxq + 1) % (mxi * mxq)][1],
                        points.data[(p + mxq + 1) % (mxi * mxq)][2],
                    ),
                    (
                        points.data[(p + mxq) % (mxi * mxq)][0],
                        points.data[(p + mxq) % (mxi * mxq)][1],
                        points.data[(p + mxq) % (mxi * mxq)][2],
                    ),
                );
            }
        }
    }
}

/// Function that adds triangles representing a torus to a matrix of triangles
pub fn add_torus(poly: &mut Matrix, c: (f32, f32, f32), r1: f32, r2: f32, itr: u32) {
    // Error checking
    if (r1 < 0.0 || r2 < 0.0) {
        eprintln!("Radii of the torus can not be negative, no changes made");
        return;
    }

    // Getting points on torus
    let points: Matrix = gen_torus(c, r1, r2, itr);

    // Adding triangles to poly matrix
    for i in 0..itr as usize {
        for q in 0..itr as usize {
            let mxi: usize = itr as usize;
            let mxq: usize = (itr + 1) as usize;
            let p: usize = (i * mxq + q) as usize;
            poly.add_triangle(
                (points.data[p][0], points.data[p][1], points.data[p][2]),
                (
                    points.data[(p + mxq + 1) % (mxi * mxq)][0],
                    points.data[(p + mxq + 1) % (mxi * mxq)][1],
                    points.data[(p + mxq + 1) % (mxi * mxq)][2],
                ),
                (
                    points.data[(p + 1) % (mxi * mxq)][0],
                    points.data[(p + 1) % (mxi * mxq)][1],
                    points.data[(p + 1) % (mxi * mxq)][2],
                ),
            );
            poly.add_triangle(
                (points.data[p][0], points.data[p][1], points.data[p][2]),
                (
                    points.data[(p + mxq) % (mxi * mxq)][0],
                    points.data[(p + mxq) % (mxi * mxq)][1],
                    points.data[(p + mxq) % (mxi * mxq)][2],
                ),
                (
                    points.data[(p + mxq + 1) % (mxi * mxq)][0],
                    points.data[(p + mxq + 1) % (mxi * mxq)][1],
                    points.data[(p + mxq + 1) % (mxi * mxq)][2],
                ),
            );
        }
    }
}

/// Function that returns the dot product of two vectors
pub fn dot(v1: &[f32], v2: &[f32]) -> f32 {
    // Error checking
    if (v1.len() != v2.len()) {
        eprintln!("Vector are not of the same dimension, returning default value");
        return 0.0;
    }

    // Iterating through vectors
    let mut sum: f32 = 0.0;
    for i in 0..v1.len() {
        sum += v1[i] * v2[i];
    }

    // Exiting function
    sum
}

/// Function that generates the points in a sphere
pub fn gen_sphere(c: (f32, f32, f32), r: f32, itr: u32) -> Matrix {
    // Error checking
    if (r < 0.0) {
        eprintln!("Radius of sphere can not be negative, returning default value");
        return Matrix::new_matrix();
    }

    // Variable declaration
    let mut ret: Matrix = Matrix::new_matrix();

    // Getting points on the surface
    for i in 0..itr {
        for q in 0..=itr {
            // Angle variables
            let phi: f32 = 2.0 * PI * (i as f32) / (itr as f32);
            let theta: f32 = PI * (q as f32) / (itr as f32);

            // Point coordinates
            let nx: f32 = r * f32::cos(theta) + c.0;
            let ny: f32 = r * f32::sin(theta) * f32::cos(phi) + c.1;
            let nz: f32 = r * f32::sin(theta) * f32::sin(phi) + c.2;

            // Saving points
            ret.add_col(&[nx, ny, nz, 1.0]);
        }
    }

    // Exiting function
    ret
}

/// Function that generates the points in a torus
pub fn gen_torus(c: (f32, f32, f32), r1: f32, r2: f32, itr: u32) -> Matrix {
    // Error checking
    if (r1 < 0.0 || r2 < 0.0) {
        eprintln!("Radii of the torus can not be negative, returning default value");
        return Matrix::new_matrix();
    }

    // Variable declaration
    let mut ret: Matrix = Matrix::new_matrix();

    // Getting points on the surface
    for i in 0..itr {
        for q in 0..=itr {
            // Angle variables
            let phi: f32 = 2.0 * PI * (i as f32) / (itr as f32);
            let theta: f32 = 2.0 * PI * (q as f32) / (itr as f32);

            // Point coordinates
            let nx: f32 = f32::cos(phi) * (r1 * f32::cos(theta) + r2) + c.0;
            let ny: f32 = r1 * f32::sin(theta) + c.1;
            let nz: f32 = (-f32::sin(phi)) * (r1 * f32::cos(theta) + r2) + c.2;

            // Saving points
            ret.add_col(&[nx, ny, nz, 1.0]);
        }
    }

    // Exiting function
    ret
}

/// Function that returns the surface normal vector of a triangle at a certain position in the triangle matrix
pub fn normal(poly: &Matrix, ind: usize) -> Vec<f32> {
    // Error checking
    if (ind >= poly.col_num as usize) {
        eprintln!("Index is out of range, returning default values");
        return vec![0.0; 3];
    }
    if (poly.row_num != 4) {
        eprintln!("Matrix is not of size Nx4, returning default values");
        return vec![0.0; 3];
    }

    // Getting vector components from vertices
    let v1: (f32, f32, f32) = (
        poly.data[ind - 1][0] - poly.data[ind - 2][0],
        poly.data[ind - 1][1] - poly.data[ind - 2][1],
        poly.data[ind - 1][2] - poly.data[ind - 2][2],
    );
    let v2: (f32, f32, f32) = (
        poly.data[ind][0] - poly.data[ind - 2][0],
        poly.data[ind][1] - poly.data[ind - 2][1],
        poly.data[ind][2] - poly.data[ind - 2][2],
    );

    // Returning the cross product
    return vec![
        (v1.1 * v2.2) - (v1.2 * v2.1),
        (v1.2 * v2.0) - (v1.0 * v2.2),
        (v1.0 * v2.1) - (v1.1 * v2.0),
    ];
}
