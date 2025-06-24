use super::handler::*;
use actix_web::web;

#[allow(warnings, unused)]
pub fn health_config(config: &mut web::ServiceConfig) {
    config.route("/health", web::get().to(health_check));
}

//获取所有课程
#[allow(warnings, unused)]
pub fn course_config(config: &mut web::ServiceConfig) {
    config.route("/courses", web::get().to(get_all_course));
    config.route("/courses/{id}", web::get().to(get_course_by_id));
    config.route(
        "/teachers/{teacher_id}/courses",
        web::get().to(get_course_by_tid),
    );
    config.route(
        "/teachers/{teacher_id}/courses/{course_id}",
        web::get().to(get_course_by_t_cid),
    );
    config.route("/addCourse", web::post().to(post_course_into_table));
    config.route(
        "/deleteCourse",
        web::delete().to(delete_course_by_cid),
    );
}
