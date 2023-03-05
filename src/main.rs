use std::{env, fs::File, io::{BufReader, BufRead}};
use text_colorizer::*;
use words_count;
#[derive(Debug)]
struct Argument {
    filename: String
}

fn main() {

    let file = get_filename();

    let file_content = match  File::open(file.filename) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("{} reading file content - {:?}", "Error:".red().bold(), error);
            std::process::exit(1);
        }
    };

    let mut buff_reader = BufReader::new(file_content);

    let words = match counter(&mut buff_reader) {
        Ok(count) => count,
        Err(error) => {
            eprintln!("{} counting words: {:?}", "Error:".red().bold(), error);
            std::process::exit(1);
        }
    };

    eprintln!("{}: {}", "Total words in given file".blue().bold(), words);    
}


fn counter<R: BufRead>(reader: &mut R) -> Result<u32, String> {
    
    let mut total_words :u32 = 0;

    let mut line = String::new();

    loop {
        match reader.read_line(& mut line) {
            Ok(_) => {
                if line.is_empty() {
                        break;
                    }
                let words_line =  words_count::count(&line);
                line.clear();
                let words = words_line.words as u32;
                total_words += words;
            },
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }
    Ok(total_words)
}

fn get_filename() -> Argument {
    let arg: Vec<String> = env::args().skip(1).collect();

    if arg.len() != 1 {
        eprintln!("{} Wrong number of arguments, expected a filename got {}", "Error:".red().bold(), arg.len());
        std::process::exit(1);
    }

    Argument { filename: arg[0].clone() }
}

