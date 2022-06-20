// Imports
use crate::format::{constant, image::Image, matrix::Matrix, pixel::Pixel};
use std::f32::consts::PI;

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
    let (ax, bx, cx, dx): (f32, f32, f32, f32) = (
        xret.data[0][0],
        xret.data[0][1],
        xret.data[0][2],
        xret.data[0][3],
    );

    // Getting y coefficients
    let mut ymat: Matrix = Matrix::new_matrix();
    ymat.add_col(&[s.1, e.1, m1.1, m2.1]);
    let yret: Matrix = Matrix::multiply_matrices(&constant::BEZIER, &ymat);
    let (ay, by, cy, dy): (f32, f32, f32, f32) = (
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
    let (ax, bx, cx, dx): (f32, f32, f32, f32) = (
        xret.data[0][0],
        xret.data[0][1],
        xret.data[0][2],
        xret.data[0][3],
    );

    // Getting y coefficients
    let mut ymat: Matrix = Matrix::new_matrix();
    ymat.add_col(&[s.1, e.1, sr.1, er.1]);
    let yret: Matrix = Matrix::multiply_matrices(&constant::HERMITE, &ymat);
    let (ay, by, cy, dy): (f32, f32, f32, f32) = (
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

/// Function that draws an arbitrary line by using the 4 octants above
pub fn draw_line(s: (i32, i32, f32), e: (i32, i32, f32), img: &mut Image, pix: Pixel) {
    // Quadrant 1, 2, 7, 8 cases
    if (e.0 >= s.0) {
        // Quadrant 1, 2
        if (e.1 >= s.1) {
            // Quadrant 2
            if (e.1 - s.1 > e.0 - s.0) {
                draw_oct2(img, pix, s, e);
            }
            // Quadrant 1
            else {
                draw_oct1(img, pix, s, e);
            }
        } else {
            // Quadrant 7, 8
            // Quadrant 7
            if (s.1 - e.1 > e.0 - s.0) {
                draw_oct7(img, pix, s, e);
            }
            // Quadrant 8
            else {
                draw_oct8(img, pix, s, e);
            }
        }
    } else {
        // Quadrant 3, 4, 5, 6 cases
        // Quadrant 3, 4
        if (e.1 >= s.1) {
            // Quadrant 3
            if (e.1 - s.1 > s.0 - e.0) {
                draw_oct7(img, pix, e, s);
            }
            // Quadrant 4
            else {
                draw_oct8(img, pix, e, s);
            }
        } else {
            // Quadrant 5, 6
            // Quadrant 6
            if (s.1 - e.1 > s.0 - e.0) {
                draw_oct2(img, pix, e, s);
            }
            // Quadrant 5
            else {
                draw_oct1(img, pix, e, s);
            }
        }
    }
}

/// Drawing a line in octant I
fn draw_oct1(img: &mut Image, pix: Pixel, s: (i32, i32, f32), e: (i32, i32, f32)) {
    // Variable declarations
    let (mut x, mut y, mut z): (i32, i32, f32) = (s.0, s.1, s.2);
    let a: i32 = 2 * (e.1 - s.1);
    let b: i32 = 2 * (s.0 - e.0);
    let dz: f32 = (e.2 - s.2) / (e.0 - s.0 + 1).abs() as f32;
    let mut d: i32 = a + b / 2;

    // Looping through range
    while (x <= e.0) {
        // Changing pixel
        img.update_pixel_xy(x, y, z, pix);

        // Updating y value if necessary
        if (d > 0) {
            y += 1;
            d += b;
        }

        // Necessary updates
        z += dz;
        x += 1;
        d += a;
    }
}

/// Drawing a line in octant II
fn draw_oct2(img: &mut Image, pix: Pixel, s: (i32, i32, f32), e: (i32, i32, f32)) {
    // Variable declarations
    let (mut x, mut y, mut z): (i32, i32, f32) = (s.0, s.1, s.2);
    let a: i32 = 2 * (e.1 - s.1);
    let b: i32 = 2 * (s.0 - e.0);
    let dz: f32 = (e.2 - s.2) / (e.1 - s.1 + 1).abs() as f32;
    let mut d: i32 = a / 2 + b;

    // Looping through range
    while (y <= e.1) {
        // Changing pixel
        img.update_pixel_xy(x, y, z, pix);

        // Updating y value if necessary
        if (d < 0) {
            x += 1;
            d += a;
        }

        // Necessary updates
        z += dz;
        y += 1;
        d += b;
    }
}

/// Drawing a line in octant VII
fn draw_oct7(img: &mut Image, pix: Pixel, s: (i32, i32, f32), e: (i32, i32, f32)) {
    // Variable declarations
    let (mut x, mut y, mut z): (i32, i32, f32) = (s.0, s.1, s.2);
    let a: i32 = 2 * (s.1 - e.1);
    let b: i32 = 2 * (s.0 - e.0);
    let dz: f32 = (e.2 - s.2) / (e.1 - s.1 + 1).abs() as f32;
    let mut d: i32 = a / 2 + b;

    // Looping through range
    while (y >= e.1) {
        // Changing pixel
        img.update_pixel_xy(x, y, z, pix);

        // Updating x value if necessary
        if (d < 0) {
            x += 1;
            d += a;
        }

        // Necessary updates
        z += dz;
        if (y > 0) {
            y -= 1;
        } else {
            break;
        }
        d += b;
    }
}

/// Drawing a line in octant VIII
fn draw_oct8(img: &mut Image, pix: Pixel, s: (i32, i32, f32), e: (i32, i32, f32)) {
    // Variable declarations
    let (mut x, mut y, mut z): (i32, i32, f32) = (s.0, s.1, s.2);
    let a: i32 = 2 * (s.1 - e.1);
    let b: i32 = 2 * (s.0 - e.0);
    let dz: f32 = (e.2 - s.2) / (e.0 - s.0 + 1).abs() as f32;
    let mut d: i32 = a + b / 2;

    // Looping through range
    while (x <= e.0) {
        // Changing pixel
        img.update_pixel_xy(x, y, z, pix);

        // Updating y value if necessary
        if (d > 0) {
            if (y > 0) {
                y -= 1;
            } else {
                break;
            }
            d += b;
        }

        // Necessary updates
        z += dz;
        x += 1;
        d += a;
    }
}
