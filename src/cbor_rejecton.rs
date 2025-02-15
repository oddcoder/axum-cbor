use crate::errors::{FailedToParseCbor, MissingCBorContentType};
use axum::{
    extract::rejection::BytesRejection,
    response::{IntoResponse, Response},
};

/// Top-level Errors
#[derive(Debug)]
pub enum CborRejection {
    MissingCBorContentType(MissingCBorContentType),
    BytesRejection(BytesRejection),
    FailedToParseCbor(FailedToParseCbor),
}

impl IntoResponse for CborRejection {
    fn into_response(self) -> Response {
        match self {
            CborRejection::MissingCBorContentType(c) => c.into_response(),
            CborRejection::BytesRejection(b) => b.into_response(),
            CborRejection::FailedToParseCbor(b) => b.into_response(),
        }
    }
}

impl From<BytesRejection> for CborRejection {
    fn from(x: BytesRejection) -> Self {
        CborRejection::BytesRejection(x)
    }
}
