// Imports
use crate::format::image::Image;
use crate::format::pixel::Pixel;

// Drawing a line in octant I
pub fn draw_oct1(img : &mut Image, pix : Pixel, x0 : u32, y0 : u32, x1 : u32, y1 : u32) {
    // Variable declarations
    let (mut x, mut y) : (u32, u32) = (x0, y0);
    let a : u32 = 2 * (y1 - y0);
    let b : u32 = 2 * (x0 - x1);
    let mut d : i32 = (a + b/2) as i32;

    // Looping through range
    while (x <= x1) {
        // Changing pixel
        img.update_pixel_xy2(x, y, pix);

        // Updating y value if necessary
        if (d < 0) {
            y += 1;
            d += (b as i32);
        }

        // Necessary updates
        x += 1;
        d += (a as i32);
    }
}

// Drawing a line in octant II
pub fn draw_oct2(img : &mut Image, pix : Pixel, x0 : u32, y0 : u32, x1 : u32, y1 : u32) {
    // Variable declarations
    let (mut x, mut y) : (u32, u32) = (x0, y0);
    let a : u32 = 2 * (y1 - y0);
    let b : u32 = 2 * (x0 - x1);
    let mut d : i32 = (a/2 + b) as i32;

    // Looping through range
    while (y <= y1) {
        // Changing pixel
        img.update_pixel_xy2(x, y, pix);

        // Updating y value if necessary
        if (d < 0) {
            x += 1;
            d += (a as i32);
        }

        // Necessary updates
        y += 1;
        d += (b as i32);
    }
}

// Drawing a line in octant VII
pub fn draw_oct7(img : &mut Image, pix : Pixel, x0 : u32, y0 : u32, x1 : u32, y1 : u32) {
    // Variable declarations
    let (mut x, mut y) : (u32, u32) = (x0, y0);
    let a : u32 = 2 * (y0 - y1);
    let b : u32 = 2 * (x0 - x1);
    let mut d : i32 = (a/2 + b) as i32;

    // Looping through range
    while (y >= y1) {
        // Changing pixel
        img.update_pixel_xy2(x, y, pix);

        // Updating y value if necessary
        if (d < 0) {
            x += 1;
            d += (a as i32);
        }

        // Necessary updates
        if (y > 0) {y -= 1;}
        else {break;}
        d += (b as i32);
    }
}

// Drawing a line in octant VIII
pub fn draw_oct8(img : &mut Image, pix : Pixel, x0 : u32, y0 : u32, x1 : u32, y1 : u32) {
    // Variable declarations
    let (mut x, mut y) : (u32, u32) = (x0, y0);
    let a : u32 = 2 * (y0 - y1);
    let b : u32 = 2 * (x0 - x1);
    let mut d : i32 = (a + b/2) as i32;

    // Looping through range
    while (x <= x1) {
        // Changing pixel
        img.update_pixel_xy2(x, y, pix);

        // Updating y value if necessary
        if (d < 0) {
            if (y > 0) {y -= 1;}
            else {break;}
            d += (b as i32);
        }

        // Necessary updates
        x += 1;
        d += (a as i32);
    }
}

// Function that draws an arbitrary line by using the 4 octants above
pub fn draw_line(x0 : u32, y0 : u32, x1 : u32, y1 : u32, img : &mut Image, pix : Pixel) {
    // Quadrant 1, 2, 7, 8 cases
    if (x1 >= x0) {
        // Quadrant 1, 2
        if (y1 >= y0) {
            // Quadrant 2
            if (y1-y0 > x1-x0) {draw_oct2(img, pix, x0, y0, x1, y1);}

            // Quadrant 1
            else {draw_oct1(img, pix, x0, y0, x1, y1);}
        } else { // Quadrant 7, 8
            // Quadrant 7
            if (y0-y1 > x1-x0) {draw_oct7(img, pix, x0, y0, x1, y1);}

            // Quadrant 8
            else {draw_oct8(img, pix, x0, y0, x1, y1);}
        }
    } else { // Quadrant 3, 4, 5, 6 cases
        // Quadrant 3, 4
        if (y1 >= y0) {
            // Quadrant 3
            if (y1-y0 > x0-x1) {draw_oct7(img, pix, x1, y1, x0, y0);}

            // Quadrant 4
            else {draw_oct8(img, pix, x1, y1, x0, y0);}
        } else { // Quadrant 5, 6
            // Quadrant 6
            if (y0-y1 > x0-x1) {draw_oct2(img, pix, x1, y1, x0, y0);}

            // Quadrant 5
            else {draw_oct1(img, pix, x1, y1, x0, y0);}
        }
    }
}
