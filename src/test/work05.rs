// Imports
use crate::format::file;
use crate::template::work05;

/// Testing out work05 stuff
#[test]
fn test_work05() {
    // Running function
    work05::create_work05_images(1);

    // Making sure image files exist
    assert!(file::file_exists("image/script/w05_face.png"));
    assert!(file::file_exists("image/script/w05_solar.png"));
    assert!(file::file_exists("image/ppm/w05_filled_sphere.ppm"));
    assert!(file::file_exists("image/ppm/w05_rgb_sphere.ppm"));
}
