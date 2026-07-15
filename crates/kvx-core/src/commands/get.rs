use crate::{Key, Operation, Value};

#[derive(Debug, Clone)]
pub struct Get {
    key: Key,
}

impl Get {
    pub fn new(key: impl Into<Key>) -> Self {
        Self { key: key.into() }
    }

    pub fn key(&self) -> &Key {
        &self.key
    }
}

impl Operation for Get {
    type Output = Option<Value>;
}

use crate::{
    Execute,
    KvxError,
    Request,
    Response,
};

impl Execute for Get {
    type Output = Option<Value>;

    fn into_request(self) -> Request {
        Request::Get(self)
    }

    fn from_response(
        response: Response,
    ) -> Result<Self::Output, KvxError> {
        match response {
            Response::Value(value) => Ok(value),

            _ => Err(KvxError::Protocol(
                "expected Response::Value".into(),
            )),
        }
    }
}