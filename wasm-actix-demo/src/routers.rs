use crate::hanlders::course::*;
use crate::hanlders::health;
use actix_web::web;

pub fn health_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("health", web::get().to(health::hanlders));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("/", web::post().to(new_course))
            .route("/update", web::post().to(update_course))
            .route("/{teacher_id}", web::get().to(get_course_by_teacher_id))
            .route("/{teacher_id}/{course_id}", web::get().to(get_courses)),
    );
}
