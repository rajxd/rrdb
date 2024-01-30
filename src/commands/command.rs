use crate::database::database::Database;
use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser;
use sqlparser::ast::Statement;

#[derive(Debug)]
pub enum CommandType{
    MetaCommand(MetaCommand),
    DbCommand(DbCommand)
}

#[derive(Debug)]
pub enum MetaCommand{
    Exit,
    ListTables,
    Unknown(String)
}

#[derive(Debug)]
pub enum DbCommand{
    CreateTable(String),
    Insert(String),
    Unknown(String)
}

impl MetaCommand{
    pub fn new(cmd: String) -> MetaCommand{
        match cmd.as_ref(){
            ".exit" => MetaCommand::Exit,
            ".tables" => MetaCommand::ListTables,
            _ => MetaCommand::Unknown(cmd),

        }
    }
}


impl DbCommand{
    pub fn new(cmd: String) -> DbCommand{
        let v = cmd.split(" ").collect::<Vec<&str>>();
        match v[0]{
            "create" => DbCommand::CreateTable(cmd),
            "insert" => DbCommand::Insert(cmd),
            _ => DbCommand::Unknown(cmd)
        }
    }
}

pub fn get_command_type(cmd: &String) -> CommandType{
    match cmd.starts_with(".") {
        true => CommandType::MetaCommand(MetaCommand::new(cmd.to_owned())),
        false => CommandType::DbCommand(DbCommand::new(cmd.to_owned()))
    }
}

pub fn process_meta_command(meta_command : MetaCommand, db: &mut Database) {
    match meta_command {
        MetaCommand::Exit => std::process::exit(0),
        MetaCommand::ListTables => println!("TODO"),
        MetaCommand::Unknown(cmd) => println!("Unknown Command, {}", cmd)
        
    }
}

pub fn process_db_command(query : String, db: &mut Database) {
    let dialect = MySqlDialect{};
    let parse_sql = Parser::parse_sql(&dialect, &query);
    match parse_sql {
        Err( .. ) => {
            println!("Invalid command");
            return;
        }
        Ok( .. )=>print!("")
    }
    let statements = &parse_sql.unwrap();
    println!("statement: {:?}", statements);
    for s in statements {
        match s {
            Statement::CreateTable{ .. } =>{
                println!("Inside Create")
            }
            _ => println!("Invalid Command")
        }
    }

    
}
