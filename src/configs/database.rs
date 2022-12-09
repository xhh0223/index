use lazy_static::lazy_static;
use sqlx::{mysql, Error, MySql, Pool};
use std::str;
pub struct MysqlConfig {
    username: &'static str,
    password: &'static str,
    host: &'static str,
    port: usize,
    database: &'static str,
}

impl MysqlConfig {
    pub async fn getPool(&self, num: usize) -> Result<Pool<MySql>, Error> {
        Ok(mysql::MySqlPoolOptions::new()
            .max_connections(num as u32)
            .connect(&GITHUB_INDEX_MYSQL.to_string())
            .await?)
    }
}

impl ToString for MysqlConfig {
    fn to_string(&self) -> String {
        let MysqlConfig {
            username,
            password,
            host,
            port,
            database,
        } = *self;
        return format!("mysql://{username}:{password}@{host}:{port}/{database}");
    }
}


lazy_static! {
    pub static ref GITHUB_INDEX_MYSQL: MysqlConfig = MysqlConfig {
        username: "root",
        password: "04095413",
        host: "127.0.0.1",
        port: 3306,
        database: "github_index"
    };
}
