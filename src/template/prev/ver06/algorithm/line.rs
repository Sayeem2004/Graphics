// Imports
use crate::template::prev::ver06::format::{image::Image, pixel::Pixel};

/// Function that draws an arbitrary line by using the 4 octants above
pub fn draw_line(s: (i32, i32), e: (i32, i32), img: &mut Image, pix: Pixel) {
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
fn draw_oct1(img: &mut Image, pix: Pixel, s: (i32, i32), e: (i32, i32)) {
    // Variable declarations
    let (mut x, mut y): (i32, i32) = (s.0, s.1);
    let a: i32 = 2 * (e.1 - s.1);
    let b: i32 = 2 * (s.0 - e.0);
    let mut d: i32 = a + b / 2;

    // Looping through range
    while (x <= e.0) {
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
fn draw_oct2(img: &mut Image, pix: Pixel, s: (i32, i32), e: (i32, i32)) {
    // Variable declarations
    let (mut x, mut y): (i32, i32) = (s.0, s.1);
    let a: i32 = 2 * (e.1 - s.1);
    let b: i32 = 2 * (s.0 - e.0);
    let mut d: i32 = a / 2 + b;

    // Looping through range
    while (y <= e.1) {
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
fn draw_oct7(img: &mut Image, pix: Pixel, s: (i32, i32), e: (i32, i32)) {
    // Variable declarations
    let (mut x, mut y): (i32, i32) = (s.0, s.1);
    let a: i32 = 2 * (s.1 - e.1);
    let b: i32 = 2 * (s.0 - e.0);
    let mut d: i32 = a / 2 + b;

    // Looping through range
    while (y >= e.1) {
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
fn draw_oct8(img: &mut Image, pix: Pixel, s: (i32, i32), e: (i32, i32)) {
    // Variable declarations
    let (mut x, mut y): (i32, i32) = (s.0, s.1);
    let a: i32 = 2 * (s.1 - e.1);
    let b: i32 = 2 * (s.0 - e.0);
    let mut d: i32 = a + b / 2;

    // Looping through range
    while (x <= e.0) {
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
