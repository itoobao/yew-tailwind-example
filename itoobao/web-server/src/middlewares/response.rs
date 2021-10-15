use axum::{
    body::BoxBody,
    http::{header, HeaderValue, Response},
};
use std::convert::Infallible;

pub async fn customer_response(res: Response<BoxBody>) -> Result<Response<BoxBody>, Infallible> {
    tracing::debug!("/response");
    let mut r = res;
    r.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("*"),
    );
    r.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("Content-Type"),
    );

    Ok(r)
}
