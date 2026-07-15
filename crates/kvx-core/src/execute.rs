use crate::{
    KvxError,
    Request,
    Response,
};

pub trait Execute: Sized {
    type Output;

    /// Convert a typed command into the internal protocol.
    fn into_request(self) -> Request;

    /// Convert the protocol response back into the typed output.
    fn from_response(
        response: Response,
    ) -> Result<Self::Output, KvxError>;
}