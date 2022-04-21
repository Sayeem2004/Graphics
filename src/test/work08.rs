// Imports
use crate::format::file;
use crate::template::work08;

/// Testing out work08 stuff
#[test]
fn test_work08() {
    // Running function
    work08::create_work08_images(1);

    // Making sure image files exist
    assert!(file::file_exists("image/script/w08_robot.png"));
    assert!(file::file_exists("image/ppm/w08_gradient.ppm"));
    assert!(file::file_exists("image/gif/w08_night_sky.gif"));
    assert!(file::file_exists("image/ppm/w08_color_wheel.ppm"));
}
