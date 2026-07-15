use crate::{Execute, Key, KvxError, Operation, Request, Response};

#[derive(Debug, Clone)]
pub struct Delete {
    key: Key,
}

impl Delete {
    pub fn new(key: impl Into<Key>) -> Self {
        Self { key: key.into() }
    }

    pub fn key(&self) -> &Key {
        &self.key
    }
}

impl Operation for Delete {
    type Output = bool;
}

impl Execute for Delete {
    type Output = bool;

    fn into_request(self) -> Request {
        Request::Delete(self)
    }

    fn from_response(
        response: Response,
    ) -> Result<Self::Output, KvxError> {
        match response {
            Response::Bool(value) => Ok(value),

            _ => Err(KvxError::Protocol(
                "expected Response::Bool".into(),
            )),
        }
    }
}