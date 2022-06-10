// Imports
use crate::format::util;
use crate::template::work12;

/// Testing out work12 stuff
#[test]
fn test_work12() {
    // Running function
    work12::create_work12_images(1);

    // Making sure image files exist
    assert!(util::file_exists("image/script/w12_savecs_test.png"));
}
