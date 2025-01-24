
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Number(i32),
    Symbol(char),
    StringLiteral(String),
    Operator(String),
}

pub struct Tokenizer;

impl Tokenizer {
    pub fn tokenize(input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(&c) = chars.peek() {
            match c {
                // Identifiers and keywords
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
                    if [ "if", "else", "while", "and", "or", "not","for"].contains(&identifier.as_str()) {
                        tokens.push(Token::Identifier(identifier));
                    } else if ["print","variable", "while","update"].contains(&&identifier.as_str()){
                        tokens.push(Token::Keyword(identifier));
                    } else {
                        tokens.push(Token::Identifier(identifier));
                    }
                }
                // Numbers
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
                // String literals
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
                // Multi-character operators
                '=' | '!' | '<' | '>' => {
                    let mut operator = String::new();
                    operator.push(c);
                    chars.next();
                    if let Some(&next_char) = chars.peek() {
                        if next_char == '=' {
                            operator.push(next_char);
                            chars.next();
                        }
                    }
                    tokens.push(Token::Operator(operator));
                }
                // Single-character symbols
                ':' | '+' | '-' | '*' | '/' | '{' | '}' | '(' | ')' | ';' => {
                    tokens.push(Token::Symbol(c));
                    chars.next();
                }
                // Comments
                '/' => {
                    chars.next(); // Consume '/'
                    if let Some(&next_char) = chars.peek() {
                        if next_char == '/' {
                            // Single-line comment
                            while let Some(&ch) = chars.peek() {
                                if ch == '\n' {
                                    break;
                                }
                                chars.next();
                            }
                        } else {
                            tokens.push(Token::Symbol('/'));
                        }
                    }
                }
                // Whitespace
                _ if c.is_whitespace() => {
                    chars.next(); // Skip whitespace
                }
                // Unexpected characters
                _ => {
                    panic!("Unexpected character: {}", c);
                }
            }
        }

        tokens
    }

    pub fn reconstruct(tokens: &[Token]) -> String {
        tokens.iter().map(|token| match token {
            Token::Keyword(kw) => kw.clone(),
            Token::Identifier(id) => id.clone(),
            Token::Number(num) => num.to_string(),
            Token::Symbol(sym) => sym.to_string(),
            Token::StringLiteral(lit) => format!("\"{}\"", lit),
            Token::Operator(op) => op.clone(),
        }).collect::<Vec<_>>().join(" ")
    }
}
