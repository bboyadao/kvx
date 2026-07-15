use redis::{
    aio::MultiplexedConnection,
    Client,
};
use kvx_core::{Delete, Get, KvxError, Set, Value};

use redis::AsyncCommands;

use crate::RedisOptions;




pub struct RedisClient {

    connection: MultiplexedConnection,

}



impl RedisClient {

    pub async fn connect(
        options: RedisOptions,
    ) -> redis::RedisResult<Self> {

        let client =
            Client::open(
                options.url
            )?;

        let connection =
            client
                .get_multiplexed_async_connection()
                .await?;

        Ok(Self {
            connection
        })

    }

    pub(crate) fn connection(
        &self
    ) -> MultiplexedConnection {

        self.connection.clone()

    }

    pub(crate) async fn get(
        &self,
        operation: Get,
    ) -> Result<Option<Value>, KvxError> {

        let mut conn =
            self.connection();

        let value: Option<Vec<u8>> =
            conn
                .get(
                    operation.key().as_bytes()
                )
                .await
                .map_err(|e|
                    KvxError::Backend(
                        e.to_string()
                    )
                )?;

        Ok(
            value.map(Value::from)
        )

    }

    pub(crate) async fn set(
    &self,
    operation: Set,
) -> Result<(), KvxError> {

    let mut conn =
        self.connection();

    let _: () =
        conn
            .set(
                operation.key().as_bytes(),
                operation.value().as_bytes(),
            )
            .await
            .map_err(|e|
                KvxError::Backend(
                    e.to_string()
                )
            )?;

    Ok(())

}

    pub(crate) async fn delete(
    &self,
    operation: Delete,
) -> Result<bool, KvxError> {

    let mut conn =
        self.connection();

    let deleted: usize =
        conn
            .del(
                operation.key().as_bytes()
            )
            .await
            .map_err(|e|
                KvxError::Backend(
                    e.to_string()
                )
            )?;

    Ok(deleted > 0)

}
}