#[macro_use]
extern crate rbatis;
use std::io::Error;

use rbdc_mysql::driver::MysqlDriver;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Img {
    id: usize,
    name: String,
}
impl_select!(Img{select_all()=>""},"image");
pub mod configs;
#[tokio::main]
async fn main() -> Result<(), rbatis::Error> {
    let mut result = configs::database::MYSQL_RBATIS.lock().unwrap();
    result.init(
        MysqlDriver {},
        "mysql://root:04095413@127.0.0.1:3306/github_index",
    );
    Img::select_all(&mut *result).await?;

    Ok(())
}
