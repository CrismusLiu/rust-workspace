extern crate sqlx;
use crate::state::AppState;
use crate::{error::MyError, models::course::*};
use actix_web::{web, HttpResponse};
use std::result::Result;

use crate::db_access::course;

pub async fn new_course(
    course: web::Json<CreateCourse>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    println!("new_course");

    course::new_course(&app_state.db, &course.try_into()?)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn update_course(
    course: web::Json<UpdateCourse>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    println!("update_course");
    // let course_param = UpdateCourse::try_from(course)?;
    course::update_course(&app_state.db, &course.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_course_by_teacher_id(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    println!("{}", teacher_id);

    course::get_course_by_teacher_id(&app_state.db, teacher_id)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_courses(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();

    course::get_courses_by_params(&app_state.db, teacher_id, course_id)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))
}

// #[cfg(test)]
// mod test {

//     use crate::models::course::*;
//     use crate::state::AppState;
//     use actix_web::{http::StatusCode, web};
//     use std::sync::Mutex;

//     use crate::hanlders::course::{get_course_by_teacher_id, get_courses, new_course};

// #[actix_rt::test]
// async fn create_course_hanlder() {
//     let course = web::Json(Course {
//         id: None,
//         teacher_id: 1,
//         name: "tt".into(),
//         time: None,
//     });

//     let app_state = web::Data::new(AppState {
//         health_check_response: "".to_string(),
//         visit_count: Mutex::new(0),
//         courses: Mutex::new(vec![]),
//     });

//     let res = new_course(course, app_state).await;
//     assert_eq!(res.status(), StatusCode::OK);
// }

// #[actix_rt::test]
// async fn get_course_by_teacher_id_handler() {
//     let app_state = web::Data::new(AppState {
//         health_check_response: "".to_string(),
//         visit_count: Mutex::new(0),
//         courses: Mutex::new(vec![]),
//     });

//     let params = web::Path::from((1));
//     let resp = get_course_by_teacher_id(app_state, params).await;

//     assert_eq!(resp.status(), StatusCode::OK);
// }

// #[actix_rt::test]
// async fn get_courses_hanlder() {
//     let app_state = web::Data::new(AppState {
//         health_check_response: "".to_string(),
//         visit_count: Mutex::new(0),
//         courses: Mutex::new(vec![]),
//     });

//     let params = web::Path::from((1, 1));
//     let resp = get_courses(app_state, params).await;

//     assert_eq!(resp.status(), StatusCode::OK);
// }
// }
