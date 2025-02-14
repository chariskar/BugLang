#[derive(Debug)]
pub struct Variable {
    pub Type: Value,
}
#[derive(Debug)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}
use std::{any, collections::HashMap};

pub struct VarManager {
    scopes: Vec<HashMap<String, Variable>>,
}

impl VarManager {
    pub fn new() -> Self {
        Self {
            scopes: vec![
                HashMap::new()], // Start with a global scope
        }
    }

    // Add a variable to the current scope
    pub fn define(&mut self, name: String, value: Variable) {
        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.insert(name, value);
        }
    }

    // Retrieve a variable, searching from the innermost scope outward
    pub fn get(&self, name: &str) -> Option<&Variable> {
        for scope in self.scopes.iter().rev() {
            if let Some(var) = scope.get(name) {
                return Some(var);
            }
        }
        None
    }

    // Update a variable in the nearest scope where it is defined
    pub fn assign(&mut self, name: &str, new_value: &str) -> Result<(), String> {
        // Iterate over scopes from innermost to outermost
        for scope in self.scopes.iter_mut().rev() {
            if let Some(variable) = scope.get_mut(name) {
                match &variable.Type {
                    Value::Integer(i) => {
                        if let Ok(int_val) = new_value.clone().parse::<i64>() {
                            variable.Type = Value::Integer(int_val);
                        }     
                    },
                    Value::Float(f) => {
                        if let Ok(int_val) = new_value.clone().parse::<f64>() {
                            variable.Type = Value::Float(int_val);
                        }                        },
                    Value::Boolean(b) => {},
                    Value::String(s) => {},
                }
                return Ok(()); // Successfully updated
            }
        }
        // If the variable is not found, return an error
        Err(format!("Undefined variable `{}`", name))
    }
    

    // Add a new scope (e.g., for a function or block)
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    // Remove the innermost scope
    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn parse_value(&mut self,input: &str) -> Option<Value> {
        // Try parsing as an integer
        if let Ok(int_val) = input.parse::<i64>() {
            return Some(Value::Integer(int_val));
        }
        
        // Try parsing as a float
        if let Ok(float_val) = input.parse::<f64>() {
            return Some(Value::Float(float_val));
        }

        // Try parsing as a boolean
        if let "true" | "false" = input.to_lowercase().as_str() {
            return Some(Value::Boolean(input.parse().unwrap()));
        }

        // If it doesn't match any type, return it as a String
        Some(Value::String(input.to_string()))
    }
}
