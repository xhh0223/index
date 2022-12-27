use std::sync::Mutex;

use lazy_static::lazy_static;
use rbatis::Rbatis;

lazy_static! {
    pub static ref MYSQL_RBATIS: Mutex<Rbatis> = Mutex::new(Rbatis::new());
}
