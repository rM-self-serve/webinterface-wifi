use hyper::{Body, Response, StatusCode};

pub fn not_auth_resp(auth_content: String) -> Response<Body> {
    Response::builder()
        .header("WWW-Authenticate", auth_content)
        .status(StatusCode::UNAUTHORIZED)
        .body(Body::default())
        .unwrap()
}

pub fn unreq_auth_header() -> Response<Body> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::from(
            "Browser is sending unrecognized authentication header, try a different browser.",
        ))
        .unwrap()
}
