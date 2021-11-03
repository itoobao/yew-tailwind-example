use serde::{Deserialize, Serialize};

/// 登录-参数 结构
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

///登录成功后，返回的用户信息 结构
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub username: String,
    pub image: Option<String>,
    pub token: String,
}
