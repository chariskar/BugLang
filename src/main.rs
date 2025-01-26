mod utils;
mod interpreter;
use std::{fs,env};
use clap::{Arg, Command};
use interpreter::Interpreter;

fn main() {
    // Define the CLI arguments and subcommands
    let matches = Command::new("My CLI Tool")
        .version("1.0")
        .author("Charis charis.karametos@gmail.com")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Sets an input file")
                .required(false),  // Make it optional
        )
        .get_matches();
    let arg = matches.args_present();
    if !arg{
        println!("This is the cli tool for bugland, cuase im bored")
    } else {
        let path = matches.get_one::<String>("input");
        if let Some(path_str) = path {
            let current_dir = env::current_dir().expect("Failed to get current directory");
            let relative_path = current_dir.join(path_str); // `path_str` is a &String, which implements AsRef<Path>

            if fs::metadata(&relative_path).is_ok() {
                let interpreter =&mut Interpreter::new(); 
                let contents = fs::read_to_string(path_str).expect("Failed to read test file");
                interpreter.interpret(&contents);
            } else {
                panic!("File not found.");
            }
        } else {
            panic!("No input file provided.");
        }
    }
}
