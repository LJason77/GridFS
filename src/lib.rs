#![deny(clippy::pedantic)]
#![allow(clippy::non_ascii_literal)]

use mongodb::{Client, Database};
use rocket::{catchers, Build, Rocket};

mod catchers;
mod models;

/// 初始化 mongodb 数据库
pub async fn init_mongo(name: &str) -> Database {
    let database_url = dotenv::var("MONGO_URL").expect("MONGO_URL 变量未找到");
    let client = Client::with_uri_str(&database_url)
        .await
        .expect("连接数据库失败：");
    client.database(name)
}

pub async fn rocket() -> Rocket<Build> {
    let db = init_mongo("grid").await;

    rocket::build()
        .manage(db)
        // 错误处理
        .register("/", catchers![catchers::not_found])
}
