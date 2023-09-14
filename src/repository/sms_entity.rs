use mysql::{FromRowError, Row};
use mysql::prelude::FromRow;
use serde_derive::Serialize;

// create table sms_template
// (
// id                  bigint auto_increment comment '主键'
// primary key,
// app_id              varchar(200) default '' not null,
// template_code       char(200)               not null comment '模版code',
// channel_code        char(200)               not null comment '模版code',
// template_content    text                    not null comment '模版内容',
// audit_status        char(200)               not null comment '审核状态',
// template_type       varchar(20)             null,
// cloud_template_code char(200)               not null comment '云端模版code',
// cloud_sign_code     varchar(100) default '' not null,
// sign_name           varchar(255)            not null comment '签名',
// create_time         datetime                null comment '创建时间',
// update_time         datetime                null comment '修改时间'
// )

#[derive(Debug,Serialize)]
pub struct SmsTemplateEntity {
    id: i64,
    app_id: String,
    template_code: String,
    channel_code: String,
    template_content: String,
    audit_status: String,
    template_type: String,
    cloud_template_code: String,
    cloud_sign_code: String,
    sign_name: String,
    create_time: String,
    update_time: String,
}

impl SmsTemplateEntity {
    pub fn default() -> SmsTemplateEntity {
        todo!()
    }
}

impl FromRow for SmsTemplateEntity {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> where Self: Sized {
        let (id, app_id, template_code, channel_code, template_content, audit_status, template_type, cloud_template_code, cloud_sign_code, sign_name, create_time, update_time) = mysql::from_row_opt(row)?;
        Ok(Self {
            id,
            app_id,
            template_code,
            channel_code,
            template_content,
            audit_status,
            template_type,
            cloud_template_code,
            cloud_sign_code,
            sign_name,
            create_time,
            update_time,
        })
    }
}