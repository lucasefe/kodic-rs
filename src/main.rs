use std::env;
use std::process;

// TODO: Check for proper error code to exit with. 
fn exit_with_error(message: &str) {
    println!("ERR: {}", message);
    process::exit(1);
}

fn run(command: &str) {
    let valid_commands = &vec!["version"];
    
    if !valid_commands.contains(&command) {
        exit_with_error(&format!("Invalid command: {}", command));
    }
    println!("{}", command);
}

// parse action
// build
// execute
// parse response
// render

fn main() {
    match env::args().nth(1) {
        Some(command)   => run(&command),
        None            => exit_with_error("you need to provide a command"),
    }
}
