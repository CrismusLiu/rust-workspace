use super::hanlders::*;
use actix_web::web;


pub fn health_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("health", web::get().to(hanlders));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("/courses")
            .route("/", web::post().to(new_course))
            .route("/{teacher_id}", web::get().to(get_course_by_teacher_id))
            .route("/{teacher_id}/{course_id}", web::get().to(get_courses))
    );
}

