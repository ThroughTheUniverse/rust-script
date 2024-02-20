use crate::scanner::token::Token;
use crate::scanner::token::TokenKind;

pub mod token;

pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
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
            line: 1,
        }
    }
    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenKind::EOF);
        }

        let c = self.advance();
        if c.is_alphabetic() {
            return self.identifier();
        }
        if c.is_digit(10) {
            return self.number();
        }

        match c {
            '(' => self.make_token(TokenKind::LeftParen),
            ')' => self.make_token(TokenKind::RightParen),
            '{' => self.make_token(TokenKind::LeftBrace),
            '}' => self.make_token(TokenKind::RightBrace),
            ';' => self.make_token(TokenKind::Semicolon),
            ',' => self.make_token(TokenKind::Comma),
            '.' => self.make_token(TokenKind::Dot),
            '-' => self.make_token(TokenKind::Minus),
            '+' => self.make_token(TokenKind::Plus),
            '*' => self.make_token(TokenKind::Star),
            '/' => self.make_token(TokenKind::Slash),
            '!' => {
                if self.matches('=') {
                    self.make_token(TokenKind::BangEqual)
                } else {
                    self.make_token(TokenKind::Bang)
                }
            }
            '=' => {
                if self.matches('=') {
                    self.make_token(TokenKind::EqualEqual)
                } else {
                    self.make_token(TokenKind::Equal)
                }
            }
            '>' => {
                if self.matches('=') {
                    self.make_token(TokenKind::GreaterEqual)
                } else {
                    self.make_token(TokenKind::Greater)
                }
            }
            '<' => {
                if self.matches('=') {
                    self.make_token(TokenKind::LessEqual)
                } else {
                    self.make_token(TokenKind::Less)
                }
            }
            '"' => self.string(),
            _ => self.error_token("Unexpected character."),
        }
    }

    // Check if the scanner has reached the end of the source
    fn is_at_end(&self) -> bool {
        self.source.get(self.current).is_some_and(|c| *c == '\0')
    }

    // Consume the current character and return it
    fn advance(&mut self) -> char {
        let c = self.source.get(self.current).unwrap().to_owned();
        self.current += 1;
        c
    }

    // Get the current character without consuming it
    fn peek(&self) -> char {
        self.source.get(self.current).unwrap().to_owned()
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.get(self.current + 1).unwrap().to_owned()
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
            text: self.source[self.start..self.current].iter().collect(),
            line: self.line,
        }
    }

    // Create an error token with the given message
    fn error_token(&self, message: &str) -> Token {
        Token {
            kind: TokenKind::Error,
            text: message.to_owned(),
            line: self.line,
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
                self.line += 1;
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
        let compare: String = self.source[self.start + start..self.current]
            .iter()
            .collect();

        if compare.as_str() == rest && self.current - self.start == start + length {
            return kind;
        }

        TokenKind::Identifier
    }

    fn identifier_type(&self) -> TokenKind {
        match self.source.get(self.start).unwrap() {
            'a' => self.check_keyword(1, 2, "nd", TokenKind::And),
            'e' => self.check_keyword(1, 3, "lse", TokenKind::Else),
            'f' => {
                if self.current - self.start > 1 {
                    match self.source.get(self.start + 1).unwrap() {
                        'a' => self.check_keyword(2, 3, "lse", TokenKind::False),
                        'o' => self.check_keyword(2, 1, "r", TokenKind::For),
                        'n' => self.check_keyword(2, 0, "", TokenKind::Fn),
                        _ => TokenKind::Identifier,
                    }
                } else {
                    TokenKind::Identifier
                }
            }
            'i' => self.check_keyword(1, 1, "f", TokenKind::If),
            'n' => self.check_keyword(1, 3, "one", TokenKind::None),
            'o' => self.check_keyword(1, 1, "r", TokenKind::Or),
            'p' => self.check_keyword(1, 4, "rint", TokenKind::Print),
            'r' => self.check_keyword(1, 5, "eturn", TokenKind::Return),
            's' => {
                if self.current - self.start > 1 {
                    match self.source.get(self.start + 1).unwrap() {
                        'e' => self.check_keyword(2, 2, "lf", TokenKind::Self_),
                        't' => self.check_keyword(2, 4, "ruct", TokenKind::Struct),
                        'u' => self.check_keyword(2, 4, "uper", TokenKind::Super),
                        _ => TokenKind::Identifier,
                    }
                } else {
                    TokenKind::Identifier
                }
            }
            't' => self.check_keyword(1, 3, "rue", TokenKind::True),
            'l' => self.check_keyword(1, 2, "et", TokenKind::Let),
            'w' => self.check_keyword(1, 4, "hile", TokenKind::While),
            _ => TokenKind::Identifier,
        }
    }

    fn identifier(&mut self) -> Token {
        while self.peek().is_alphabetic() || self.peek().is_digit(10) {
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
        self.make_token(TokenKind::Number)
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            return self.error_token("Unterminated string");
        }
        self.advance();
        self.make_token(TokenKind::String)
    }

    // Scan and tokenize the source code
}
