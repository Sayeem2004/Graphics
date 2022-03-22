// Imports
use crate::format::matrix::Matrix;
use std::f32::consts::PI;

/// Function that adds edges representing a rectangular prism to a matrix of edges
pub fn add_box(edges: &mut Matrix, x: f32, y: f32, z: f32, w: f32, h: f32, d: f32) {
    // Defining vertices
    let (x0, y0, z0) = (x + 0.0, y - 0.0, z - 0.0);
    let (x1, y1, z1) = (x + w, y - 0.0, z - 0.0);
    let (x2, y2, z2) = (x + 0.0, y - h, z - 0.0);
    let (x3, y3, z3) = (x + 0.0, y - 0.0, z - d);
    let (x4, y4, z4) = (x + w, y - h, z - 0.0);
    let (x5, y5, z5) = (x + w, y - 0.0, z - d);
    let (x6, y6, z6) = (x + 0.0, y - h, z - d);
    let (x7, y7, z7) = (x + w, y - h, z - d);

    // Adding edges to matrix
    edges.add_edge(x0, y0, z0, x1, y1, z1);
    edges.add_edge(x0, y0, z0, x2, y2, z2);
    edges.add_edge(x0, y0, z0, x3, y3, z3);
    edges.add_edge(x1, y1, z1, x4, y4, z4);
    edges.add_edge(x1, y1, z1, x5, y5, z5);
    edges.add_edge(x2, y2, z2, x4, y4, z4);
    edges.add_edge(x2, y2, z2, x6, y6, z6);
    edges.add_edge(x3, y3, z3, x5, y5, z5);
    edges.add_edge(x3, y3, z3, x6, y6, z6);
    edges.add_edge(x4, y4, z4, x7, y7, z7);
    edges.add_edge(x5, y5, z5, x7, y7, z7);
    edges.add_edge(x6, y6, z6, x7, y7, z7);
}

/// Function that generates the points in a sphere
pub fn gen_sphere(x: f32, y: f32, z: f32, r: f32, itr: u32) -> Matrix {
    // Variable declaration
    let mut ret : Matrix = Matrix::new_matrix();

    // Getting points on the surface
    for i in 0..itr {
        for q in 0..=itr {
            // Angle variables
            let phi: f32 = 2.0 * PI * (i as f32) / (itr as f32);
            let theta: f32 = PI * (q as f32) / (itr as f32);

            // Point coordinates
            let nx: f32 = r * f32::cos(theta) + x;
            let ny: f32 = r * f32::sin(theta) * f32::cos(phi) + y;
            let nz: f32 = r * f32::sin(theta) * f32::sin(phi) + z;

            // Saving points
            ret.add_col(&vec![nx, ny, nz, 1.0]);
        }
    }

    // Exiting function
    return ret;
}

/// Function that adds edges representing a sphere to a matrix of edges
pub fn add_sphere(edges: &mut Matrix, x: f32, y: f32, z: f32, r: f32, itr: u32) {
    // Getting points on sphere
    let points: Matrix = gen_sphere(x, y, z, r, itr);

    // Adding points to edge matrix
    for p in points.data.iter() {
        edges.add_edge(p[0], p[1], p[2], p[0] + 1.0, p[1] + 1.0, p[2] + 1.0);
    }
}

/// Function that generates the points in a torus
pub fn gen_torus(x: f32, y: f32, z: f32, r1: f32, r2: f32, itr: u32) -> Matrix {
    // Variable declaration
    let mut ret : Matrix = Matrix::new_matrix();

    // Getting points on the surface
    for i in 0..itr {
        for q in 0..=itr {
            // Angle variables
            let phi: f32 = 2.0 * PI * (i as f32) / (itr as f32);
            let theta: f32 = 2.0 * PI * (q as f32) / (itr as f32);

            // Point coordinates
            let nx: f32 = f32::cos(phi) * (r1 * f32::cos(theta) + r2) + x;
            let ny: f32 = r1 * f32::sin(theta) + y;
            let nz: f32 = (-f32::sin(phi)) * (r1 * f32::cos(theta) + r2) + z;

            // Saving points
            ret.add_col(&vec![nx, ny, nz, 1.0]);
        }
    }

    // Exiting function
    return ret;
}

/// Function that adds edges representing a torus to a matrix of edges
pub fn add_torus(edges: &mut Matrix, x: f32, y: f32, z: f32, r1: f32, r2: f32, itr: u32) {
    // Getting points on torus
    let points: Matrix = gen_torus(x, y, z, r1, r2, itr);

    // Adding points to edge matrix
    for p in points.data.iter() {
        edges.add_edge(p[0], p[1], p[2], p[0] + 1.0, p[1] + 1.0, p[2] + 1.0);
    }
}
