
use std::io::{stdout, stdin, Write};

#[path = "commands/command.rs"]
mod command;
mod database;
use crate::database::database::Database;

fn main() {
    println!("Hello Welcome to rrdb");
    let mut command = String::new();
    let mut db = Database::new();
    loop{
        print!("sdb> ");
        stdout().flush().unwrap();

        stdin().read_line(&mut command).expect("Error in reading from stdin");
        
        print!("CommandType: {:?}", command::get_command_type(&command));
        command = "".to_string();

        println!("{}", command);
        
        command = "".to_string();
    }
}
