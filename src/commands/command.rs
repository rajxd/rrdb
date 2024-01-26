use crate::database::database::Database;

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
    
}

pub fn process_db_command(cmd : String, db: &mut Database) {
    
}
