extern crate core;

use std::fmt::Debug;
use std::io;
use std::io::{BufRead, Write};
use std::path::Path;
use std::process;
use std::process::exit;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{env, fs};

use scanner::Scanner;

use crate::parser::Parser;

mod ast;
mod ast_printer;
mod environment;
mod interpreter;
mod parser;
mod scanner;

// Global flag to indicate if an error has occurred
static HAD_ERROR: AtomicBool = AtomicBool::new(false);

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

    if HAD_ERROR.load(Relaxed) {
        exit(65)
    }
}

fn run_prompt() -> io::Result<()> {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        print!("> ");
        io::stdout().flush()?; // Flush to ensure the prompt is displayed before waiting for input

        match line {
            Ok(input) => {
                run(input);
                HAD_ERROR.store(false, Relaxed);
            }
            Err(_) => break,
        }
    }

    Ok(())
}

fn run(input: String) {
    let mut scanner = Scanner::new(input);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens.clone());
    let expression = parser.parse();
    dbg!(&expression);

    for exp in expression {
        let value = exp.evaluate();
        dbg!(value);
    }
}

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, where_: &str, message: &str) {
    eprintln!("[line {}] Error{}: {}", line, where_, message);
    HAD_ERROR.store(true, Ordering::Relaxed);
}
