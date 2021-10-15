use async_trait::async_trait;
use axum::{
    extract::FromRequest,
    http::{Method, StatusCode},
};

pub struct SupportRequestMethod;

#[async_trait]
impl<B> FromRequest<B> for SupportRequestMethod
where
    B: Send,
{
    type Rejection = StatusCode;
    async fn from_request(
        req: &mut axum::extract::RequestParts<B>,
    ) -> Result<Self, Self::Rejection> {
        tracing::debug!("check support request method ");

        if req.method() == Method::OPTIONS {
            tracing::debug!("method OPTIONS");
            return Err(StatusCode::OK);
        }
        Ok(Self)
    }
}
