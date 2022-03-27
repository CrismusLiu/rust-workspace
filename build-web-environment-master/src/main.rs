use actix_web::{get, App, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions, MySqlConnectOptions};
use std::env;

#[get("/hello")]
async fn index() -> impl Responder {
    format!("你好, 世界!")
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // 准备环境变量
    // dotenv().ok();

    // 获取环境变量中mysql连接地址
    // let database_url = env::var("DATABASE_URL").expect("没有从环境变量中获得数据库地址");

    // 创建Mysql连接池实例
    // let pool = MySqlPool::connect(&database_url).await.unwrap();

    // let pool = MySqlPoolOptions::new()
    //     .min_connections(2)
    //     .max_connections(2)
    //     .test_before_acquire(false)
    //     .connect(&database_url)
    //     .await?;



    let options = MySqlConnectOptions::new()
        .host("49.233.69.124:3306")
        .username("root")
        .password("persion*#000Lsc")
        .database("ag_auth")
        .to_owned();

    let pool = MySqlPoolOptions::new()
                .max_connections(20)
                .connect_timeout(std::time::Duration::from_secs(1))
                .connect_with(options).await?;
    // 启动Web应用程序, 共享Mysql连接池实例
    HttpServer::new(move || App::new().data(pool.clone()).service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await?;

    Ok(())
}
