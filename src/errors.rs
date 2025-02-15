use crate::CborRejection;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug, Default)]
pub struct FailedToParseCbor;

impl From<FailedToParseCbor> for CborRejection {
    fn from(x: FailedToParseCbor) -> Self {
        CborRejection::FailedToParseCbor(x)
    }
}

impl IntoResponse for FailedToParseCbor {
    fn into_response(self) -> Response {
        (Self::status(), Self::body_text()).into_response()
    }
}

impl FailedToParseCbor {
    /// Get the response body text used for this rejection.
    #[must_use]
    pub fn body_text() -> &'static str {
        "Invalid Request"
    }

    /// Get the status code used for this rejection.
    #[must_use]
    pub fn status() -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}

#[derive(Debug, Default)]
pub struct MissingCBorContentType;

impl From<MissingCBorContentType> for CborRejection {
    fn from(x: MissingCBorContentType) -> Self {
        CborRejection::MissingCBorContentType(x)
    }
}

impl IntoResponse for MissingCBorContentType {
    fn into_response(self) -> Response {
        (Self::status(), Self::body_text()).into_response()
    }
}

impl MissingCBorContentType {
    /// Get the response body text used for this rejection.
    #[must_use]
    pub fn body_text() -> &'static str {
        "Expected request with `content-type: application/cbor`"
    }

    /// Get the status code used for this rejection.
    #[must_use]
    pub fn status() -> StatusCode {
        StatusCode::UNSUPPORTED_MEDIA_TYPE
    }
}
