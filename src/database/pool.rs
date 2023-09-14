use mysql::{Pool, OptsBuilder};
use lazy_static::lazy_static;
use std::sync::Mutex;
// const URL: &str = "mysql://7ce4224f1e584c7d9e73e36910f87dfain01.internal.cn-north-4.mysql.rds.myhuaweicloud.com:3306/g7n_dispatch_dev?useUnicode=true&characterEncoding=utf-8&useSSL=true&autoReconnect=true&zeroDateTimeBehavior=CONVERT_TO_NULL";
const URL: &str = "mysql://7ce4224f1e584c7d9e73e36910f87dfain01.internal.cn-north-4.mysql.rds.myhuaweicloud.com:3306/g7n_dispatch_dev";
const USER: &str = "qce_rw";
const password: &str = "qce_rw@2021";

lazy_static! {
    pub static ref POOL: Mutex<Pool> = Mutex::new({

        let opts = OptsBuilder::new()
            .ip_or_hostname(Some("7ce4224f1e584c7d9e73e36910f87dfain01.internal.cn-north-4.mysql.rds.myhuaweicloud.com"))
            .user(Some(USER))
            .pass(Some(password))
            .db_name(Some("g7n_dispatch_dev"));

        // Pool::new(OptsBuilder::from_opts(opts)).unwrap()
        Pool::new(opts).unwrap()
    });
}
