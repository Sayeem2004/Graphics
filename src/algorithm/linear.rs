// Imports
use std::f32::consts::PI;

/// Function that translates a given line by a certain distance in the x direction
pub fn translate(x0 : i32, y0 : i32, x1 : i32, y1 : i32, xdist : i32, ydist : i32) -> (i32, i32, i32, i32) {
    // Returning new points
    return (x0+xdist, y0+ydist, x1+xdist, y1+ydist);
}

/// Function that rotates a given line by a certain angle in degrees
pub fn rotate_degree(x0 : i32, y0 : i32, x1 : i32, y1 : i32, angle : f32) -> (i32, i32, i32, i32) {
    // Getting trig stuff
    let cos : f32 = f32::cos(angle/180.0*PI);
    let sin : f32 = f32::sin(angle/180.0*PI);

    // Rotating points
    let nx : i32 = (((x1 - x0) as f32)*cos - ((y1 - y0) as f32)*sin + x0 as f32) as i32;
    let ny : i32 = (((x1 - x0) as f32)*sin + ((y1 - y0) as f32)*cos + y0 as f32) as i32;

    // Returning new points
    return (x0, y0, nx, ny);
}

/// Function that rotates a given line by a certain angle in radians
pub fn rotate_radian(x0 : i32, y0 : i32, x1 : i32, y1 : i32, angle : f32) -> (i32, i32, i32, i32) {
    // Getting trig stuff
    let cos : f32 = f32::cos(angle);
    let sin : f32 = f32::sin(angle);

    // Rotating points
    let nx : i32 = (((x1 - x0) as f32)*cos - ((y1 - y0) as f32)*sin + x0 as f32) as i32;
    let ny : i32 = (((x1 - x0) as f32)*sin + ((y1 - y0) as f32)*cos + y0 as f32) as i32;

    // Returning new points
    return (x0, y0, nx, ny);
}

/// Function that dilates a given line by a certain scale factor
pub fn dilate(x0 : i32, y0 : i32, x1 : i32, y1 : i32, scale : f32) -> (i32, i32, i32, i32) {
    // Scaling points
    let nx : i32 = ((x0 as f32) + ((x1 - x0) as f32 * scale)) as i32;
    let ny : i32 = ((y0 as f32) + ((y1 - y0) as f32 * scale)) as i32;

    // Returning new points
    return (x0, y0, nx, ny);
}
