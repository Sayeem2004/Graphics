use crate::script::compile;

/// Function that creates all images from work 10
pub fn create_work10_images(mode: i32) {
    // Creating face image test
    compile::compile("data/w10/w10_face.mdl", 500, mode);

    // Creating robot image test
    if (mode != 0) {
        compile::compile("data/w10/w10_test.mdl", 500, mode);
    }

    // Creating excalibur image
    compile::compile("data/w10/w10_excalibur.mdl", 750, mode);

    // Creating trophy image
    compile::compile("data/w10/w10_trophy.mdl", 750, mode);

    // Creating spiral image
    compile::compile("data/w10/w10_W.mdl", 750, mode);
}
