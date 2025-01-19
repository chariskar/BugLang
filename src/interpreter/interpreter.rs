use std::collections::HashMap;
use crate::utils::{Token, Context, Tokenizer};

struct Operator {
    value: String,
}

pub struct Interpreter {
    context: Context,
    variables: HashMap<String, i32>, // Variable storage
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            context: Context::Global,
            variables: HashMap::new(),
        }
    }


    pub fn parse_and_interpret(&mut self, input: &str) {
        let tokens: Vec<Token> = Tokenizer::tokenize(input);
        let mut token_iter = tokens.iter().peekable();

        while let Some(token) = token_iter.next() {
            match token {
                Token::Keyword(ref k) if k == "define" => {
                    self.handle_variable_declaration(&mut token_iter);
                }
                Token::Keyword(ref k) if k == "print" => {
                    self.handle_print(&mut token_iter);
                }
                Token::Keyword(ref k) if k == "if" => {
                    self.handle_if(&mut token_iter);
                }
                Token::Keyword(ref k) if k == "while" => {
                    self.handle_while(&mut token_iter);
                }
                _ => {
                    println!("Unexpected token: {:?}", token);
                }
            }
        }
    }
    
    fn handle_print<'a, I>(&mut self, tokens: &mut I)
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut print_tokens = Vec::new();
    
        // Collect tokens until we hit the semicolon
        while let Some(token) = tokens.next() {
            print_tokens.push(token.clone());
            if let Token::Symbol(';') = token {
                break; // Stop when we encounter a semicolon
            }
        }
        println!("Tokens received by handle_print: {:?}", print_tokens);
    
        // Process the print statement
        if let Some(Token::StringLiteral(s)) = print_tokens.get(1) {
            println!("{}", s); // Print the string literal
        } else if let Some(Token::Identifier(var_name)) = print_tokens.get(1) {
            if let Some(value) = self.variables.get(var_name) {
                println!("{}", value); // Print the variable value
            } else {
                println!("Error: Variable '{}' not found.", var_name); // Handle variable not found
            }
        } else {
            println!("Error: Invalid print statement syntax.");
        }
    }


    fn handle_variable_declaration<'a, I>(&mut self, tokens: &mut I)
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut tokens_collected = Vec::new();
        while let Some(token) = tokens.next() {
            tokens_collected.push(token.clone());
            if let Token::Symbol(';') = token {
                break;
            }
        }

        if tokens_collected.len() == 7 {
            if let [
                Token::Identifier(ref variable),
                Token::Identifier(ref var_name),
                Token::Symbol(':'),
                Token::Identifier(ref var_type),
                Token::Symbol('='),
                Token::Number(value),
                Token::Symbol(';'),
            ] = tokens_collected[..]
            {
                if *variable == "variable" && *var_type == "number" {
                    self.variables.insert(var_name.to_string(), value);
                    println!("Variable declared: {} = {}", var_name, value);
                } else {
                    println!("Error: Invalid variable declaration.");
                }
            } else {
                println!("Error: Invalid token sequence for variable declaration.");
            }
        } else {
            println!("Error: Incorrect number of tokens for variable declaration.");
        }
    }
    
    fn handle_if<'a, I>(&mut self, tokens: &mut I)
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut condition_tokens: Vec<Token> = tokens.map(|t| t.clone()).collect();
        println!("Condition tokens: {:?}", condition_tokens);
    
        if let Some(true) = self.evaluate_condition(&mut condition_tokens.into_iter()) {
            self.execute_block(tokens);
        } else {
            if let Some(Token::Keyword(ref keyword)) = tokens.next() {
                if keyword == "else" {
                    self.handle_else(tokens);
                }
            }
        }
    }
    
    fn handle_while<'a, I>(&mut self, tokens: &mut I)
    where
        I: Iterator<Item = &'a Token>,
    {
        if let Some(Token::Symbol('(')) = tokens.next() {
            // Collect tokens for the condition
            let condition_tokens: Vec<Token> = tokens.cloned().collect();
    
            // Evaluate the condition
            let mut condition_result = self
                .evaluate_condition(&mut condition_tokens.iter().cloned())
                .unwrap_or(false);
    
            if let Some(Token::Symbol(')')) = tokens.next() {
                let mut loop_count = 0;
                let max_iterations = 100;
    
                while condition_result && loop_count < max_iterations {
                    self.execute_block(tokens);
    
                    // Re-evaluate the condition
                    condition_result = self
                        .evaluate_condition(&mut condition_tokens.iter().cloned())
                        .unwrap_or(false);
    
                    loop_count += 1;
                    if loop_count >= max_iterations {
                        println!("Error: Exceeded maximum iterations in while loop.");
                        break;
                    }
                }
            } else {
                println!("Error: Expected closing parenthesis ')'.");
            }
        } else {
            println!("Error: Expected opening parenthesis '('.");
        }
    }
    
    fn execute_block<'a, I>(&mut self, tokens: &mut I)
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut block_depth = 1;
        let mut block_tokens = Vec::new();
        println!("Entering execute_block");

        while let Some(token) = tokens.next() {
            match token {
                Token::Symbol('}') => {
                    block_depth -= 1;
                    if block_depth == 0 {
                        break;
                    }
                }
                Token::Symbol('{') => {
                    block_depth += 1;
                }
                _ => {
                    block_tokens.push(token.clone());
                }
            }
        }

        let mut block_iter = block_tokens.iter();
        while let Some(token) = block_iter.next() {
            match token {
                Token::Keyword(ref k) if k == "print" => {
                    println!("Executing print");
                    self.handle_print(&mut block_iter); // Pass the correct iterator
                }
                Token::Keyword(ref k) if k == "while" => {
                    println!("Executing while loop");
                    self.handle_while(&mut block_iter);
                }
                Token::Keyword(ref k) if k == "define" => {
                    println!("Handling variable declaration");
                    self.handle_variable_declaration(&mut block_iter);
                }
                Token::Keyword(ref k) if k == "else" => {
                    println!("Executing else block");
                    self.handle_else(&mut block_iter);
                }
                _ => {
                    println!("Unexpected token in block: {:?}", token);
                }
            }
        }

        println!("Exiting execute_block");
    }
    

    fn skip_block<'a, I>(&mut self, tokens: &mut I)
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut block_depth = 1;
        while let Some(token) = tokens.next() {
            match token {
                Token::Symbol('}') => {
                    block_depth -= 1;
                    if block_depth == 0 {
                        break;
                    }
                }
                Token::Symbol('{') => {
                    block_depth += 1;
                }
                _ => {}
            }
        }
    }

    fn evaluate_condition<'a, I>(&self, tokens: &mut I) -> Option<bool>
    where
        I: Iterator<Item = Token>,
    {
        let mut left_operand: Option<i32> = None;
        let mut operator: Option<char> = None;
    
        // Parse the left operand
        if let Some(token) = tokens.next() {
            match token {
                Token::Identifier(var_name) => {
                    if let Some(value) = self.variables.get(&var_name) {
                        left_operand = Some(*value);
                    } else {
                        return None;
                    }
                }
                Token::Number(n) => left_operand = Some(n),
                _ => return None,
            }
        }
    
        // Parse the operator
        if let Some(token) = tokens.next() {
            if let Token::Symbol(op) = token {
                if op == '<' || op == '>' || op == '=' {
                    operator = Some(op);
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
    
        // Parse the right operand
        if let Some(operator) = operator {
            if let Some(Token::Number(rhs)) = tokens.next() {
                if let Some(lhs) = left_operand {
                    match operator {
                        '<' => return Some(lhs < rhs),
                        '>' => return Some(lhs > rhs),
                        '=' => return Some(lhs == rhs),
                        _ => return None,
                    }
                }
            }
        }
    
        None
    }
    

    fn handle_else<'a, I>(&mut self, tokens: &mut I)
    where
        I: Iterator<Item = &'a Token>,
    {
        self.skip_block(tokens);
        println!("Handling else block...");
    }
}
