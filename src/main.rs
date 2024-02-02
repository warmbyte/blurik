use std::{env::args, fs, io::Read, path::PathBuf};

use blurik::parser::lexer;

fn main() {
    let args: Vec<String> = args().collect();
    if let Some(file_path) = args.get(1) {
        let file_path = PathBuf::from(&file_path);
        let mut source_code = String::new();
        let mut file = fs::File::open(file_path).unwrap();
        file.read_to_string(&mut source_code).unwrap();

        let tokens = lexer::Lexer::parse(&source_code);
        println!("{:?}", tokens);
    }
}
