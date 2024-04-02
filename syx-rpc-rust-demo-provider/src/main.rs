extern crate actix_web;
extern crate syx_rpc_rust_demo_api;

use syx_rpc_rust_core::RpcProviderBootstrap;

use crate::provider::DemoServiceImpl;

mod provider;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 注册服务
    let service = Box::new(DemoServiceImpl {});

    RpcProviderBootstrap::new(8888)
        .register("DemoServiceImpl", service)
        .start()
        .await
}