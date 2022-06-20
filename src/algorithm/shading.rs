// Imports
use crate::algorithm::curve;
use crate::format::{image::Image, matrix::Matrix, pixel::Pixel};

/// Function that handles scanline conversion for a triangle using flat shading
pub fn scanline(poly: &Matrix, ind: i32, img: &mut Image, pix: Pixel) {
    // Sorting points by y coordinate
    let mut list: Vec<(i32, i32, f32)> = vec![
        (
            poly.data[ind as usize][0] as i32,
            poly.data[ind as usize][1] as i32,
            poly.data[ind as usize][2],
        ),
        (
            poly.data[(ind - 1) as usize][0] as i32,
            poly.data[(ind - 1) as usize][1] as i32,
            poly.data[(ind - 1) as usize][2],
        ),
        (
            poly.data[(ind - 2) as usize][0] as i32,
            poly.data[(ind - 2) as usize][1] as i32,
            poly.data[(ind - 2) as usize][2],
        ),
    ];
    list.sort_by(|a, b| (a.1).partial_cmp(&(b.1)).unwrap());

    // Checking for degenerate triangles
    if ((list[0].0 == list[1].0 && list[1].0 == list[2].0)
        || (list[0].1 == list[1].1 && list[1].1 == list[2].1))
    {
        curve::draw_line(
            (list[0].0, list[0].1, list[0].2),
            (list[1].0, list[1].1, list[1].2),
            img,
            pix,
        );
        curve::draw_line(
            (list[2].0, list[2].1, list[2].2),
            (list[1].0, list[1].1, list[1].2),
            img,
            pix,
        );
        return;
    }

    // Casework on ordering of points
    let bot: (i32, i32, f32);
    let mid: (i32, i32, f32);
    let top: (i32, i32, f32);
    if (list[0].1 == list[1].1) {
        // Bottom y == middle y case
        if ((list[0].0 - list[2].0).abs() >= list[1].0 - list[2].0.abs()) {
            (bot, mid, top) = (list[0], list[1], list[2]);
        } else {
            (mid, bot, top) = (list[0], list[1], list[2]);
        }

        // Initializing coordinates
        let (mut y, mxy): (i32, i32) = (bot.1, top.1);
        let (mut x1, mut x2): (f32, f32) = (bot.0 as f32, mid.0 as f32);
        let (mut z1, mut z2): (f32, f32) = (bot.2, mid.2);
        let (dx1, dx2): (f32, f32) = (
            (top.0 - bot.0) as f32 / (top.1 - bot.1) as f32,
            (top.0 - mid.0) as f32 / (top.1 - mid.1) as f32,
        );
        let (dz1, dz2): (f32, f32) = (
            (top.2 - bot.2) as f32 / (top.1 - bot.1) as f32,
            (top.2 - mid.2) as f32 / (top.1 - mid.1) as f32,
        );

        // Iterating through coordinates
        while (y <= mxy) {
            // Drawing scanline
            curve::draw_line((x1 as i32, y, z1), (x2 as i32, y, z2), img, pix);

            // Updating coordinates
            x1 += dx1;
            x2 += dx2;
            z1 += dz1;
            z2 += dz2;
            y += 1;
        }
    } else if (list[1].1 == list[2].1) {
        // Middle y == top y case
        if ((list[2].0 - list[0].0).abs() >= list[1].0 - list[0].0.abs()) {
            (bot, mid, top) = (list[0], list[1], list[2]);
        } else {
            (bot, top, mid) = (list[0], list[1], list[2]);
        }

        // Initializing coordinates
        let (mut y, mxy): (i32, i32) = (bot.1, top.1);
        let (mut x1, mut x2): (f32, f32) = (bot.0 as f32, bot.0 as f32);
        let (mut z1, mut z2): (f32, f32) = (bot.2, bot.2);
        let (dx1, dx2): (f32, f32) = (
            (top.0 - bot.0) as f32 / (top.1 - bot.1) as f32,
            (mid.0 - bot.0) as f32 / (mid.1 - bot.1) as f32,
        );
        let (dz1, dz2): (f32, f32) = (
            (top.2 - bot.2) as f32 / (top.1 - bot.1) as f32,
            (mid.2 - bot.2) as f32 / (mid.1 - bot.1) as f32,
        );

        // Iterating through coordinates
        while (y <= mxy) {
            // Drawing scanline
            curve::draw_line((x1 as i32, y, z1), (x2 as i32, y, z2), img, pix);

            // Updating coordinates
            x1 += dx1;
            x2 += dx2;
            z1 += dz1;
            z2 += dz2;
            y += 1;
        }
    } else {
        // No equivalent y levels case
        (bot, mid, top) = (list[0], list[1], list[2]);

        // Initializing coordinates
        let (mut y, miy, mxy): (i32, i32, i32) = (bot.1, mid.1, top.1);
        let (mut x1, mut x2): (f32, f32) = (bot.0 as f32, bot.0 as f32);
        let (mut z1, mut z2): (f32, f32) = (bot.2 as f32, bot.2 as f32);
        let (dx1, dx2, dx3): (f32, f32, f32) = (
            (top.0 - bot.0) as f32 / (top.1 - bot.1) as f32,
            (mid.0 - bot.0) as f32 / (mid.1 - bot.1) as f32,
            (top.0 - mid.0) as f32 / (top.1 - mid.1) as f32,
        );
        let (dz1, dz2, dz3): (f32, f32, f32) = (
            (top.2 - bot.2) as f32 / (top.1 - bot.1) as f32,
            (mid.2 - bot.2) as f32 / (mid.1 - bot.1) as f32,
            (top.2 - mid.2) as f32 / (top.1 - mid.1) as f32,
        );

        // Iterating through coordinates
        while (y <= mxy) {
            // Drawing scanline
            curve::draw_line((x1 as i32, y, z1), (x2 as i32, y, z2), img, pix);

            // Updating coordinates
            x1 += dx1;
            x2 += if (y < miy) { dx2 } else { dx3 };
            z1 += dz1;
            z2 += if (y < miy) { dz2 } else { dz3 };
            y += 1;
        }
    }
}
