extern crate actix_web;

use std::sync::Mutex;
use actix_web::{App, get, HttpServer, web};
use lazy_static::lazy_static;
use syx_rpc_rust_core::RpcConsumerClientConfig;
use syx_rpc_rust_demo_api::User;

lazy_static! {
    static ref CONFIG: RpcConsumerClientConfig = RpcConsumerClientConfig::new(
        "localhost".to_string(), 8888
    );

    // 创建DemoServer全局客户端
    static ref DEMO_SERVER_INSTANCE: Mutex<syx_rpc_rust_demo_api::DemoServerRpc>
    = Mutex::new(syx_rpc_rust_demo_api::DemoServerRpc::new(&CONFIG));
}

#[get("/hello/{name}")]
async fn invoker_hello(name: web::Path<String>) -> actix_web::Result<web::Json<String>> {
    let rpc = DEMO_SERVER_INSTANCE.lock().unwrap();
    let res = rpc.hello(name.to_string()).await;
    Ok::<web::Json<String>, actix_web::Error>(web::Json(res))
}

#[get("/add/{a}/{b}")]
async fn invoker_add(path: web::Path<(i32, i32)>) -> actix_web::Result<web::Json<i32>> {
    let (a, b) = path.into_inner();
    let rpc = DEMO_SERVER_INSTANCE.lock().unwrap();
    let res = rpc.add(a, b).await;
    Ok::<web::Json<i32>, actix_web::Error>(web::Json(res))
}

#[get("/find_user/{id}")]
async fn invoker_find_user(path: web::Path<i32>) -> actix_web::Result<web::Json<User>> {
    let id = path.into_inner();
    let rpc = DEMO_SERVER_INSTANCE.lock().unwrap();
    let res = rpc.find_user(id).await;
    Ok::<web::Json<User>, actix_web::Error>(web::Json(res))
}

#[get("/get_age")]
async fn invoker_get_age() -> actix_web::Result<web::Json<i32>> {
    let user = User {
        name: "syx".to_string(),
        age: 18,
    };
    let rpc = DEMO_SERVER_INSTANCE.lock().unwrap();
    let res = rpc.get_age(user).await;
    Ok::<web::Json<i32>, actix_web::Error>(web::Json(res))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(invoker_hello)
            .service(invoker_add)
            .service(invoker_find_user)
            .service(invoker_get_age)
    })
        .bind(("localhost", 7777))?
        .run()
        .await
}
