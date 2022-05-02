// Imports
use crate::format::{constant, pixel::Pixel};

/// Function that returns the dot product of two vectors
pub fn dot(v1: (f32, f32, f32), v2: (f32, f32, f32)) -> f32 {
    // Returning sum
    (v1.0 * v2.0) + (v1.1 * v2.1) + (v1.2 * v2.2)
}

/// Function that returns the lighting of a certain polygon
pub fn calculate(
    amb: Pixel,
    pnt: (Pixel, f32, f32, f32),
    surf: (f32, f32, f32),
    view: (f32, f32, f32),
    normal: (f32, f32, f32),
    div: (f32, f32, f32),
) -> Pixel {
    // Variable declarations
    let (mut fr, mut fg, mut fb): (u8, u8, u8) = (0, 0, 0);
    let (ka, kd, ks): (f32, f32, f32) = scale(div, 1.0 / (div.0 + div.1 + div.2));
    let hnorm: (f32, f32, f32) = normalize(normal);
    let hview: (f32, f32, f32) = normalize(view);
    let cpnt: Pixel = pnt.0;
    let hpnt: (f32, f32, f32) = normalize(diff((pnt.1, pnt.2, pnt.3), surf));

    // Ambient lighting
    fr += (amb.0 as f32 * ka) as u8;
    fg += (amb.1 as f32 * ka) as u8;
    fb += (amb.2 as f32 * ka) as u8;

    // Diffuse lighting
    fr += (cpnt.0 as f32 * kd * dot(hnorm, hpnt)) as u8;
    fg += (cpnt.1 as f32 * kd * dot(hnorm, hpnt)) as u8;
    fb += (cpnt.2 as f32 * kd * dot(hnorm, hpnt)) as u8;

    // Specular lighting
    let rhat: (f32, f32, f32) = diff(scale(hnorm, 2_f32 * dot(hnorm, hpnt)), hpnt);
    let cos: f32 = dot(rhat, hview).max(0.0);
    fr += (cpnt.0 as f32 * ks * cos.powf(constant::EXP)) as u8;
    fg += (cpnt.1 as f32 * ks * cos.powf(constant::EXP)) as u8;
    fb += (cpnt.2 as f32 * ks * cos.powf(constant::EXP)) as u8;

    // Returning pixel values
    Pixel::new_value(fr, fg, fb)
}

/// Function that normalizes a vector
pub fn normalize(vec: (f32, f32, f32)) -> (f32, f32, f32) {
    // Calculating magnitude
    let temp: f32 = vec.0 * vec.0 + vec.1 * vec.1 + vec.2 * vec.2;
    let norm: f32 = temp.sqrt();

    // Exiting function
    (vec.0 / norm, vec.1 / norm, vec.2 / norm)
}

/// Function that returns the difference vector between two vectors
pub fn diff(vec1: (f32, f32, f32), vec2: (f32, f32, f32)) -> (f32, f32, f32) {
    (vec1.0 - vec2.0, vec1.1 - vec2.1, vec1.2 - vec2.2)
}

/// Function that scales a vector by a value
pub fn scale(vec: (f32, f32, f32), scale: f32) -> (f32, f32, f32) {
    (vec.0 * scale, vec.1 * scale, vec.2 * scale)
}
