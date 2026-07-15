use kvx_types::Value;

#[derive(Debug)]
pub enum Response {

    Value(Value),

    Empty,

}