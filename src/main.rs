mod utils;
mod interpreter;
use std::fs;
fn main() -> std::io::Result<()> {
    let file_path = "test.txt";
    
    // Read the entire file into a String
    let contents = fs::read_to_string(file_path)?;

    Ok({})
}

#[cfg(test)]
mod tests {
    use interpreter::Interpreter;
    use super::*; // Import the main module and its contents
    use std::fs;
#[test]
    fn test_interpreter_execution() {
        // Prepare a temporary test file
        let test_file_path = r"C:\Users\chari\Documents\GitHub\pseudolanguage\src\test.txt";

        let interpreter =&mut Interpreter::new(); 

        // Read and interpret the file
        let contents = fs::read_to_string(test_file_path).expect("Failed to read test file");
        interpreter.interpret(&contents);


    }
}
