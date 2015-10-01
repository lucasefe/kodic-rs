extern crate hyper;
extern crate rustc_serialize;

use std::env;
use std::process;

mod commands;
mod client;

static VALID_COMMANDS: &'static [ &'static str ] = &["version"];

// TODO: Check for proper error code to exit with.
fn exit_with_error(message: &str) {
    println!("ERR: {}", message);    
    process::exit(1);
}

fn run(command: &str) {
    if !&VALID_COMMANDS.contains(&command) {
        exit_with_error(&format!("Invalid command: {}", command));
    }

    println!("Executing command: {}", command);

    let request = commands::version::Request::new();
    let data = client::execute(request);
    let response = commands::version::Response::new(data);

    println!("Version: {}", response);
}

fn main() {
    match env::args().nth(1) {
        Some(command)   => run(&command),
        None            => exit_with_error("you need to provide a command"),
    }
}
