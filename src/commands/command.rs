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


