// Imports
use crate::format::util;
use crate::template::work06;

/// Testing out work06 stuff
#[test]
fn test_work06() {
    // Running function
    work06::create_work06_images(1);

    // Making sure image files exist
    assert!(util::file_exists("image/script/w06_face.png"));
    assert!(util::file_exists("image/gif/w06_purple_hollow.gif"));
    assert!(util::file_exists("image/gif/w06_perspectives.gif"));
    assert!(util::file_exists("image/script/w06_spheres.png"));
}
