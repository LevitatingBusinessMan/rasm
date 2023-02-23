use std::fs;

mod lexer;
mod backends;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() >= 1 {
        let source = fs::read_to_string(&args[0]).unwrap();
        let tokens = lexer::lex(source, backends::x86::X86);
    } else {
        eprint!("No file supplied");
    }
}
