use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Number(i32),
    Symbol(char),
    StringLiteral(String),
    Operator(char),
}

#[derive(Debug)]
pub enum Context {
    Global,
    InsideIf,
    InsideWhile,
    InsideFunction,
}

pub struct Tokenizer;

impl Tokenizer {
    pub fn tokenize(input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(&c) = chars.peek() {
            match c {
                'a'..='z' | 'A'..='Z' => {
                    let mut identifier = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch.is_alphanumeric() || ch == '_' {
                            identifier.push(ch);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    if identifier == "define" || identifier == "if" || identifier == "else" || identifier == "then" || identifier == "while" || identifier == "end" {
                        tokens.push(Token::Keyword(identifier));
                    } else {
                        tokens.push(Token::Identifier(identifier));
                    }
                }
                '0'..='9' => {
                    let mut number = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch.is_digit(10) {
                            number.push(ch);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Number(number.parse().unwrap()));
                }
                '"' => {
                    chars.next(); // Consume opening quote
                    let mut string_literal = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch == '"' {
                            break;
                        }
                        string_literal.push(ch);
                        chars.next();
                    }
                    chars.next(); // Consume closing quote
                    tokens.push(Token::StringLiteral(string_literal));
                }
                '=' | ':' | '+' | '-' | '*' | '/' | '<' | '>' | '{' | '}' | '(' | ')' | ';' => {
                    tokens.push(Token::Symbol(c));
                    chars.next();
                }
                _ if c.is_whitespace() => {
                    chars.next(); // Skip whitespace
                }
                _ => {
                    panic!("Unexpected character: {}", c);
                }
            }
        }

        tokens
    }
}
