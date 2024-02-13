use std::{process, str};
use inquire::Select;

fn handle_command(command: &str) {
    match command {
        "scan" => println!("Not implemented yet"),
        "pair" => println!("Not implemented yet"),
        "exit" => {print!("exiting"); process::exit(0)},
        _ => println!("error")
    }
}

fn main() {
    println!("YAFTT - SN shell");

    loop {
        let command = Select::new("Select a command:", vec!["scan", "pair", "exit"]).prompt();
        
        match command {
            Ok(command) => handle_command(command),
            Err(_) => {println!("An error occurred, exiting"); process::exit(1)},
        }
    }

}
