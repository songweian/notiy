use mysql::prelude::Queryable;

use crate::database::pool::POOL;
use crate::repository::sms_entity::SmsTemplateEntity;

pub fn query_by_id(id: i32) -> Option<SmsTemplateEntity> {
    let mut conn = POOL.lock().unwrap().get_conn().unwrap();
    let query = format!("SELECT * FROM t_m_sms_template WHERE id = {}", id);
    let result: Option<SmsTemplateEntity> = conn.query_first(query).unwrap();
    result
}