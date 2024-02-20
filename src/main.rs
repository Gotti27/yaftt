
use std::{process, str};
use inquire::Select;

mod ble_auth;
use ble_auth::{BLEAuthenticatorFactory};

async fn pair_command() {
    match BLEAuthenticatorFactory::new().create().await.scan().await {
        Ok(()) => (),
        Err(_) => println!("Error during Scan command!")
    }
}

async fn handle_command(command: &str) {
    match command {
        "scan" => pair_command().await,
        "pair" => println!("Not implemented yet"),
        "exit" => {print!("exiting"); process::exit(0)},
        _ => println!("error")
    }
}

#[tokio::main]
async fn main() {
    println!("YAFTT - SN shell");

    loop {
        let command = Select::new("Select a command:", vec!["scan", "pair", "exit"]).prompt();
        
        match command {
            Ok(command) => handle_command(command).await,
            Err(_) => {println!("An error occurred, exiting"); process::exit(1)},
        }
    }

}
