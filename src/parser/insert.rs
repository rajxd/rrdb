
use sqlparser::ast::{Expr, Query, SetExpr, Statement, Value, Values};

pub struct InsertQuery {
    pub table_name: String,
    pub columns: Vec<String>,
    pub values: Vec<Vec<String>>,
}

impl InsertQuery {
    pub fn new(statement: &Statement) {
        let mut t_name: Option<String> = None;
        let mut t_columns: Vec<String> = vec![];
        let mut t_values: Vec<Vec<String>> = vec![];

        if let Statement::Insert {
            table_name,
            columns,
            source,
            or,
            ignore,
            into,
            table_alias,
            overwrite,
            partitioned,
            after_columns,
            table,
            on,
            returning,
            replace_into,
            priority,
        } = statement
        {
            t_name = Some(table_name.to_string());

            for c in columns {
                t_columns.push(c.value.to_string());
            }

            match source {
                Some(s) => match &**s {
                    Query {
                        with,
                        body,
                        order_by,
                        limit,
                        limit_by,
                        offset,
                        fetch,
                        locks,
                        for_clause,
                    } => {
                        if let SetExpr::Values(values) = &**body {
                            if let Values { explicit_row, rows } = values {
                                for i in rows {
                                    let mut value_set: Vec<String> = vec![];

                                    for e in i {
                                        match e {
                                            Expr::Value(v) => match v {
                                                Value::Number(n, r) => {
                                                    value_set.push(n.to_string());
                                                }
                                                Value::Boolean(b) => {
                                                    value_set.push(b.to_string());
                                                }
                                                Value::SingleQuotedString(s) => {
                                                    value_set.push(s.to_string());
                                                }
                                                Value::DollarQuotedString(s) => {
                                                    value_set.push(s.to_string());
                                                }
                                                Value::EscapedStringLiteral(s) => {
                                                    value_set.push(s.to_string());
                                                }
                                                Value::SingleQuotedByteStringLiteral(s) => {
                                                    value_set.push(s.to_string());
                                                }
                                                Value::DoubleQuotedByteStringLiteral(s) => {
                                                    value_set.push(s.to_string());
                                                }
                                                Value::RawStringLiteral(s) => {
                                                    value_set.push(s.to_string());
                                                }
                                                Value::NationalStringLiteral(s) => {
                                                    value_set.push(s.to_string());
                                                }
                                                Value::HexStringLiteral(s) => {
                                                    value_set.push(s.to_string());
                                                }
                                                Value::DoubleQuotedString(s) => {
                                                    value_set.push(s.to_string());
                                                }
                                                Value::Null => {
                                                    value_set.push("Null".to_string());
                                                }
                                                Value::Placeholder(s) => {
                                                    value_set.push(s.to_string());
                                                }
                                                Value::UnQuotedString(s) => {
                                                    value_set.push(s.to_string());
                                                }
                                            },
                                            Expr::Identifier(idf) => {
                                                value_set.push(idf.value.to_string());
                                            }
                                            _ => {}
                                        }
                                    }
                                    t_values.push(value_set);
                                }
                            }
                        }
                    }
                },
                None => (),
            }
            
        }

        let _ = match t_name {
            Some(t) => Ok(InsertQuery {
                table_name: t,
                columns: t_columns,
                values: t_values
            }),
            None => Err(String::from("Error parsing insert statement")),
        };
    }
}
