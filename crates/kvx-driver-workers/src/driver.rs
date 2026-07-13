use kvx_core::Driver;

use crate::{
    WorkersClient,
    WorkersOptions,
};

pub struct WorkersDriver;

impl Driver for WorkersDriver {
    type Client = WorkersClient;
    type Options = WorkersOptions;
}