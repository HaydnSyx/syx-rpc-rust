extern crate actix_web;
extern crate syx_rpc_rust_demo_api;

use std::collections::HashMap;
use std::sync::Arc;
use actix_web::{App, HttpServer, post, web};

use syx_rpc_rust_core::{get_service, register, RpcRequest, RpcService};
use syx_rpc_rust_core::RpcResponse;
use syx_rpc_rust_demo_api::DemoServer;

use crate::provider::DemoServiceImpl;

mod provider;

#[post("/")]
async fn invoker(req: web::Json<RpcRequest>) -> actix_web::Result<web::Json<RpcResponse<String>>> {
    let service = &req.service;
    // let method_sign = &req.method_sign;
    let param = &req.args;
    // 根据服务名称获取到对应的服务
    let server = get_service(service);


    // 执行调用
    let result = server.invoke(param);

    let response = RpcResponse {
        code: 0,
        msg: String::from("success"),
        data: result,
    };
    Ok::<web::Json<RpcResponse<String>>, actix_web::Error>(web::Json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 创建注册中心
    /*let mut registry = Registry {
        map: HashMap::new(),
    };*/

    // 注册服务
    let demo = DemoServiceImpl {};
    let service = demo as Arc<dyn RpcService>;
    register("syx_rpc_rust_demo_api::DemoServer", service);

    HttpServer::new(move || {
        App::new()
            .service(invoker)
    })
        .bind(("localhost", 8080))?
        .run()
        .await
}