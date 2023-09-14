use crate::repository::sms_template;

pub fn query() {
    sms_template::query_by_id(10);
}