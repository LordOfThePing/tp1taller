mod read_file;

use read_file::read_file;
use std::env;

fn main() {
    
    

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Error: invalid arg count");
    } else {
        
        let output: String = read_file(&args[1]);
        println!("{:?}", output.as_bytes());

    }

}
