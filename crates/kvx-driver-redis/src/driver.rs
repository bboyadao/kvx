use kvx_core::Driver;

use crate::{
    RedisClient,
    RedisOptions,
};


pub struct RedisDriver;


impl Driver for RedisDriver {

    type Client = RedisClient;

    type Options = RedisOptions;

}