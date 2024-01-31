
use std::io::{stdout, stdin, Write};

#[path = "commands/command.rs"]
mod command;
mod database;
mod parser;
use crate::{database::database::Database, command::CommandType};

fn main() {
    println!("Hello Welcome to rrdb");
    let mut command = String::new();
    let mut db = Database::new();
    loop{
        print!("sdb> ");
        stdout().flush().unwrap();

        stdin().read_line(&mut command).expect("Error in reading from stdin");
        
        let command_type = command::get_command_type(&command.trim().to_owned());
        match command_type{
            CommandType::MetaCommand(meta_command) => {
                command::process_meta_command(meta_command, &mut db);
            }
            CommandType::DbCommand(_c) => {
                command::process_db_command(command.trim().to_string(), &mut db);
            }
        }

        // println!("{}", command);
        
        command = "".to_string();
    }
}
