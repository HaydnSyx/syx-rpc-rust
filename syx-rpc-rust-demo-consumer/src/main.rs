use reqwest::Client;
use serde_json::json;

use syx_rpc_rust_core::RpcRequest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let request = RpcRequest {
        service: String::from("DemoServiceImpl"),
        method_sign: String::from("hello"),
        args: String::from("syx"),
    };

    let client = Client::new();
    let res = client.post("http://localhost:8080/")
        .json(&json!(&request))
        .send()
        .await?
        .text()
        .await?;
    println!("{:#?}", res);
    Ok(())
}
