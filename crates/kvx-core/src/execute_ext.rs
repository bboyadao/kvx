use crate::{
    Execute,
};

use kvx_types::{
    Delete,
    Get,
    Set,
};



pub trait ExecuteExt {

}



impl<T> ExecuteExt for T
where
    T: Execute<Set>
        + Execute<Get>
        + Execute<Delete>,
{

}