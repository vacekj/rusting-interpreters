use std::{env, fs};
use std::process;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        n if n > 2 => {
            println!("Usage: rlox [script]");
            process::exit(64);
        },
        2 => run_file(&args[1])?,
        _ => run_prompt()?,
    }


    Ok(())
}

fn run_file(path: &str) -> io::Result<()> {
    let content = fs::read_to_string(Path::new(path))?;
    run(&content)
}

fn run(input: &str) -> io::Result<()> {
    // TODO: Implement the logic for running the input
    Ok(())
}

fn run_prompt() -> io::Result<()> {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        print!("> ");
        io::stdout().flush()?;  // Flush to ensure the prompt is displayed before waiting for input

        match line {
            Ok(input) => run(&input).unwrap(),
            Err(_) => break,
        }
    }

    Ok(())
}

struct Scanner {
    source: String;
}

impl Scanner {
    pub fn scanTokens
}
