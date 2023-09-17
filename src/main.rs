use crate::TokenType::{And, BangEqual, Class, Comma, Dot, Else, Eof, EqualEqual, False, For, Fun, GreaterEqual, Ident, If, LeftBrace, LeftParen, LessEqual, Minus, Nil, Number, Or, Plus, Print, Return, RightBrace, RightParen, Semicolon, Slash, Star, Super, This, True, Var, While};
use std::io;
use std::io::{BufRead, Write};
use std::path::Path;
use std::process;
use std::process::exit;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{env, fmt, fs};
use std::collections::HashMap;
use std::fmt::Debug;
use crate::TokenValue::NumberLiteral;

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
    keywords: HashMap<String, TokenType>
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let mut keywords = HashMap::new();

        keywords.insert("and".to_string(),    And);
        keywords.insert("class".to_string(),  Class);
        keywords.insert("else".to_string(),   Else);
        keywords.insert("false".to_string(),  False);
        keywords.insert("for".to_string(),    For);
        keywords.insert("fun".to_string(),    Fun);
        keywords.insert("if".to_string(),     If);
        keywords.insert("nil".to_string(),    Nil);
        keywords.insert("or".to_string(),     Or);
        keywords.insert("print".to_string(),  Print);
        keywords.insert("return".to_string(), Return);
        keywords.insert("super".to_string(),  Super);
        keywords.insert("this".to_string(),   This);
        keywords.insert("true".to_string(),   True);
        keywords.insert("var".to_string(),    Var);
        keywords.insert("while".to_string(),  While);

        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords
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

        self.tokens
            .push(Token::new(Eof, String::from(""), None, 0));

        &self.tokens
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(LeftParen, None),
            ')' => self.add_token(RightParen, None),
            '{' => self.add_token(RightBrace, None),
            '}' => self.add_token(LeftBrace, None),
            ',' => self.add_token(Comma, None),
            '.' => self.add_token(Dot, None),
            '-' => self.add_token(Minus, None),
            '+' => self.add_token(Plus, None),
            ';' => self.add_token(Semicolon, None),
            '*' => self.add_token(Star, None),
            '!' => {
                let ty = if self.metch('=') {
                    BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(ty, None);
            }
            '=' => {
                let ty = if self.metch('=') {
                    EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(ty, None);
            }
            '<' => {
                let ty = if self.metch('=') {
                    LessEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(ty, None);
            }
            '>' => {
                let ty = if self.metch('=') {
                    GreaterEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(ty, None);
            }
            '/' => {
                if self.metch('/') {
                    /* Comment goes until the very end of the line */
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(Slash, None);
                }
            }
            ' ' | '\r' | '\t' => { /*Ignore whitespace*/ }
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.string();
            }
            any => {
                if self.is_digit(any.clone()) {
                    self.number();
                } else if self.is_alpha(any) {
                    self.identifier();
                } else {
                    error(self.line.clone(), "Unexpected char");
                }
            }
        }
    }

    /** Lookahead */
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        return self.source.chars().nth(self.current.clone()).unwrap();
    }

    fn metch(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current.clone()).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn add_token(&mut self, ty: TokenType, value: Option<TokenValue>) {
        let text = &self.source.as_str()[self.start..self.current];
        self.tokens.push(Token::new(
            ty,
            text.to_string(),
            value,
            self.line.clone(),
        ));
    }

    fn advance(&mut self) -> char {
        let char_ = self.source.chars().nth(self.current.clone());
        self.current += 1;
        char_.unwrap()
    }
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            error(self.line.clone(), "Unterminated string");
        }

        /*Closing " */
        self.advance();

        let value = &self.source.as_str()[self.start.clone() + 1..self.current.clone() - 1];
        self.add_token(TokenType::String, Some(TokenValue::StringLiteral(value.to_string())));
    }

    fn is_digit(&self, c: char) -> bool {
        c.clone() >= '0' && c <= '9'
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let value = &self.source.as_str()[self.start.clone()..self.current.clone()];

        self.add_token(Number, Some(NumberLiteral(value.parse::<f64>().unwrap())));
    }

    fn peek_next(&self) -> char {
        if self.current.clone() + 1 >= self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.current.clone() + 1).unwrap()
    }
    fn is_alpha(&self, c: char) -> bool {
        (&c >= &'a' && &c <= &'z') ||
            (&c >= &'A' && &c <= &'Z') ||
            &c == &'_'
    }

    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source.as_str()[self.start.clone()..self.current.clone()];
        let ty = self.keywords.get(text).unwrap_or(&Ident);

        self.add_token(*ty, None);
    }
    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_alpha(c.clone()) || self.is_digit(c)
    }
}

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, where_: &str, message: &str) {
    eprintln!("[line {}] Error{}: {}", line, where_, message);
    HAD_ERROR.store(true, Ordering::Relaxed);
}

#[derive(Debug)]
enum TokenValue {
    StringLiteral(String),
    NumberLiteral(f64),
}

#[derive(Debug, Copy, Clone)]
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
struct Token {
    ty: TokenType,
    lexeme: String,
    literal: Option<TokenValue>,
    line: usize,
}

impl Token {
    pub fn new(ty: TokenType, lexeme: String, literal: Option<TokenValue>, line: usize) -> Token {
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
        format!("{ty} {lexeme}");
    }
}
