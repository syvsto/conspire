use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

pub fn write_to_file<'a>(path: &Path, contents: &'a str) {
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    match file.write_all(contents.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("Successfully wrote to {}", display),
    }
}
