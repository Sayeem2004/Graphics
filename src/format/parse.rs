// Imports
use crate::format::file;

/// Function that parses a graphics script file and makes an image
pub fn parse(path : &str) {
    // Making sure the script file exists
    if (!file::file_exists(path)) {
        eprintln!("Script file does not exist, ending parsing attempt");
        return;
    }

    // Getting all lines from the script file
    let lines : Vec<String> = file::read_lines(path);
    println!("{}", lines[0]);
}
