extern crate actix_web;

use actix_web::{App, get, HttpServer, web};

#[get("/hello/{name}")]
async fn invoker_hello(name: web::Path<String>) -> actix_web::Result<web::Json<String>> {
    let rpc = syx_rpc_rust_demo_api::DemoServerRpc {};
    let res = rpc.hello(name.to_string()).await;
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
