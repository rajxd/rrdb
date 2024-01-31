use std::{collections::HashMap, hash::Hash};
use crate::parser::{
    create::CreateQuery
};
#[derive(Debug)]
pub struct Table{
    pub name: String,
    pub columns: Vec<ColumnHeader>,
    pub rows: HashMap<String, ColumnData>
}
#[derive(Debug)]
pub struct ColumnHeader{
    pub name: String,
    pub datatype: DataType
}

impl ColumnHeader{
    pub fn new(name:String, data_type:String) -> ColumnHeader{
        ColumnHeader{name:name, datatype:DataType::new(data_type)}
    }
}

#[derive(Debug)]
pub enum ColumnData {
    Int(Vec<i32>),
    Str(Vec<String>),
    Float(Vec<f32>),
    Bool(Vec<bool>),
    None,
}


#[derive(Debug)]
pub enum DataType{
    Int,
    Str,
    Float,
    Boolean,
    Invalid
}

impl DataType{
    pub fn new(datatype : String) -> DataType{
        match datatype.to_lowercase().as_ref() {
            "int" => DataType::Int,
            "string" => DataType::Str,
            "float" => DataType::Float,
            "bool" => DataType::Boolean,
            _ => DataType::Invalid
        }
    }
}

impl Table{
    pub fn new(cq : CreateQuery) -> Table{
        let mut table_columns :  Vec<ColumnHeader> = vec![];
        let mut table_data : HashMap<String, ColumnData> = HashMap::new();
        for column in &cq.columns{
            table_columns.push(ColumnHeader::new(column.name.to_string(), column.data_type.to_string()));

            table_data.insert(column.name.to_string(),
             {
                match DataType::new(column.data_type.to_string()) {
                    DataType::Boolean => ColumnData::Bool(vec![]),
                    DataType::Invalid => ColumnData::None,
                    DataType::Int => ColumnData::Int(vec![]),
                    DataType::Str => ColumnData::Str(vec![]),
                    DataType::Float => ColumnData::Float(vec![])
                }
            });
        }

        Table { name: cq.table_name, columns: table_columns, rows: table_data }
    }
}

