use crate::types::ErrorInfo;
use thiserror::Error as ThisError;

#[derive(ThisError, Clone, Debug)]
pub enum Error {
    //401
    #[error("Unauthorized")]
    Unauthorized,

    /// 403
    #[error("Forbidden")]
    Forbidden,

    /// 404
    #[error("Not Found")]
    NotFound,

    /// 422
    #[error("Unprocessable Entity:{0:?}")]
    UnprocessableEntity(ErrorInfo),

    ///500
    #[error("Internal Server Error")]
    InternalServerError,

    ///序列化/反序列化错误
    #[error("Deserialize Error")]
    DeserializeError,

    ///请求错误
    #[error("Http Request Error")]
    RequestError,

    ///业务错误
    #[error("{0:?}")]
    BusinessError(String),
}
