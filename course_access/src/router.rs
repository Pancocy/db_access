use super::handler::*;
use actix_web::{web};

#[allow(warnings, unused)]
pub fn health_config(config:&mut web::ServiceConfig){
    config.route("/health",web::get().to(health_check));
}

//获取所有课程
#[allow(warnings, unused)]
pub fn course_config(config:&mut web::ServiceConfig){
    config.route("/course",web::get().to(get_all_course));
    config.route("/getCourseByCid/{cid}",web::get().to(get_course_by_id));
    config.route("/getCourseByTid/{tid}",web::get().to(get_course_by_tid));
}
