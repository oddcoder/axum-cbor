use crate::{Cbor, CborReq, CborResp};
use axum::{http::StatusCode, routing::post, Router};
use axum_test::TestServer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    text: String,
    number: u64,
}

#[tokio::test]
async fn deserialize_body() {
    let app = Router::new().route("/", post(|input: Cbor<Data>| async { input.0.text }));
    let client = TestServer::new(app).unwrap();
    let data = Data {
        text: String::from("Hello, world!"),
        number: 7,
    };

    let res = client.post("/").cbor(&data).await;
    let body = res.text();
    assert_eq!(body, "Hello, world!");
}

#[tokio::test]
async fn consume_body_to_cbor_requires_cbor_content_type() {
    let app = Router::new().route("/", post(|input: Cbor<Data>| async { input.0.text }));
    let client = TestServer::new(app).unwrap();
    let data = Data {
        text: String::from("Hello, world!"),
        number: 7,
    };
    let mut raw_data = Vec::new();
    ciborium::into_writer(&data, &mut raw_data).unwrap();
    let res = client.post("/").bytes(r#"{ "foo": "bar" }"#.into()).await;

    let status = res.status_code();

    assert_eq!(status, StatusCode::UNSUPPORTED_MEDIA_TYPE);
}

#[tokio::test]
async fn cbor_content_types() {
    async fn valid_cbor_content_type(content_type: &str) -> bool {
        let app = Router::new().route("/", post(|_: Cbor<String>| async {}));
        let mut raw_data = Vec::new();
        ciborium::into_writer("Hello, world!", &mut raw_data).unwrap();

        let res = TestServer::new(app)
            .unwrap()
            .post("/")
            .content_type(content_type)
            .bytes(raw_data.into())
            .await;
        res.status_code() == StatusCode::OK
    }
    assert!(valid_cbor_content_type("application/cbor").await);
    assert!(valid_cbor_content_type("application/cbor; charset=utf-8").await);
    assert!(valid_cbor_content_type("application/cbor;charset=utf-8").await);
    assert!(valid_cbor_content_type("application/cloudevents+cbor").await);
    assert!(!valid_cbor_content_type("text/cbor").await);
    assert!(!valid_cbor_content_type("foobar").await);
}

#[tokio::test]
async fn invalid_cbor_syntax() {
    let app = Router::new().route("/", post(|_: Cbor<String>| async {}));
    let client = TestServer::new(app).unwrap();
    let res = client
        .post("/")
        .bytes("Hello, world!".into())
        .content_type("application/cbor")
        .await;
    assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
}

#[derive(Debug, Serialize, Deserialize)]
struct DifferentData {
    number1: u64,
    number2: u64,
}
#[tokio::test]
async fn invalid_cbor_data() {
    let app = Router::new().route("/", post(|_: Cbor<DifferentData>| async {}));
    let client = TestServer::new(app).unwrap();
    let data = Data {
        text: String::from("Hello, world!"),
        number: 7,
    };
    let res = client.post("/").cbor(&data).await;
    assert_eq!(res.status_code(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn send_cbor_content_header() {
    let app = Router::new().route("/", post(|| async { Cbor("Hello, world!".to_owned()) }));
    let client = TestServer::new(app).unwrap();
    let res = client.post("/").await;
    let hdr = res.headers();
    let content_hdr = hdr.get("content-type");
    assert!(content_hdr.is_some());
    assert_eq!(content_hdr.unwrap().to_str().unwrap(), "application/cbor");
}

#[tokio::test]
async fn response_cbor_data() {
    let app = Router::new().route(
        "/",
        post(|| async {
            Cbor(Data {
                text: "Hello, world!".to_owned(),
                number: 7,
            })
        }),
    );
    let client = TestServer::new(app).unwrap();
    let res = client.post("/").await;
    let data: Data = res.cbor();
    assert_eq!(data.number, 7);
    assert_eq!(data.text, "Hello, world!");
}

#[derive(Debug, Serialize, Deserialize)]
enum Unserializable {
    #[serde(skip)]
    SkipMe,
}

#[tokio::test]
async fn response_cbor_error() {
    let app = Router::new().route("/", post(|| async { Cbor(Unserializable::SkipMe) }));
    let client = TestServer::new(app).unwrap();
    let res = client.post("/").await;
    assert_eq!(res.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(res.text(), "Failed to serialize");
}
