extern crate actix_web;
extern crate syx_rpc_rust_demo_api;

use actix_web::{App, HttpServer, post, web};

use syx_rpc_rust_core::{add_service, invoke_service, RpcRequest};
use syx_rpc_rust_core::RpcResponse;

use crate::provider::DemoServiceImpl;

mod provider;

#[post("/")]
async fn invoker(req: web::Json<RpcRequest>) -> actix_web::Result<web::Json<RpcResponse<String>>> {
    println!("provider接收到的请求: {:?}", req.0);
    let response = invoke_service(&req).await;
    Ok::<web::Json<RpcResponse<String>>, actix_web::Error>(web::Json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 注册服务
    let demo = DemoServiceImpl {};
    let service = Box::new(demo);
    add_service("DemoServiceImpl", service);

    HttpServer::new(move || {
        App::new()
            .service(invoker)
    })
        .bind(("localhost", 8888))?
        .run()
        .await
}