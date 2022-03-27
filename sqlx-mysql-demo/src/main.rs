extern crate sqlx;
use async_std::prelude::*;
use sqlx::mysql::MySqlConnectOptions;
use sqlx::mysql::MySqlPool;
use sqlx::mysql::MySqlPoolOptions;
use structopt::StructOpt;

#[derive(sqlx::FromRow)]
struct Todo {
    description: String,
    id: Option<u64>,
    done: i32,
}

#[derive(StructOpt)]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt)]
enum Command {
    Add {
        description: String,
        done: i32,
    },
    Update {
        id: u64,
        description: String,
        done: i32,
    },
}

// #[tokio::main]
#[async_std::main]
#[paw::main]
async fn main(args: Args) {
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

    match args.cmd {
        Some(Command::Add { description, done }) => {
            add_todo(
                &pool,
                Todo {
                    description: description,
                    done: done,
                    id: None,
                },
            )
            .await
            .unwrap();
        }
        Some(Command::Update {
            id,
            description,
            done,
        }) => {
            update_todo(
                &pool,
                Todo {
                    description: description,
                    done: done,
                    id: Some(id),
                },
            )
            .await
            .unwrap();
        }
        None => {
            list_todos(&pool).await.unwrap();
        }
    }
}

async fn add_todo(pool: &MySqlPool, todo: Todo) -> std::io::Result<u64> {
    let tx = pool.begin().await.unwrap();

    let todo_id = sqlx::query(
        r#"
INSERT INTO todos ( description, done )
VALUES ( ?, ? )
        "#,
    )
    .bind(todo.description)
    .bind(todo.done)
    .execute(pool)
    .await
    .unwrap()
    .last_insert_id();

    let res = tx.commit().await.unwrap();

    std::io::Result::Ok(todo_id)
}

async fn update_todo(pool: &MySqlPool, todo: Todo) -> std::io::Result<bool> {
    let rows_affected = sqlx::query(
        r#"
UPDATE todos
SET done = ?
WHERE id = ?
        "#,
    )
    .bind(todo.done)
    .bind(todo.id.unwrap())
    .execute(pool)
    .await
    .unwrap()
    .last_insert_id();
    // .rows_affected();

    std::io::Result::Ok(rows_affected > 0)
}

async fn list_todos(pool: &MySqlPool) -> std::io::Result<()> {
    let sql = r#"
    SELECT id, description, done
    FROM todos
    ORDER BY id
            "#;

    let rows = sqlx::query_as::<_, Todo>(sql)
        .fetch_all(pool)
        .await
        .unwrap();

    for row in rows {
        println!("{} {} {}", row.id.unwrap(), row.description, row.done);
    }

    std::io::Result::Ok(())
}
