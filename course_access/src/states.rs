use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::types::chrono::NaiveDate;
use std::sync::Mutex;

#[allow(warnings, unused)]
pub struct AppState {
    pub health_status: String,
    pub visite_count: Mutex<i32>,
    pub db: PgPool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Course {
    pub id: i32,
    pub teacher_id: i32,
    pub course_name: String,
    pub date: Option<NaiveDate>,
}
//Record中一些成员为Option的原因：
//数据库中 这些 字段允许为 NULL，所以在 Rust 中必须用 Option<i32> 来表示它是可选的
#[derive(Deserialize, Serialize, Debug)]
pub struct Record {
    pub id: i32,
    pub teacher_id: Option<i32>,
    pub name: Option<String>,
    pub date: Option<NaiveDate>,
}

//执行sql后的返回状态结构体
#[derive(Deserialize, Serialize, Debug)]
pub struct ExecuteStatus {
    pub status: String,
    pub course_name: String,
}
