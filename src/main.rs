use std::env;
use text_colorizer::*;

#[derive(Debug)]
struct Argument {
    filename: String
}

fn main() {
    let filename = get_filename();
    println!("supplied filename: {:?}", filename)
}


fn get_filename() -> Argument {
    let arg: Vec<String> = env::args().skip(1).collect();

    if arg.len() != 1 {
        eprintln!("{} Wrong number of arguments, expected 1 got {}", "Error:".red().bold(), arg.len());
        std::process::exit(1);
    }

    Argument { filename: arg[0].clone() }
}
