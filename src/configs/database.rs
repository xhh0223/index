use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Hash,PartialEq, Eq)]
pub enum DataBase<'a> {
    Mysql(&'a str),
}

lazy_static! {
    pub static ref DATA_BASE_CONNECTION_INFO: HashMap<DataBase<'static>, &'static str> = {
        let mut data_base_connection_info_map = HashMap::new();

        data_base_connection_info_map.insert(
            DataBase::Mysql("xhh_github_index"),
            "mysql://root:root@127.0.0.1:3306/xhh_github_index",
        );

        data_base_connection_info_map
    };
}
