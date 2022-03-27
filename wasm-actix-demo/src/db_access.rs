use crate::{error::MyError, models::Course};
use sqlx::mysql::MySqlPool;
use std::result::Result;

pub async fn new_course(db: &MySqlPool, course: &Course) -> Result<u64, MyError> {
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
            row.id.unwrap(),
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
