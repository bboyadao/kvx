use crate::Value;

#[derive(Debug)]
pub enum Response {
    Value(Option<Value>),

    Bool(bool),

    Empty,
}