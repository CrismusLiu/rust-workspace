use async_std::prelude::*;
// use futures::{future::UnsafeFutureObj, TryFutureExt};
use sqlx::{MySqlPool, mysql::{MySqlConnectOptions, MySqlPoolOptions, MySqlRow}, Row};
use std::{env, ops::Index};
use structopt::StructOpt;
use dotenv::dotenv;

#[derive(StructOpt)]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt)]
enum Command {
    Add { description: String },
    Done { id: u64 },
}

#[async_std::main]
#[paw::main]
async fn main(args: Args) -> anyhow::Result<()> {
    // dotenv().ok();
     // 获取环境变量中mysql连接地址
    //  let database_url = env::var("DATABASE_URL").expect("没有从环境变量中获得数据库地址");
     let options = MySqlConnectOptions::new()
        .host("49.233.69.124")
        .port(3306)
        .username("root")
        .password("persion*#000Lsc")
        .database("ag_auth")
        .to_owned();

    let pool = MySqlPoolOptions::new()
                .max_connections(20)
                .connect_timeout(std::time::Duration::from_secs(10))
                .connect_with(options).await?;
             
    // match args.cmd {
    //     Some(Command::Add { description }) => {
    //         println!("Adding new todo with description '{}'", &description);
    //         let todo_id = add_todo(&pool, description).await?;
    //         println!("Added new todo with id {}", todo_id);
    //     }
    //     Some(Command::Done { id }) => {
    //         println!("Marking todo {} as done", id);
    //         if complete_todo(&pool, id).await? {
    //             println!("Todo {} is marked as done", id);
    //         } else {
    //             println!("Invalid id {}", id);
    //         }
    //     }
    //     None => {
    //         println!("Printing list of all todos");
    //         list_todos(&pool).await?;
    //     }
    // }
    
    list_todos(&pool).await?;

    Ok(())
}

// async fn add_todo(pool: &MySqlPool, description: String) -> anyhow::Result<u64> {
//     // Insert the task, then obtain the ID of this row
//     let todo_id = sqlx::query!(
//         r#"
// INSERT INTO todos ( description )
// VALUES ( ? )
//         "#,
//         description
//     )
//     .execute(pool)
//     .await?
//     .last_insert_id();

//     Ok(todo_id)
// }

// async fn complete_todo(pool: &MySqlPool, id: u64) -> anyhow::Result<bool> {
//     let rows_affected = sqlx::query!(
//         r#"
// UPDATE todos
// SET done = TRUE
// WHERE id = ?
//         "#,
//         id
//     )
//     .execute(pool)
//     .await?
//     .rows_affected();

//     Ok(rows_affected > 0)
// }

async fn list_todos(pool: &MySqlPool) -> anyhow::Result<()> {
    let sql = 
            r#"
SELECT id, description, done
FROM todos
ORDER BY id
        "#
;
    // let recs = sqlx::query(sql).fetch_all(pool).await?;

//     let recs = sqlx::query(
//         r#"
// SELECT id, description, done
// FROM todos
// ORDER BY id
//         "#
//     )
//     .fetch_all(pool)
//     .await?;

    // NOTE: Booleans in MySQL are stored as `TINYINT(1)` / `i8`
    //       0 = false, non-0 = true
    // for rec in recs {
    //     println!(
    //         "- [{}] {}: {}",
    //         if rec.done != 0 { "x" } else { " " },
    //         rec.id,
    //         &rec.description,
    //     );
    // }

    // let rec = recs.index(0);

    // println!(
    //     "{:?}", rec.try_column("id")
    // );

    // let sql = "select id, username from t_user where id < ?";
    let rows = sqlx::query(sql).fetch_all(pool).await?;
    
    // println!("{:?}", rows);


    Ok(())
}
