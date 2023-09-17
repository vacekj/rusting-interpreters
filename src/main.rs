use std::{env, fmt, fs};
use std::ascii::Char;
use std::process;
use std::io;
use std::io::{BufRead, Write};
use std::path::Path;
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::atomic::Ordering::Relaxed;
use crate::TokenType::{Eof, LeftParen};

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
        io::stdout().flush()?;  // Flush to ensure the prompt is displayed before waiting for input

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
    let mut scanner = Scanner::new(input.to_owned());
    let tokens = scanner.scan_tokens();

    for token in tokens {
        dbg!(token);
    }
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        &self.current >= &self.source.len()
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !&self.is_at_end() {
            self.start = self.current.clone();
            self.scan_token();
        }

        self.tokens.push(Token::new(Eof, String::from(""), String::from(""), 0));

        &self.tokens
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(LeftParen),
            _ => unreachable!()
        }
    }

    fn add_token(&mut self, ty: TokenType) {
        let text = &self.source.as_str()[self.start.clone()..self.current.clone()];
        self.tokens.push(Token::new(ty,
                                    text.to_string(),
                                    text.to_string(),
                                    self.line.clone()));
    }

    fn advance(&mut self) -> char {
        let char_ = self.source.chars().nth(self.current.clone());
        self.current += 1;
        char_.unwrap()
    }
}

#[derive(Debug)]
enum TokenType {
    /*Single-char tokens*/
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    /*One or two-char tokens*/
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    /*Literals*/
    Ident,
    String,
    Number,

    /*Keywords*/
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl fmt::Display for TokenType {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
struct Token {
    ty: TokenType,
    lexeme: String,
    literal: String,
    line: usize,
}

impl Token {
    pub fn new(ty: TokenType, lexeme: String, literal: String, line: usize) -> Token {
        Token {
            literal,
            ty,
            lexeme,
            line,
        }
    }

    pub fn to_string(&self) {
        let ty = &self.ty;
        let lexeme = &self.lexeme;
        let literal = &self.literal;
        format!("{ty} {lexeme} {literal}");
    }
}

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, where_: &str, message: &str) {
    eprintln!("[line {}] Error{}: {}", line, where_, message);
    HAD_ERROR.store(true, Ordering::Relaxed);
}
