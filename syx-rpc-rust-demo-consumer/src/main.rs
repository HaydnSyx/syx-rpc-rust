extern crate actix_web;

use actix_web::{App, get, HttpServer, web};

use syx_rpc_rust_core::RpcRequest;
use syx_rpc_rust_macro::rpc_trait;

#[rpc_trait(provider="DemoServer")]
pub trait DemoServerConsumer {
    fn hello(&self, name: String) -> String;
}

#[get("/hello/{name}")]
// #[tokio::main]
async fn invoker_hello(name: web::Path<String>) -> actix_web::Result<web::Json<String>> {
   /* let request = RpcRequest {
        service: String::from("DemoServiceImpl"),
        method_sign: String::from("hello"),
        args: name.to_string(),
    };
    let res = syx_rpc_rust_core::invoke_provider(&request).await;*/
    let rpc = DemoServerConsumerRpc::new();
    let res = rpc.hello(name.to_string());
    Ok::<web::Json<String>, actix_web::Error>(web::Json(res))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(invoker_hello)
    })
        .bind(("localhost", 7777))?
        .run()
        .await
}
