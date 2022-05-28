use crate::script::compile;

/// Function that creates all images from work 09
pub fn create_work10_images(mode: i32) {
    // Creating face image test
    compile::compile("data/w10/w10_face.mdl", 500, mode);

    // // Creating robot image test
    // compile::compile("data/w10/w10_test.mdl", 500, mode);
}
