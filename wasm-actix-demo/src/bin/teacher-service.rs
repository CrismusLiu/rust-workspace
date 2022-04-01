extern crate actix_web;

use error::MyError;
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};

use std::sync::Mutex;

use actix_web::{web, App, HttpServer};

#[path = "../state.rs"]
mod state;

#[path = "../routers.rs"]
mod routers;

#[path = "../handlers/mod.rs"]
mod hanlders;

#[path = "../models/mod.rs"]
mod models;

#[path = "../error.rs"]
mod error;

#[path = "../dbaccess/mod.rs"]
mod db_access;

use routers::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let options = MySqlConnectOptions::new()
        .host("49.233.69.124")
        .port(3306)
        .username("root")
        .password("persion*#000Lsc")
        .database("ag_auth")
        .ssl_mode(sqlx::mysql::MySqlSslMode::Disabled)
        .to_owned();

    let pool = MySqlPoolOptions::new()
        .max_connections(20)
        .connect_timeout(std::time::Duration::from_secs(10))
        .connect_with(options)
        .await
        .unwrap();

    let share_data = web::Data::new(AppState {
        health_check_response: "ok".to_string(),
        visit_count: Mutex::new(0),
        db: pool,
    });

    let app = move || {
        App::new()
            .app_data(share_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                MyError::InvalidInput("Please provide valid Json input".to_string()).into()
            }))
            .configure(health_routes)
            .configure(course_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
