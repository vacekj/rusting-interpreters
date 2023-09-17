use std::{env, fs};
use std::process;
use std::io;
use std::io::{BufRead, Write};
use std::path::Path;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        n if n > 2 => {
            println!("Usage: rlox [script]");
            process::exit(64);
        }
        2 => run_file(&args[1]),
        _ => run_prompt()?,
    }


    Ok(())
}

fn run_file(path: &str) {
    let content = fs::read_to_string(Path::new(path)).unwrap();
    run(content);
}

fn run_prompt() -> io::Result<()> {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        print!("> ");
        io::stdout().flush()?;  // Flush to ensure the prompt is displayed before waiting for input

        match line {
            Ok(input) => run(input),
            Err(_) => break,
        }
    }

    Ok(())
}

fn run(input: String) {
    let scanner = Scanner::new(input.to_owned());
    let tokens = scanner.scan_tokens();

    for token in tokens {
        dbg!(token);
    }
}

struct Scanner {
    source: String,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source
        }
    }

    pub fn scan_tokens(self) -> Vec<Token> {
        vec![]
    }
}

#[derive(Debug)]
enum Token {

}
