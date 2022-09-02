mod read_file;

use read_file::read_file;
use std::env;

fn main() {
    
    

    let args: Vec<String> = env::args().collect();

    let output: String = read_file(args);

}
