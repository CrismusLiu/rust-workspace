use crate::{error::MyError, models::course::*};
use sqlx::mysql::MySqlPool;
use std::result::Result;

pub async fn new_course(db: &MySqlPool, course: &CreateCourse) -> Result<u64, MyError> {
    let insert_id = sqlx::query(r#"INSERT INTO course (teacher_id, name) values (?,?) "#)
        .bind(course.teacher_id)
        .bind(course.name.clone())
        .execute(db)
        .await?
        .last_insert_id();

    let json = format!("course added success, id {}", insert_id);
    println!("{}", json);

    if insert_id > 0 {
        Ok(insert_id)
    } else {
        Err(MyError::NotFound("course new fail".into()))
    }
}

pub async fn _new_course_model(db: &MySqlPool, course: &CreateCourse) -> Result<Course, MyError> {
    let course =
        sqlx::query_as::<_, Course>(r#"INSERT INTO course (teacher_id, name) values (?,?) "#)
            .bind(course.teacher_id)
            .bind(course.name.clone())
            .fetch_one(db)
            .await?;

    let json = format!("course added success, model {:?}", course);
    println!("{}", json);

    Ok(course)
}

// 两种错误处理方式
pub async fn update_course(db: &MySqlPool, course: &UpdateCourse) -> Result<Course, MyError> {
    let current_course_row = sqlx::query_as::<_, Course>(r#"select * from course where id = ? "#)
        .bind(course.id)
        .fetch_one(db)
        .await
        .map_err(|_err| MyError::NotFound("course update> query course not found by id".into()))?;

    let json = format!("current course {:?}", current_course_row);
    println!("{}", json);

    let teacher_id_up = if let Some(teacher_id) = course.teacher_id {
        teacher_id
    } else {
        current_course_row.teacher_id
    };

    let name_up = if let Some(name) = &course.name {
        name.into()
    } else {
        current_course_row.name
    };

    let course_updated =
        sqlx::query_as::<_, Course>(r#"update course set teacher_id = ?, name = ? where id = ?"#)
            .bind(teacher_id_up)
            .bind(name_up)
            .bind(course.id)
            .fetch_one(db)
            .await;

    if let Ok(course) = course_updated {
        Ok(course)
    } else {
        Err(MyError::NotFound("course not found".into()))
    }
}

pub async fn get_courses_by_params(
    db: &MySqlPool,
    teacher_id: i32,
    course_id: i32,
) -> Result<Vec<Course>, MyError> {
    let sql = r#"select id, teacher_id, name, time FROM course where id = ? and teacher_id = ?"#;
    let selected_courses = sqlx::query_as::<_, Course>(sql)
        .bind(course_id)
        .bind(teacher_id)
        .fetch_all(db)
        .await?;

    match selected_courses.len() {
        0 => Err(MyError::NotFound("course not fond".into())),
        _ => Ok(selected_courses),
    }
}

pub async fn get_course_by_teacher_id(
    db: &MySqlPool,
    teacher_id: i32,
) -> Result<Vec<Course>, MyError> {
    let sql = r#"SELECT id, teacher_id, name, time FROM course where id = ? "#;

    let rows = sqlx::query_as::<_, Course>(sql)
        .bind(&teacher_id)
        .fetch_all(db)
        .await?;

    for row in &rows {
        println!(
            "{} {} {} {}",
            row.id,
            row.teacher_id,
            row.name,
            row.time.unwrap()
        );
    }

    match rows.len() {
        0 => Err(MyError::NotFound("course not fond".into())),
        _ => Ok(rows),
    }
}
