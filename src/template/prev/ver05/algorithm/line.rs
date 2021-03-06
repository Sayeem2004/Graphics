// Imports
use crate::template::prev::ver05::format::{image::Image, pixel::Pixel};
use std::f32::consts::PI;

/// Drawing a line in octant I
fn draw_oct1(img: &mut Image, pix: Pixel, x0: i32, y0: i32, x1: i32, y1: i32) {
    // Variable declarations
    let (mut x, mut y): (i32, i32) = (x0, y0);
    let a: i32 = 2 * (y1 - y0);
    let b: i32 = 2 * (x0 - x1);
    let mut d: i32 = a + b / 2;

    // Looping through range
    while (x <= x1) {
        // Changing pixel
        img.update_pixel_xy(x, y, pix);

        // Updating y value if necessary
        if (d > 0) {
            y += 1;
            d += b;
        }

        // Necessary updates
        x += 1;
        d += a;
    }
}

/// Drawing a line in octant II
fn draw_oct2(img: &mut Image, pix: Pixel, x0: i32, y0: i32, x1: i32, y1: i32) {
    // Variable declarations
    let (mut x, mut y): (i32, i32) = (x0, y0);
    let a: i32 = 2 * (y1 - y0);
    let b: i32 = 2 * (x0 - x1);
    let mut d: i32 = a / 2 + b;

    // Looping through range
    while (y <= y1) {
        // Changing pixel
        img.update_pixel_xy(x, y, pix);

        // Updating y value if necessary
        if (d < 0) {
            x += 1;
            d += a;
        }

        // Necessary updates
        y += 1;
        d += b;
    }
}

/// Drawing a line in octant VII
fn draw_oct7(img: &mut Image, pix: Pixel, x0: i32, y0: i32, x1: i32, y1: i32) {
    // Variable declarations
    let (mut x, mut y): (i32, i32) = (x0, y0);
    let a: i32 = 2 * (y0 - y1);
    let b: i32 = 2 * (x0 - x1);
    let mut d: i32 = a / 2 + b;

    // Looping through range
    while (y >= y1) {
        // Changing pixel
        img.update_pixel_xy(x, y, pix);

        // Updating y value if necessary
        if (d < 0) {
            x += 1;
            d += a;
        }

        // Necessary updates
        if (y > 0) {
            y -= 1;
        } else {
            break;
        }
        d += b;
    }
}

/// Drawing a line in octant VIII
fn draw_oct8(img: &mut Image, pix: Pixel, x0: i32, y0: i32, x1: i32, y1: i32) {
    // Variable declarations
    let (mut x, mut y): (i32, i32) = (x0, y0);
    let a: i32 = 2 * (y0 - y1);
    let b: i32 = 2 * (x0 - x1);
    let mut d: i32 = a + b / 2;

    // Looping through range
    while (x <= x1) {
        // Changing pixel
        img.update_pixel_xy(x, y, pix);

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
        x += 1;
        d += a;
    }
}

/// Function that draws an arbitrary line by using the 4 octants above
pub fn draw_line(x0: i32, y0: i32, x1: i32, y1: i32, img: &mut Image, pix: Pixel) {
    // Quadrant 1, 2, 7, 8 cases
    if (x1 >= x0) {
        // Quadrant 1, 2
        if (y1 >= y0) {
            // Quadrant 2
            if (y1 - y0 > x1 - x0) {
                draw_oct2(img, pix, x0, y0, x1, y1);
            }
            // Quadrant 1
            else {
                draw_oct1(img, pix, x0, y0, x1, y1);
            }
        } else {
            // Quadrant 7, 8
            // Quadrant 7
            if (y0 - y1 > x1 - x0) {
                draw_oct7(img, pix, x0, y0, x1, y1);
            }
            // Quadrant 8
            else {
                draw_oct8(img, pix, x0, y0, x1, y1);
            }
        }
    } else {
        // Quadrant 3, 4, 5, 6 cases
        // Quadrant 3, 4
        if (y1 >= y0) {
            // Quadrant 3
            if (y1 - y0 > x0 - x1) {
                draw_oct7(img, pix, x1, y1, x0, y0);
            }
            // Quadrant 4
            else {
                draw_oct8(img, pix, x1, y1, x0, y0);
            }
        } else {
            // Quadrant 5, 6
            // Quadrant 6
            if (y0 - y1 > x0 - x1) {
                draw_oct2(img, pix, x1, y1, x0, y0);
            }
            // Quadrant 5
            else {
                draw_oct1(img, pix, x1, y1, x0, y0);
            }
        }
    }
}

/// Function that rotates a given line by a certain angle in degrees
pub fn rotate_degree(s: (i32, i32), e: (i32, i32), angle: f32) -> (i32, i32, i32, i32) {
    rotate_radian(s, e, angle * PI / 180.0)
}

/// Function that rotates a given line by a certain angle in radians
pub fn rotate_radian(s: (i32, i32), e: (i32, i32), angle: f32) -> (i32, i32, i32, i32) {
    // Getting trig stuff
    let cos: f32 = f32::cos(angle);
    let sin: f32 = f32::sin(angle);

    // Rotating points
    let nx: i32 = (((e.0 - s.0) as f32) * cos - ((e.1 - s.1) as f32) * sin + s.0 as f32) as i32;
    let ny: i32 = (((e.0 - s.0) as f32) * sin + ((e.1 - s.1) as f32) * cos + s.1 as f32) as i32;

    // Returning new points
    (s.0, s.1, nx, ny)
}

/// Function that dilates a given line by a certain scale factor
pub fn dilate(s: (i32, i32), e: (i32, i32), scale: f32) -> (i32, i32, i32, i32) {
    // Scaling points
    let nx: i32 = ((s.0 as f32) + ((e.0 - s.0) as f32 * scale)) as i32;
    let ny: i32 = ((s.1 as f32) + ((e.1 - s.1) as f32 * scale)) as i32;

    // Returning new points
    (s.0, s.1, nx, ny)
}
