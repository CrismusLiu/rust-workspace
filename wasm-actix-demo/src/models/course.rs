use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::error::MyError;

// 不使用DeSerialize，Course只用来做查询结果，不用来新增或修改
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Course {
    pub id: i32,
    pub teacher_id: i32,
    pub name: String,
    pub time: Option<NaiveDateTime>,
}

// 新增使用的model
#[derive(Debug, Clone, Deserialize)]
pub struct CreateCourse {
    pub teacher_id: i32,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateCourse {
    pub id: i32,
    pub teacher_id: Option<i32>,
    pub name: Option<String>,
}

//POST请求时，json转化Model需要使用如下：

// impl From<web::Json<Course>> for Course {
//     fn from(course: web::Json<Course>) -> Self {
//         Course {
//             id: course.id,
//             teacher_id: course.teacher_id,
//             name: course.name.clone(),
//             time: course.time,
//         }
//     }
// }

impl TryFrom<web::Json<CreateCourse>> for CreateCourse {
    type Error = MyError;

    fn try_from(course: web::Json<CreateCourse>) -> Result<Self, Self::Error> {
        Ok(CreateCourse {
            teacher_id: course.teacher_id,
            name: course.name.clone(),
        })
    }
}

impl From<web::Json<UpdateCourse>> for UpdateCourse {
    fn from(course: web::Json<UpdateCourse>) -> Self {
        UpdateCourse {
            id: course.id,
            teacher_id: course.teacher_id,
            name: course.name.clone(),
        }
    }
}
