use crate::{Execute, Key, KvxError, Operation, Request, Response, Value};

#[derive(Debug, Clone)]
pub struct Set {
    key: Key,
    value: Value,
}

impl Set {
    pub fn new(
        key: impl Into<Key>,
        value: impl Into<Value>,
    ) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }

    pub fn key(&self) -> &Key {
        &self.key
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}

impl Operation for Set {
    type Output = ();
}


impl Execute for Set {
    type Output = ();

    fn into_request(self) -> Request {
        Request::Set(self)
    }

    fn from_response(
        response: Response,
    ) -> Result<Self::Output, KvxError> {
        match response {
            Response::Empty => Ok(()),

            _ => Err(KvxError::Protocol(
                "expected Response::Empty".into(),
            )),
        }
    }
}