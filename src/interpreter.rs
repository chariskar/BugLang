// TODO: Modularise the interpreter
#![allow(unused)]
use core::panic;
use std::any::Any;
use std::collections::*;
use crate::utils::tokeniser::*;
use crate::utils::variables::*;

pub struct Interpreter {
    var_manager: VarManager,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            var_manager: VarManager::new()
        }
    }

    pub fn interpret(&mut self, input: &str) {
        let tokens = Tokenizer::tokenize(input);
        let mut token_iter = tokens.iter().peekable();
        while let Some(token) = token_iter.next() {
            match token {
                
                Token::Keyword(k) if k == "print" => {
                    self.handle_print(&mut token_iter);
                },
                Token::Keyword(k) if k == "variable" => {
                    self.handle_variable_declaration(k,&mut token_iter);
                }, 
                Token::Identifier(k) if k == "if" => {
                    self.handle_if(&mut token_iter);                    
                },
                Token::Keyword(k) if k == "while" => {
                    self.handle_while(&mut token_iter);
                },
                Token::Keyword(k) if k == "update" => {
                    self.handle_variable_update(&mut token_iter);
                },
                Token::Identifier(k) if k == "for"=>{
                    self.handle_for(&mut token_iter);
                }

                _ => panic!("Unexpected token: {:?}", token),
            }
        }
    }

    fn handle_print<'a, I>(&mut self, tokens: &mut I)
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

        match tokens_collected.as_slice() {
            [value_token, Token::Symbol(';')] => {
                match value_token {
                    Token::StringLiteral(literal) => println!("{}", literal),
                    Token::Identifier(var_name) => {
                        if let Some(variable) = self.get_var(var_name) {
                            match &variable.Type {
                                Value::Integer(i) => println!("{}", i),
                                Value::Float(f) => println!("{}", f),
                                Value::Boolean(b) => println!("{}", b),
                                Value::String(s) => println!("{}", s),
                            }
                        } else {
                            panic!("Error: Undefined variable '{}'.", var_name);
                        }
                    }
                    Token::Number(num) => println!("{}", num),
                    Token::Float(f) => println!("{}", f),
                    Token::Boolean(b) => println!("{}", b),

                    
                    _ => panic!("Error: invalid token type for print statement"),
                }
            }
            _ => {
                panic!("Error: invalid print syntax");
            }
        }
    }



    // DO NOT TOUCH, IDK HOW I MADE IT WORK 
    fn handle_variable_declaration<'a, I>(&mut self,first_token: &String, tokens: &mut I)
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
        let mut equals = String::new();
        equals = "=".to_string();
        if let [

            Token::Identifier(ref var_name),
            Token::Operator(equals),
            Token::Number(value),
            Token::Symbol(';')
        ] = tokens_collected.as_slice()
        {
            if *first_token == "variable" {
                self.set_var(var_name, value.to_string().as_str());
            } else {
                panic!("Error: Invalid variable declaration.");
            }
        } else {
            panic!("Error: Incorrect variable declaration syntax.");
        }
    }

    fn handle_if<'a, I>(&mut self, tokens: &mut I)
    where
        I: Iterator<Item = &'a Token>,
    {
        // Collect tokens until we encounter a closing parenthesis
        let mut tokens_collected = Vec::new();
        while let Some(token) = tokens.next() {
            tokens_collected.push(token.clone());  // Clone is now valid
            if let Token::Symbol(')') = token {
                break;
            }
        }
    
        // Extract condition tokens as references
        let condition_tokens = tokens_collected
            .iter()
            .skip(1) // Skip the opening '('
            .take_while(|&token| !matches!(token, Token::Symbol(')'))); // Use `&token`
    
        // Evaluate the condition
        if self.evaluate_condition(&mut condition_tokens.cloned()).unwrap_or(false) {
            self.execute_block(tokens);
        } else {
            self.skip_block(tokens);
        }
    }
    
    fn execute_block<'a, I>(&mut self, tokens: &mut I)
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut block_depth = 1;
        let mut block_tokens = Vec::new();

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

        self.interpret(&Tokenizer::reconstruct(&block_tokens));
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

    fn evaluate_condition<'a, I>(&mut self, tokens: &mut I) -> Option<bool>
    where
        I: Iterator<Item = Token>,
    {
        let mut left_operand: Option<i64> = None;
        let mut operator: Option<char> = None;
    
        // Parse the left operand
        if let Some(token) = tokens.next() {
            match token {
                Token::Identifier(var_name) => {
                    if let Some(var) = self.get_var(var_name.as_str()) {
                        if let Value::Integer(i) = var.Type {
                            left_operand = Some(i);
                        } else {
                            return None;
                        }
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
            match token {
                Token::Operator(op) => {
                    if op.len() == 1 {
                        operator = Some(op.chars().next().unwrap());
                    } else {
                        return None; // Handle multi-character operators if needed
                    }
                }
                _ => return None,
            }
        }
        
    
        // Parse the right operand
        if let Some(token) = tokens.next() {
            match token {
                Token::Number(n) => {
                    if let (Some(lhs), Some(op)) = (left_operand, operator) {
                        match op {
                            '<' => return Some(lhs < n),
                            '>' => return Some(lhs > n),
                            '=' => return Some(lhs == n),
                            _ => return None,
                        }
                    }
                }
                _ => return None,
            }
        }
    
        None
    }
    


    fn handle_while<'a, I>(&mut self, tokens: &mut I)
    where
        I: Iterator<Item = &'a Token>,
    {
        // Collect tokens until we encounter a closing parenthesis
        let mut tokens_collected = Vec::new();
        while let Some(token) = tokens.next() {
            tokens_collected.push(token.clone());
            if let Token::Symbol(')') = token {
                break;
            }
        }

        // Extract condition tokens (excluding the surrounding parentheses)
        let condition_tokens: Vec<Token> = tokens_collected
            .iter()
            .skip(1) // Skip the opening '('
            .take_while(|&token| !matches!(token, Token::Symbol(')')))
            .cloned()
            .collect();

        // Prepare to collect the body tokens
        let mut body_tokens = Vec::new();
        let mut open_braces = 0;

        // Collect the body tokens (assuming a `{}` block)
        while let Some(token) = tokens.next() {
            match token {
                Token::Symbol('{') => open_braces += 1,
                Token::Symbol('}') => {
                    open_braces -= 1;
                    if open_braces == 0 {
                        break;
                    }
                }
                _ => {}
            }
            body_tokens.push(token.clone());
        }

        // Execute the loop
        loop {
            // Evaluate the condition
            let condition_result = self.evaluate_condition(
                &mut condition_tokens.clone().into_iter()
            );
            if !condition_result.unwrap_or(false) {
                break;
            }
            self.execute_block(&mut body_tokens.iter());
        }
        }

    fn get_var(&mut self,var_name: &str) -> Option<&Variable> {
        return self.var_manager.get(var_name);
    }

    fn set_var(&mut self,var_name: &str, value: &str){
        if !self.get_var(var_name).is_none(){
            self.var_manager.assign(var_name, value);
            
        } else {
            match self.var_manager.parse_value(value) {
                Some(Value::Integer(i)) => {
                    let variable = Variable {
                        Type: Value::Integer(i)
                    };
                    self.var_manager.define(var_name.to_string(),variable);
                },
                Some(Value::Float(f)) => {
                    let variable = Variable {
                        Type: Value::Float(f)
                    };
                    self.var_manager.define(var_name.to_string(),variable);
                },
                Some(Value::Boolean(b)) => {
                    let variable = Variable {
                        Type: Value::Boolean(b)
                    };
                    self.var_manager.define(var_name.to_string(),variable);
                },
                Some(Value::String(s)) => {
                    let variable = Variable {
                        Type: Value::String(s)
                    };
                    self.var_manager.define(var_name.to_string(),variable);
                },
                None => panic!("Invalid value"),
            }
            
        }
    }

    fn handle_variable_update<'a, I>(&mut self, tokens: &mut I)
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

        match tokens_collected.as_slice() {
            [Token::Identifier(var_name), Token::Symbol('+'), Token::Symbol('+'), Token::Symbol(';')] => {
                if let Some(variable) = self.get_var(var_name) {
                    match &variable.Type {
                        Value::Integer(i) => {
                            let new_value = *i + 1;
                            self.set_var(var_name, new_value.to_string().as_str());
                        },
                        _ => panic!("Invalid variable type for increment"),
                    }
                } else {
                    panic!("Tried updating a non-existing variable: {}", var_name);
                }
            },
            [Token::Identifier(var_name), Token::Symbol('-'), Token::Symbol('-'), Token::Symbol(';')] => {
                if let Some(variable) = self.get_var(var_name) {
                    match &variable.Type {
                        Value::Integer(i) => {
                            let new_value = *i - 1;
                            self.set_var(var_name, new_value.to_string().as_str());
                        },
                        _ => panic!("Invalid variable type for decrement"),
                    }
                } else {
                    panic!("Tried updating a non-existing variable: {}", var_name);
                }
            },
            [Token::Identifier(var_name), Token::Symbol(op), Token::Number(amount), Token::Symbol(';')] if *op == '+' || *op == '-' => {
                if let Some(variable) = self.get_var(var_name) {
                    match &variable.Type {
                        Value::Integer(i) => {
                            let mut new_value = *i;
                            if *op == '+' {
                                new_value += amount;
                            } else {
                                new_value -= amount;
                            }
                            self.set_var(var_name, new_value.to_string().as_str());
                        },
                        _ => panic!("Invalid variable type for arithmetic update"),
                    }
                } else {
                    panic!("Tried updating a non-existing variable: {}", var_name);
                }
            },
            [Token::Identifier(var_name), Token::Operator(op), value_token, Token::Symbol(';')] if op == "=" => {
                let value_str = match value_token {
                    Token::StringLiteral(s) => format!("\"{}\"", s), // Ensure string is quoted
                    Token::Number(n) => n.to_string(),
                    Token::Float(f) => f.to_string(),
                    Token::Boolean(b) => b.to_string(),
                    _ => panic!("Unsupported value type in update"),
                };
                self.set_var(var_name, &value_str);
            },
            _ => panic!("Invalid variable update syntax: {:?}", tokens_collected),
        }
    }

    fn handle_for<'a, I>(&mut self, tokens: &mut I)
    where
        I: Iterator<Item = &'a Token>,
    {
        // Parse initialization, condition, and update sections
        let (init_tokens, condition_tokens, update_tokens) = self.parse_for_header(tokens);

        // Parse the body 
        let body_tokens = self.parse_block(tokens);

        if !init_tokens.is_empty() {
            self.process_tokens(&init_tokens, "initialization");
        }

        // Execute the for loop
        loop {
            // Evaluate the condition dynamically on each iteration
            let condition_result = self
                .evaluate_condition(&mut condition_tokens.clone().into_iter())
                .unwrap_or(false);

            // Break the loop if the condition is false
            if !condition_result {
                break;
            }

            // Execute the block
            self.execute_block(&mut body_tokens.iter());

            // Process update
            if !update_tokens.is_empty() {
                self.process_tokens(&update_tokens, "update");
            }
        }
    }


    fn parse_for_header<'a, I>(
        &mut self,
        tokens: &mut I,
    ) -> (Vec<Token>, Vec<Token>, Vec<Token>)
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut init_tokens = Vec::new();
        let mut condition_tokens = Vec::new();
        let mut update_tokens = Vec::new();
        let mut stage = 0;

        while let Some(token) = tokens.next() {
            match token {
                Token::Symbol(';') => stage += 1,
                Token::Symbol('(') | Token::Symbol(')') => continue,
                Token::Symbol('{') => break, // End of the header
                _ => match stage {
                    0 => init_tokens.push(token.clone()),
                    1 => condition_tokens.push(token.clone()),
                    2 => update_tokens.push(token.clone()),
                    _ => panic!("Unexpected token in for loop header: {:?}", token),
                },
            }
        }

        (init_tokens, condition_tokens, update_tokens)
    }

    fn parse_block<'a, I>(&mut self, tokens: &mut I) -> Vec<Token>
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut block_tokens = Vec::new();
        let mut open_braces = 1; // We've already seen the opening `{`

        while let Some(token) = tokens.next() {
            match token {
                Token::Symbol('{') => open_braces += 1,
                Token::Symbol('}') => {
                    open_braces -= 1;
                    if open_braces == 0 {
                        break; // End of block
                    }
                }
                _ => {}
            }
            block_tokens.push(token.clone());
        }

        block_tokens
    }

    
    fn process_tokens(&mut self, tokens: &[Token], context: &str) {
        match tokens {
            [Token::Identifier(var), Token::Operator(ref op), Token::Number(value)] if op == "=" => {
                self.set_var(var, value.to_string().as_str());
            }
            [Token::Identifier(var), Token::Symbol('+'), Token::Operator(ref op), Token::Number(value)] if op == "=" => {
                if let Some(existing_value) = self.get_var(var.clone().as_str()) {
                    if let Value::Integer(existing_int) = existing_value.Type {
                        self.set_var(var, (existing_int + value).to_string().as_str());
                    } else {
                        panic!("Variable {} is not an integer", var);
                    }
                } else {
                    panic!("Variable {} not found", var);
                }
            }
            // Handle other patterns (if needed)
            _ => panic!("Invalid token pattern for {}: {:?}", context, tokens),
        }
    }
    

}
