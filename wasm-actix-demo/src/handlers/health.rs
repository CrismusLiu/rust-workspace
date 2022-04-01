extern crate sqlx;
use crate::state::AppState;
use actix_web::{web, HttpResponse, Responder};

pub async fn hanlders(app_state: web::Data<AppState>) -> impl Responder {
    let health_check_res = &app_state.health_check_response;
    let mut count = app_state.visit_count.lock().unwrap();

    let response = format!("{} {} times", health_check_res, count);

    *count += 1;

    HttpResponse::Ok().json(&response)
}
