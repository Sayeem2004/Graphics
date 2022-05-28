// Imports
use crate::format::util;
use crate::template::work00;

/// Testing out work00 stuff
#[test]
fn test_work00() {
    // Running function
    work00::create_work00_images(1);

    // Making sure image files exist
    assert!(util::file_exists("image/ppm/w00_corridor.ppm"));
    assert!(util::file_exists("image/ppm/w00_checkerboard.ppm"));
    assert!(util::file_exists("image/ppm/w00_chains.ppm"));
    assert!(util::file_exists("image/ppm/w00_barnsley.ppm"));
}
