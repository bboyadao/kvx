use kvx_types::{
    Delete,
    Get,
    Set,
};

#[derive(Debug)]
pub enum Request {

    Get(Get),

    Set(Set),

    Delete(Delete),

}