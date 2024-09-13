use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sqlx::Postgres;

#[derive(Clone, Debug)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

// Existing implementations
impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::Int(v)
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Float(v)
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::String(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::String(v.to_string())
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Bool(v)
    }
}

// New implementations for other integer types
impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Value::Int(v as i64)
    }
}

impl From<i16> for Value {
    fn from(v: i16) -> Self {
        Value::Int(v as i64)
    }
}

impl From<i8> for Value {
    fn from(v: i8) -> Self {
        Value::Int(v as i64)
    }
}

impl From<u32> for Value {
    fn from(v: u32) -> Self {
        Value::Int(v as i64)
    }
}

impl From<u16> for Value {
    fn from(v: u16) -> Self {
        Value::Int(v as i64)
    }
}

impl From<u8> for Value {
    fn from(v: u8) -> Self {
        Value::Int(v as i64)
    }
}

#[derive(Clone, Debug)]
pub enum FilterCondition {
    Eq(Value),
    Ne(Value),
    Gt(Value),
    Lt(Value),
    Gte(Value),
    Lte(Value),
    Between(Value, Value),
    In(Vec<Value>),
    Like(String),
}

#[derive(Default, Clone, Debug)]
pub struct Filter {
    conditions: HashMap<String, FilterCondition>,
}

impl Filter {
    pub fn new() -> Self {
        Self {
            conditions: HashMap::new(),
        }
    }

    pub fn add<S>(&mut self, field: S, condition: FilterCondition) -> &mut Self
    where
        S: Into<String>,
    {
        self.conditions.insert(field.into(), condition);
        self
    }

    pub fn build_for_sqlx(&self) -> (String, Vec<Value>) {
        let mut conditions = Vec::new();
        let mut args: Vec<Value> = Vec::new();

        for (field, condition) in &self.conditions {
            match condition {
                FilterCondition::Eq(value) => {
                    conditions.push(format!("{} = ${}", field, args.len() + 1));
                    args.push(value_to_encode(value));
                }
                FilterCondition::Ne(value) => {
                    conditions.push(format!("{} != ${}", field, args.len() + 1));
                    args.push(value_to_encode(value));
                }
                FilterCondition::Gt(value) => {
                    conditions.push(format!("{} > ${}", field, args.len() + 1));
                    args.push(value_to_encode(value));
                }
                FilterCondition::Lt(value) => {
                    conditions.push(format!("{} < ${}", field, args.len() + 1));
                    args.push(value_to_encode(value));
                }
                FilterCondition::Gte(value) => {
                    conditions.push(format!("{} >= ${}", field, args.len() + 1));
                    args.push(value_to_encode(value));
                }
                FilterCondition::Lte(value) => {
                    conditions.push(format!("{} <= ${}", field, args.len() + 1));
                    args.push(value_to_encode(value));
                }
                FilterCondition::Between(value1, value2) => {
                    conditions.push(format!(
                        "{} BETWEEN ${} AND ${}",
                        field,
                        args.len() + 1,
                        args.len() + 2
                    ));
                    args.push(value_to_encode(value1));
                    args.push(value_to_encode(value2));
                }
                FilterCondition::In(values) => {
                    let placeholders: Vec<String> = (0..values.len())
                        .map(|i| format!("${}", args.len() + i + 1))
                        .collect();
                    conditions.push(format!("{} IN ({})", field, placeholders.join(", ")));
                    for value in values {
                        args.push(value_to_encode(value));
                    }
                }
                FilterCondition::Like(pattern) => {
                    conditions.push(format!("{} LIKE ${}", field, args.len() + 1));
                    args.push(Value::String(pattern.clone()));
                }
            }
        }

        let where_clause = if conditions.is_empty() {
            "TRUE".to_string()
        } else {
            conditions.join(" AND ")
        };

        (where_clause, args)
    }
}

fn value_to_encode(value: &Value) -> Value {
    value.clone()
}

// Helper functions to create FilterConditions
impl FilterCondition {
    pub fn eq<T: Into<Value>>(value: T) -> Self {
        FilterCondition::Eq(value.into())
    }

    pub fn ne<T: Into<Value>>(value: T) -> Self {
        FilterCondition::Ne(value.into())
    }

    pub fn gt<T: Into<Value>>(value: T) -> Self {
        FilterCondition::Gt(value.into())
    }

    pub fn lt<T: Into<Value>>(value: T) -> Self {
        FilterCondition::Lt(value.into())
    }

    pub fn gte<T: Into<Value>>(value: T) -> Self {
        FilterCondition::Gte(value.into())
    }

    pub fn lte<T: Into<Value>>(value: T) -> Self {
        FilterCondition::Lte(value.into())
    }

    pub fn between<T: Into<Value>>(value1: T, value2: T) -> Self {
        FilterCondition::Between(value1.into(), value2.into())
    }

    pub fn in_values<T: Into<Value>>(values: Vec<T>) -> Self {
        FilterCondition::In(values.into_iter().map(Into::into).collect())
    }

    pub fn like<T: Into<String>>(pattern: T) -> Self {
        FilterCondition::Like(pattern.into())
    }
}

// The rest of your code (Pagination and PaginatedRecord) remains unchanged

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Pagination {
    pub page: u32,
    pub per_page: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaginatedRecord<T> {
    pub items: Vec<T>,
    pub total_items: u64,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
}

impl<T> PaginatedRecord<T> {
    pub fn new(items: Vec<T>, total_items: u64, page: u32, per_page: u32) -> Self {
        let total_pages = ((total_items as f64) / (per_page as f64)).ceil() as u32;
        Self {
            items,
            total_items,
            page,
            per_page,
            total_pages,
        }
    }
}
