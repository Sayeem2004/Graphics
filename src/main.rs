// Imports
#![allow(unused_parens)]
#![allow(dead_code)]
mod algorithm;
mod format;
mod prev;
mod script;
use std::io;
mod template;
mod test;
use std::fs;

// Main function
fn main() {
    // Signal handling
    ctrlc::set_handler(move || {
        println!("");
        end_program();
    })
    .expect("Error setting Ctrl-C handler");

    // Running the current assignment
    // template::work00::create_work00_images(0);
    // template::work01::create_work01_images(0);
    // template::work02::create_work02_images(0);
    // template::work03::create_work03_images(0);
    // template::work04::create_work04_images(0);
    // template::work05::create_work05_images(0);
    // template::work06::create_work06_images(0);
    // template::work07::create_work07_images(0);
    // template::work08::create_work08_images(0);
    // template::work09::create_work09_images(0);
    template::work10::create_work10_images(0);

    // Variable Declarations
    let mut path: String = String::new();
    let stdin = io::stdin();

    // Forever loop
    loop {
        // Getting script file path
        println!("\nEnter script file name:");
        stdin.read_line(&mut path).unwrap();
        path = path[0..path.len() - 1].to_string();

        if (path.eq("exit") || path.eq("quit")) {
            // Exiting forever loop
            break;
        } else {
            // Parsing a script file and making an image
            script::compile::compile(&path, 750, 0);
        }

        // Resetting variable
        path.clear();
    }

    // Exiting program
    end_program();
}

fn end_program() {
    // Cleaning some things up
    if (format::util::file_exists("temp")) {
        fs::remove_dir_all("temp").expect("Unable to remove temp directory");
    }

    // Ending message
    println!("\nThanks for using this graphics engine made by Mohammad Khan\n");

    // Terminating Program
    std::process::exit(0);
}
