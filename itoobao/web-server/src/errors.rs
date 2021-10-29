use thiserror::Error as ThisError;
#[derive(ThisError, Clone, Debug)]
pub enum CustomError {
    #[error("数据库连接失败:{0:?}")]
    DbConnectionError(String),
}
