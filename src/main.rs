use std::env;
use tp1taller::{read_file::read_file, translate::translate};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Error: invalid arg count");
    } else {
        let output: String = read_file(&args[1]);
        let out = output.as_bytes();
        let vec = translate(out);
        // let mut str = String::from_utf8( translate(out) ).expect("could not obtain String");
        print_boards(out, &vec);
    }
}


fn print_boards(out: &[u8], vec: &Vec<u8> ) {
    println!("INPUT");
    for item in out {
        print!(" {:}", (*item) as char);
    }
    print!("{}{}", 10u8 as char, 10u8 as char);
    println!("OUTPUT");
    for item in vec {
        print!(" {:}", (*item) as char);
    }
}