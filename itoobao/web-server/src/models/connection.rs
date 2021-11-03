use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

use crate::errors::CustomError;

pub struct DbPool;
impl DbPool {
    pub async fn init_db() -> Result<MySqlPool, CustomError> {
        let conf = MysqlConf {
            username: String::from("root"),
            password: String::from("123456"),
            host: String::from("127.0.0.1"),
            port: 3306,
            database: String::from("rust-itoobao"),
        };
        let uri = format!(
            "msyql://{}:{}@{}:{}/{}",
            conf.username, conf.password, conf.host, conf.port, conf.database
        );

        let pool = MySqlPoolOptions::new()
            .min_connections(3)
            .max_connections(10)
            .connect_timeout(std::time::Duration::from_secs(3))
            .connect(&uri);
        let pool = match pool.await {
            Ok(p) => p,
            Err(e) => return Err(CustomError::DbConnectionError(e.to_string())),
        };
        Ok(pool)
    }
}

struct MysqlConf {
    username: String,
    password: String,
    host: String,
    port: u32,
    database: String,
}
