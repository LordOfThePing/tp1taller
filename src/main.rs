use std::env;
use tp1taller::{read_file::read_file, translate::translate};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Error: invalid arg count");
    } else {
        let output: String = read_file(&args[1]);
        let out = output.as_bytes();
        let mut vec = translate(out);
        let mut str = String::from_utf8( translate(out) ).expect("could not obtain String");

        for _ in 0..str.len() {

            
            print!("{:?}", str.pop().expect("msg"));
        }
    }
}
