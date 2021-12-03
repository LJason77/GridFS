use rocket::{
    catch,
    http::Status,
    response::status::Custom,
    serde::json::{serde_json::json, Value},
    Request,
};

use crate::models::Results;

fn info<'a>(status: Status, req: &Request, info: &'a str) -> Results<'a, &'a str> {
    println!(
        "{}：\nIP：{:?}\nurl：{:?}\n",
        status,
        &req.client_ip(),
        &req.uri().path().as_str()
    );
    Results {
        code: Some(status.code),
        data: None,
        message: Some(info),
    }
}

#[catch(404)]
pub fn not_found(status: Status, req: &Request) -> Custom<Value> {
    let result = info(status, req, "路由未找到");
    Custom(status, json!(result))
}
