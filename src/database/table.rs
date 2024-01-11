use std::collections::HashMap;

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
    Boolan,
    Invalid
}

