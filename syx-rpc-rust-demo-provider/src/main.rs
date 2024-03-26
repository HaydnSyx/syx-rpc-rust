extern crate actix_web;
extern crate syx_rpc_rust_demo_api;

use actix_web::{App, HttpServer, post, web};

use syx_rpc_rust_core::{add_service, invoke_service, RpcRequest};
use syx_rpc_rust_core::RpcResponse;

use crate::provider::DemoServiceImpl;

mod provider;

#[post("/")]
async fn invoker(req: web::Json<RpcRequest>) -> actix_web::Result<web::Json<RpcResponse<String>>> {
    let service = &req.service;
    let method_sign = &req.method_sign;
    let param = &req.args;
    // 根据服务名称获取到对应的服务
    let result = invoke_service(service, |service| service.invoke(method_sign, param)).unwrap();

    // let server = get_service(service).unwrap();
    // let result = server.invoke(method_sign, param);

    let response = RpcResponse {
        code: 0,
        msg: String::from("success"),
        data: result,
    };
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
        .bind(("localhost", 8080))?
        .run()
        .await
}