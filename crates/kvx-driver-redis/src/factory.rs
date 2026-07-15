use kvx_core::{BackendFactory};

use crate::{
    RedisClient,
    RedisOptions,
};


pub struct RedisBackendFactory;


impl BackendFactory for RedisBackendFactory {

    type Client = RedisClient;

    type Options = RedisOptions;

}