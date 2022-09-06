use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_file(args: &String) -> String {
    let path = Path::new(args);
    let _display = path.display();
    let mut file = File::open(&path).expect(&format!("Error: could not open {:?}", args));
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error: Reading string");
    contents
}
