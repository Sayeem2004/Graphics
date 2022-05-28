// Imports
use crate::format::util;
use crate::template::work10;

/// Testing out work10 stuff
#[test]
fn test_work10() {
    // Running function
    work10::create_work10_images(1);

    // Making sure image files exist
    assert!(util::file_exists("image/script/w10_face.png"));
    assert!(util::file_exists("image/script/w10_test.png"));
}
