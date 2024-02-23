use crate::scanner::token::Token;
use crate::scanner::token::TokenKind;
use crate::scanner::token::TokenKind::*;

pub mod token;

pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line_number: usize,
}

// Implementation for Scanner
impl Scanner {
    // Initialize a new Scanner
    pub fn new(source: &str) -> Self {
        let source = source.chars().collect::<Vec<char>>();
        Scanner {
            source,
            start: 0,
            current: 0,
            line_number: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(EOF);
        }

        let c = self.advance();
        if c.is_alphabetic() || c == '_' {
            return self.identifier();
        }
        if c.is_digit(10) {
            return self.number();
        }

        match c {
            '(' => self.make_token(LeftParen),
            ')' => self.make_token(RightParen),
            '{' => self.make_token(LeftBrace),
            '}' => self.make_token(RightBrace),
            ';' => self.make_token(Semicolon),
            ',' => self.make_token(Comma),
            '.' => self.make_token(Dot),
            '-' => self.make_token(Minus),
            '+' => self.make_token(Plus),
            '*' => self.make_token(Star),
            '/' => self.make_token(Slash),
            '!' => {
                if self.matches('=') {
                    self.make_token(BangEqual)
                } else {
                    self.make_token(Bang)
                }
            }
            '=' => {
                if self.matches('=') {
                    self.make_token(EqualEqual)
                } else {
                    self.make_token(Equal)
                }
            }
            '>' => {
                if self.matches('=') {
                    self.make_token(GreaterEqual)
                } else {
                    self.make_token(Greater)
                }
            }
            '<' => {
                if self.matches('=') {
                    self.make_token(LessEqual)
                } else {
                    self.make_token(Less)
                }
            }
            '"' => self.string(),
            _ => self.error_token("Unexpected character."),
        }
    }

    // Check if the scanner has reached the end of the source
    fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }

    // Consume the current character and return it
    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    // Get the current character without consuming it
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            false
        } else if self.peek() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    // Create a token with the given TokenType
    fn make_token(&self, kind: TokenKind) -> Token {
        Token {
            kind,
            lexeme: self.source[self.start..self.current].iter().collect(),
            line_number: self.line_number,
        }
    }

    // Create an error token with the given message
    fn error_token(&self, message: &str) -> Token {
        Token {
            kind: Error,
            lexeme: message.to_owned(),
            line_number: self.line_number,
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            if c.is_whitespace() {
                self.advance();
                continue;
            }
            if c == '\n' {
                self.line_number += 1;
                self.advance();
                continue;
            }
            if c == '/' {
                if self.peek_next() == '/' {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    return;
                }
                continue;
            }
            return;
        }
    }

    fn check_keyword(&self, start: usize, length: usize, rest: &str, kind: TokenKind) -> TokenKind {
        let compare: std::string::String = self.source[self.start + start..self.current]
            .iter()
            .collect();

        if compare.as_str() == rest && self.current - self.start == start + length {
            return kind;
        }

        Identifier
    }

    fn identifier_type(&self) -> TokenKind {
        match self.source.get(self.start).unwrap() {
            'a' => self.check_keyword(1, 2, "nd", And),
            'e' => self.check_keyword(1, 3, "lse", Else),
            'f' => {
                if self.current - self.start > 1 {
                    match self.source.get(self.start + 1).unwrap() {
                        'a' => self.check_keyword(2, 3, "lse", False),
                        'o' => self.check_keyword(2, 1, "r", For),
                        'n' => self.check_keyword(2, 0, "", Fn),
                        _ => Identifier,
                    }
                } else {
                    Identifier
                }
            }
            'i' => self.check_keyword(1, 1, "f", If),
            'n' => self.check_keyword(1, 3, "one", None),
            'o' => self.check_keyword(1, 1, "r", Or),
            'p' => self.check_keyword(1, 4, "rint", Print),
            'r' => self.check_keyword(1, 5, "eturn", Return),
            's' => {
                if self.current - self.start > 1 {
                    match self.source.get(self.start + 1).unwrap() {
                        'e' => self.check_keyword(2, 2, "lf", Self_),
                        't' => self.check_keyword(2, 4, "ruct", Struct),
                        'u' => self.check_keyword(2, 4, "uper", Super),
                        _ => Identifier,
                    }
                } else {
                    Identifier
                }
            }
            't' => self.check_keyword(1, 3, "rue", True),
            'l' => self.check_keyword(1, 2, "et", Let),
            'w' => self.check_keyword(1, 4, "hile", While),
            _ => Identifier,
        }
    }

    fn identifier(&mut self) -> Token {
        while self.peek().is_alphabetic() || self.peek().is_digit(10) || self.peek() == '_' {
            self.advance();
        }
        self.make_token(self.identifier_type())
    }

    fn number(&mut self) -> Token {
        while self.peek().is_digit(10) {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        self.make_token(Number)
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && self.is_at_end() {
            if self.peek() == '\n' {
                self.line_number += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            return self.error_token("Unterminated string");
        }
        self.advance();
        self.make_token(String)
    }

    // Scan and tokenize the source code
}
