use std::sync::Mutex;
use sqlx::PgPool;
use sqlx::types::chrono::NaiveDate;
use serde;

#[allow(warnings, unused)]
pub struct AppState{
    pub health_status:String,
    pub visite_count:Mutex<i32>,
    pub db:PgPool
}

#[derive(serde::Serialize)]
pub struct Course{
    pub id:i32,
    pub teacher_id:i32,
    pub course_name:String,
    pub date:Option<NaiveDate>
}
#[derive(serde::Serialize)]
pub struct Record{
    pub  id:i32,
    pub  teacher_id:Option<i32>,
    pub  name: Option<String>,
    pub  date:Option<NaiveDate>
}
