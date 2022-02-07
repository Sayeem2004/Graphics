// Imports
use crate::structs::image;

#[allow(dead_code)]
// Converts an image into the number spiral gradient
pub fn number_spiral(img : &mut image::Image) {
    for i in 0..img.get_height() {
        for q in 0..img.get_width() {
            let r : u32 = 0;
            let g : u32 = number_spiral_helper(i/5, q/5) % 256;
            let b : u32 = number_spiral_helper(q/5, i/5) % 256;
            img.update_pixel(i, q, r, g, b);
        }
    }
}

#[allow(dead_code)]
// Gradient function based on CSES number spiral problem
pub fn number_spiral_helper(row : u32, col : u32) -> u32 {
    let num : u32;
    if (row > col) {
        if (row % 2 == 0) {num = row * row - col + 1;}
        else {num = (row - 1) * (row - 1) + col;}
    } else if (col > row) {
        if (col % 2 == 1) {num = col * col - row + 1;}
        else {num = (col - 1) * (col - 1) + row;}
    } else {
        if (row % 2 == 0) {num = row * row - col + 1;}
        else {num = col * col - row + 1;}
    }
    return num;
}

#[allow(dead_code)]
// Converts an image into the number grid gradient
pub fn number_grid(img : &mut image::Image) {
    for i in 0..img.get_height() {
        for q in 0..img.get_width() {
            let r : u32 = 0;
            let g : u32 = number_grid_helper(i, q) % 256;
            let b : u32 = number_grid_helper(q/2, i/2) % 256;
            img.update_pixel(i, q, r, g, b);
        }
    }
}

#[allow(dead_code)]
// Gradient function based on CSES number grid problem
pub fn number_grid_helper(row : u32, col : u32) -> u32 {
    return (row-1)^(col-1);
}
