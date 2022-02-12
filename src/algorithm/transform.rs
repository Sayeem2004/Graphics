// Imports
use std::f64::consts::PI;

// Function that translates a given line by a certain distance
pub fn translate(x0 : u32, y0 : u32, x1 : u32, y1 : u32, dist : i32) -> (u32, u32, u32, u32) {
    // Translating variables
    let nx0 : u32 = (x0 as i32 + dist) as u32;
    let ny0 : u32 = (y0 as i32 + dist) as u32;
    let nx1 : u32 = (x1 as i32 + dist) as u32;
    let ny1 : u32 = (y1 as i32 + dist) as u32;

    // Returning new points
    return (nx0, ny0, nx1, ny1);
}

// Function that rotates a given line by a certain angle in degrees
pub fn rotate_degree(x0 : u32, y0 : u32, x1 : u32, y1 : u32, angle : f64) -> (u32, u32, u32, u32) {
    // Getting trig stuff
    let cos : f64 = f64::cos(angle/180.0*PI);
    let sin : f64 = f64::sin(angle/180.0*PI);

    // Rotating points
    let nx : u32 = (((x1 as i32 - x0 as i32) as f64)*cos - ((y1 as i32 - y0 as i32) as f64)*sin + x0 as f64) as u32;
    let ny : u32 = (((x1 as i32 - x0 as i32) as f64)*sin + ((y1 as i32 - y0 as i32) as f64)*cos + y0 as f64) as u32;

    // Returning new points
    return (x0, y0, nx, ny);
}

// Function that rotates a given line by a certain angle in radians
pub fn rotate_radian(x0 : u32, y0 : u32, x1 : u32, y1 : u32, angle : f64) -> (u32, u32, u32, u32) {
    // Getting trig stuff
    let cos : f64 = f64::cos(angle);
    let sin : f64 = f64::sin(angle);

    // Rotating points
    let nx : u32 = (((x1 as i32 - x0 as i32) as f64)*cos - ((y1 as i32 - y0 as i32) as f64)*sin + x0 as f64) as u32;
    let ny : u32 = (((x1 as i32 - x0 as i32) as f64)*sin + ((y1 as i32 - y0 as i32) as f64)*cos + y0 as f64) as u32;

    // Returning new points
    return (x0, y0, nx, ny);
}

// Function that dilates a given line by a certain scale factor
pub fn dilate(x0 : u32, y0 : u32, x1 : u32, y1 : u32, scale : f64) -> (u32, u32, u32, u32) {
    // Scaling points
    let nx : u32 = ((x0 as f64)+((x1 as i32 - x0 as i32) as f64 * scale)) as u32;
    let ny : u32 = ((y0 as f64)+((y1 as i32 - y0 as i32) as f64 * scale)) as u32;

    // Returning new points
    return (x0, y0, nx, ny);
}
