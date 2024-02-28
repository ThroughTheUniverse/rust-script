use super::*;

#[test]
fn test_left_paren() {
    let mut scanner = Scanner::new("(");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: LeftParen,
            lexeme: "(".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_right_paren() {
    let mut scanner = Scanner::new(")");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: RightParen,
            lexeme: ")".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_left_brace() {
    let mut scanner = Scanner::new("{");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: LeftBrace,
            lexeme: "{".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_right_brace() {
    let mut scanner = Scanner::new("}");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: RightBrace,
            lexeme: "}".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_comma() {
    let mut scanner = Scanner::new(",");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Comma,
            lexeme: ",".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_dot() {
    let mut scanner = Scanner::new(".");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Dot,
            lexeme: ".".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_minus() {
    let mut scanner = Scanner::new("-");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Minus,
            lexeme: "-".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_plus() {
    let mut scanner = Scanner::new("+");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Plus,
            lexeme: "+".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_semicolon() {
    let mut scanner = Scanner::new(";");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Semicolon,
            lexeme: ";".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_slash() {
    let mut scanner = Scanner::new("/");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Slash,
            lexeme: "/".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_star() {
    let mut scanner = Scanner::new("*");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Star,
            lexeme: "*".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_bang() {
    let mut scanner = Scanner::new("!");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Bang,
            lexeme: "!".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_bang_equal() {
    let mut scanner = Scanner::new("!=");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: BangEqual,
            lexeme: "!=".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_equal() {
    let mut scanner = Scanner::new("=");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Equal,
            lexeme: "=".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_equal_equal() {
    let mut scanner = Scanner::new("==");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: EqualEqual,
            lexeme: "==".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_greater() {
    let mut scanner = Scanner::new(">");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Greater,
            lexeme: ">".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_greater_equal() {
    let mut scanner = Scanner::new(">=");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: GreaterEqual,
            lexeme: ">=".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_less() {
    let mut scanner = Scanner::new("<");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Less,
            lexeme: "<".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_less_equal() {
    let mut scanner = Scanner::new("<=");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: LessEqual,
            lexeme: "<=".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_identifier() {
    let mut scanner = Scanner::new("abc123_");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Identifier,
            lexeme: "abc123_".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_string() {
    let mut scanner = Scanner::new("\"string \"");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: String,
            lexeme: "\"string \"".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_number() {
    let mut scanner = Scanner::new("123.4");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Number,
            lexeme: "123.4".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_and() {
    let mut scanner = Scanner::new("and");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: And,
            lexeme: "and".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_struct() {
    let mut scanner = Scanner::new("struct");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Struct,
            lexeme: "struct".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_else() {
    let mut scanner = Scanner::new("else");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Else,
            lexeme: "else".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_for() {
    let mut scanner = Scanner::new("for");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: For,
            lexeme: "for".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_fn() {
    let mut scanner = Scanner::new("fn");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Fn,
            lexeme: "fn".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_if() {
    let mut scanner = Scanner::new("if");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: If,
            lexeme: "if".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_none() {
    let mut scanner = Scanner::new("none");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: None,
            lexeme: "none".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_or() {
    let mut scanner = Scanner::new("or");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Or,
            lexeme: "or".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_print() {
    let mut scanner = Scanner::new("print");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Print,
            lexeme: "print".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_return() {
    let mut scanner = Scanner::new("return");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Return,
            lexeme: "return".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_self() {
    let mut scanner = Scanner::new("self");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Self_,
            lexeme: "self".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_true() {
    let mut scanner = Scanner::new("\n\ntrue");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: True,
            lexeme: "true".to_string(),
            line_number: 3
        },
        token
    );
}

#[test]
fn test_continue() {
    let mut scanner = Scanner::new("continue");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Continue,
            lexeme: "continue".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_mod() {
    let mut scanner = Scanner::new("%");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Modulo,
            lexeme: "%".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_power() {
    let mut scanner = Scanner::new("^");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Power,
            lexeme: "^".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_loop() {
    let mut scanner = Scanner::new("loop");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Loop,
            lexeme: "loop".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_let() {
    let mut scanner = Scanner::new("let");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Let,
            lexeme: "let".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_while() {
    let mut scanner = Scanner::new("while");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: While,
            lexeme: "while".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_error_char() {
    let mut scanner = Scanner::new("#$");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Error,
            lexeme: "Unexpected character.".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_error_string() {
    let mut scanner = Scanner::new("\"abc");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Error,
            lexeme: "Unterminated string.".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_eof() {
    let mut scanner = Scanner::new("");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: EOF,
            lexeme: "".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_comment() {
    let mut scanner = Scanner::new("//abcd");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: EOF,
            lexeme: "".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_whitespace() {
    let mut scanner = Scanner::new("   ");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: EOF,
            lexeme: "".to_string(),
            line_number: 1
        },
        token
    );
}

#[test]
fn test_sentence() {
    let mut scanner = Scanner::new("let a = 2;");
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Let,
            lexeme: "let".to_string(),
            line_number: 1
        },
        token
    );
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Identifier,
            lexeme: "a".to_string(),
            line_number: 1
        },
        token
    );
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Equal,
            lexeme: "=".to_string(),
            line_number: 1
        },
        token
    );
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Number,
            lexeme: "2".to_string(),
            line_number: 1
        },
        token
    );
    let token = scanner.scan_token();
    assert_eq!(
        Token {
            kind: Semicolon,
            lexeme: ";".to_string(),
            line_number: 1
        },
        token
    );
}
