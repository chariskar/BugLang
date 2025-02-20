#[derive(Debug,Clone)]
pub struct Variable {
    pub Value: Value,
}
#[derive(Debug,Clone)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}
use std::collections::HashMap;

pub struct VarManager {
    scopes: Vec<HashMap<String, Variable>>,
}

impl VarManager {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()], // Start with a global scope
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
                match &variable.Value {
                    Value::Integer(_i) => {
                        if let Ok(int_val) = new_value.parse::<i64>() {
                            variable.Value = Value::Integer(int_val);
                        }
                    }
                    Value::Float(_f) => {
                        if let Ok(float_val) = new_value.parse::<f64>() {
                            variable.Value = Value::Float(float_val);
                        }
                    }
                    Value::Boolean(_b) => {
                        if let Ok(bool_val) = new_value.parse::<bool>(){
                            variable.Value = Value::Boolean(bool_val)
                        }
                    }
                    Value::String(_s) => {
                        // Directly assign the new string value.
                        variable.Value = Value::String(new_value.to_string());
                    }
                }
                return Ok(()); // Successfully updated
            }
        }
        // If the variable is not found, return an error
        Err(format!("Undefined variable `{}`", name))
    }


    pub fn parse_value(&mut self, input: &str) -> Option<Value> {
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
