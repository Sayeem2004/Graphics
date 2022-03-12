// Imports
use crate::format::file;
use crate::template::work00;

/// Testing out work00 stuff
#[test]
fn test_work00() {
    // Running function
    work00::create_work00_images(1);

    // Making sure image files exist
    assert!(file::file_exists("image/ppm/w00_corridor.ppm"));
    assert!(file::file_exists("image/ppm/w00_checkerboard.ppm"));
    assert!(file::file_exists("image/ppm/w00_chains.ppm"));
    assert!(file::file_exists("image/ppm/w00_barnsley.ppm"));
}
