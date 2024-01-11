use std::io::{stdout, stdin, Write};

fn main() {
    println!("Hello Welcome to rrdb");
    let mut command = String::new();
    loop{
        print!("sdb> ");
        stdout().flush().unwrap();

        stdin().read_line(&mut command).expect("Error in reading from stdin");

        println!("{}", command);
        
        command = "".to_string();
    }
}
