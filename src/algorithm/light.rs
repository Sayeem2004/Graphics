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
    pnts: &[(Pixel, f32, f32, f32)],
    surf: (f32, f32, f32),
    view: (f32, f32, f32),
    normal: (f32, f32, f32),
    div: (f32, f32, f32, f32, f32, f32, f32, f32, f32),
) -> Pixel {
    // Variable declarations
    let (mut fr, mut fg, mut fb): (u8, u8, u8) = (0, 0, 0);
    let (kra, kga, kba, krd, kgd, kbd, krs, kgs, kbs): (
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
    ) = scale9(
        div,
        3.0 / (div.0 + div.1 + div.2 + div.3 + div.4 + div.5 + div.6 + div.7 + div.8),
    );
    let hnorm: (f32, f32, f32) = normalize(normal);
    let hview: (f32, f32, f32) = normalize(view);
    let mut cpnt: Vec<Pixel> = vec![Pixel::new(); pnts.len()];
    let mut hpnt: Vec<(f32, f32, f32)> = vec![(0.0, 0.0, 0.0); pnts.len()];
    for i in 0..pnts.len() {
        cpnt[i] = pnts[i].0;
        hpnt[i] = normalize(diff((pnts[i].1, pnts[i].2, pnts[i].3), surf));
    }

    // Ambient lighting
    fr += (amb.0 as f32 * kra) as u8;
    fg += (amb.1 as f32 * kga) as u8;
    fb += (amb.2 as f32 * kba) as u8;

    // Diffuse lighting
    let (mut rs1, mut gs1, mut bs1): (f32, f32, f32) = (0.0, 0.0, 0.0);
    for i in 0..pnts.len() {
        rs1 += (cpnt[i].0 as f32 * dot(hnorm, hpnt[i]));
        gs1 += (cpnt[i].1 as f32 * dot(hnorm, hpnt[i]));
        bs1 += (cpnt[i].2 as f32 * dot(hnorm, hpnt[i]));
    }
    fr += (rs1.min(255.0) * krd) as u8;
    fg += (gs1.min(255.0) * kgd) as u8;
    fb += (bs1.min(255.0) * kbd) as u8;

    // Specular lighting
    let (mut rs2, mut gs2, mut bs2): (f32, f32, f32) = (0.0, 0.0, 0.0);
    for i in 0..pnts.len() {
        let rhat: (f32, f32, f32) = diff(scale3(hnorm, 2_f32 * dot(hnorm, hpnt[i])), hpnt[i]);
        let cos: f32 = dot(rhat, hview).max(0.0);
        rs2 += (cpnt[i].0 as f32 * cos.powf(constant::EXP));
        gs2 += (cpnt[i].1 as f32 * cos.powf(constant::EXP));
        bs2 += (cpnt[i].2 as f32 * cos.powf(constant::EXP));
    }
    fr += (rs2.min(255.0) * krs) as u8;
    fg += (gs2.min(255.0) * kgs) as u8;
    fb += (bs2.min(255.0) * kbs) as u8;

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
pub fn scale9(
    vec: (f32, f32, f32, f32, f32, f32, f32, f32, f32),
    scale: f32,
) -> (f32, f32, f32, f32, f32, f32, f32, f32, f32) {
    (
        vec.0 * scale,
        vec.1 * scale,
        vec.2 * scale,
        vec.3 * scale,
        vec.4 * scale,
        vec.5 * scale,
        vec.6 * scale,
        vec.7 * scale,
        vec.8 * scale,
    )
}

/// Function that scales a vector by a value
pub fn scale3(vec: (f32, f32, f32), scale: f32) -> (f32, f32, f32) {
    (vec.0 * scale, vec.1 * scale, vec.2 * scale)
}
