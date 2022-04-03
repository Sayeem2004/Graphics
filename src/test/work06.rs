// Imports
use crate::format::file;
use crate::template::work06;

/// Testing out work05 stuff
#[test]
fn test_work06() {
    // Running function
    work06::create_work06_images(1);

    // Making sure image files exist
    assert!(file::file_exists("image/script/w06_face.png"));
    assert!(file::file_exists("image/gif/w06_purple_hollow.gif"));
    assert!(file::file_exists("image/gif/w06_perspectives.gif"));
    assert!(file::file_exists("image/script/w06_spheres.png"));
}
