// Imports
use crate::format::file;
use crate::template::work07;

/// Testing out work07 stuff
#[test]
fn test_work07() {
    // Running function
    work07::create_work07_images(1);

    // Making sure image files exist
    assert!(file::file_exists("image/script/w07_robot.png"));
}
