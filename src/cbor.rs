use crate::cbor_rejecton::CborRejection;
use crate::errors::{FailedToParseCbor, MissingCBorContentType};
use axum::{
    body::Bytes,
    extract::{FromRequest, Request},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use serde::{de::DeserializeOwned, Serialize};

#[must_use]
pub struct Cbor<T>(pub T);

fn is_valid_cbor_content_type(headers: &HeaderMap) -> bool {
    let Some(content_type) = headers.get(header::CONTENT_TYPE) else {
        return false;
    };

    let Ok(content_type) = content_type.to_str() else {
        return false;
    };

    let Ok(mime) = content_type.parse::<mime::Mime>() else {
        return false;
    };

    let is_cbor_content_type = mime.type_() == "application"
        && (mime.subtype() == "cbor" || mime.suffix().is_some_and(|name| name == "cbor"));

    is_cbor_content_type
}

impl<S, T> FromRequest<S> for Cbor<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = CborRejection;
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        if !is_valid_cbor_content_type(req.headers()) {
            return Err(MissingCBorContentType.into());
        }
        let bytes = Bytes::from_request(req, state).await?;
        let value = ciborium::from_reader(&bytes as &[u8]);
        value.map(Cbor).map_err(|_| FailedToParseCbor.into())
    }
}

impl<T> IntoResponse for Cbor<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let mut buf = Vec::new();
        match ciborium::into_writer(&self.0, &mut buf) {
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                )],
                "Failed to serialize".to_string(),
            )
                .into_response(),
            Ok(()) => (
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("application/cbor"),
                )],
                buf,
            )
                .into_response(),
        }
    }
}
