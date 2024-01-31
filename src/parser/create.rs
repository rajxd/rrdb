use sqlparser::ast::{DataType, Statement};



pub struct CreateQuery{
    pub table_name : String,
    pub columns : Vec<ParsedColumn>
}

pub struct ParsedColumn{
    pub name : String,
    pub data_type : String
}

impl CreateQuery{
    pub fn new(statement: &Statement) -> Result<CreateQuery, String>{
        match statement{
            Statement::CreateTable { or_replace, temporary, external, global, if_not_exists, transient, name, columns, constraints, hive_distribution, hive_formats, table_properties, with_options, file_format, location, query, without_rowid, like, clone, engine, comment, auto_increment_offset, default_charset, collation, on_commit, on_cluster, order_by, partition_by, cluster_by, options, strict }
            =>{
                let mut parsed_columns: Vec<ParsedColumn> = vec![];

                for column in columns{
                    let column_data_type = match column.data_type  {
                        DataType::Varchar(_) => "String",
                        DataType::BigInt(_) => "int",
                        DataType::Int(_) => "int",
                        DataType::Boolean => "bool",


                        _ => {
                            println!("not matched on custom type");
                            "invalid"
                        }
                    };
                    parsed_columns.push(ParsedColumn { name: column.name.to_string(), data_type: column_data_type.to_string() })
                }
                return Ok(CreateQuery { table_name: name.to_string(), columns: parsed_columns });
            },
            _ => return Err("Invalid query".to_string())
        }
    }
}