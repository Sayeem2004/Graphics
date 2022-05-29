// Imports
use crate::format::util;
use crate::template::work11;

/// Testing out work11 stuff
#[test]
fn test_work11() {
    // Running function
    work11::create_work11_images(1);

    // Making sure image files exist
    assert!(util::file_exists("image/gif/w11_simple_50.gif"));
}
