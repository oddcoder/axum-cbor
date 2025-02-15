use axum_test::{TestRequest, TestResponse};
use serde::{de::DeserializeOwned, Serialize};

pub trait CborReq<T: Serialize> {
    #[must_use]
    fn cbor(self, cbor: &T) -> Self;
}

pub trait CborResp<T: DeserializeOwned> {
    fn cbor(self) -> T;
}

impl<T: Serialize> CborReq<T> for TestRequest {
    fn cbor(self, cbor: &T) -> Self {
        let mut body = Vec::new();
        ciborium::into_writer(&cbor, &mut body).unwrap();
        self.bytes(body.into()).content_type("application/cbor")
    }
}

impl<T: DeserializeOwned> CborResp<T> for TestResponse {
    fn cbor(self) -> T {
        let body = self.as_bytes().to_vec();
        ciborium::from_reader(body.as_slice()).unwrap()
    }
}
