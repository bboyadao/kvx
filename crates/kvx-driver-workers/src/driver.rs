use kvx_core::BackendFactory;

use crate::{
    WorkersClient,
    WorkersOptions,
};


pub struct WorkersBackendFactory;


impl BackendFactory for WorkersBackendFactory {

    type Client = WorkersClient;

    type Options = WorkersOptions;

}