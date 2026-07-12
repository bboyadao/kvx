use kvx_core::Driver;
use crate::{
    client::RedisClient,
    RedisOptions,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct RedisDriver;

impl Driver for RedisDriver {
    type Client = RedisClient;
    type Options = RedisOptions;
}