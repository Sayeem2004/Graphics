use crate::format::file;
use crate::template::cses;

#[test]
fn test_cses() {
    // Running function
    cses::create_cses_images();

    // Making sure image files exist
    assert!(file::file_exists("image/cses/corridor.ppm"));
    assert!(file::file_exists("image/cses/checkerboard.ppm"));
    assert!(file::file_exists("image/cses/chains.ppm"));
}
