// Imports
use crate::format::util;
use crate::template::work07;

/// Testing out work07 stuff
#[test]
fn test_work07() {
    // Running function
    work07::create_work07_images(1);

    // Making sure image files exist
    assert!(util::file_exists("image/script/w07_robot.png"));
    assert!(util::file_exists("image/ppm/w07_spiralchain.ppm"));
    assert!(util::file_exists("image/ppm/w07_olympics.ppm"));
    assert!(util::file_exists("image/script/w07_umbrella.png"));
}
