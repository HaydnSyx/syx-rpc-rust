use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::json;
use reqwest::Client;

#[derive(Serialize, Deserialize)]
pub struct RpcRequest {
    pub service: String,
    pub method_sign: String,
    pub args: String,
}

#[derive(Serialize, Deserialize)]
pub struct RpcResponse<T: Serialize> {
    pub code: i32,
    pub msg: String,
    pub data: T,
}

// 通用服务暴露接口
pub trait RpcService: Send + Sync {
    fn invoke(&self, method_sign: &str, args: &str) -> String;
}

// ====================provider相关实现====================

lazy_static! {
    static ref PROVIDERS: Mutex<HashMap<String, Box<dyn RpcService>>> = Mutex::new(HashMap::new());
}

/*pub fn get_service(name: &String) -> Option<&dyn RpcService> {
    let map = PROVIDERS.lock().unwrap();
    map.get(name).map(|service| service.as_ref())
}*/

pub fn add_service(name: &str, service: Box<dyn RpcService>) {
    let mut map = PROVIDERS.lock().unwrap();
    map.insert(name.to_string(), service);
}

pub fn invoke_service(req: &RpcRequest) -> RpcResponse<String> {
    let name = &req.service;
    let method_sign = &req.method_sign;
    let param = &req.args;
    let map = PROVIDERS.lock().unwrap();
    let result = map.get(name).map(|service| service.invoke(method_sign, param)).unwrap();

    // let server = get_service(service).unwrap();
    // let result = server.invoke(method_sign, param);

    RpcResponse {
        code: 0,
        msg: String::from("success"),
        data: result,
    }
}

// ====================consumer相关实现====================

pub async fn invoke_provider(request: &RpcRequest) -> String {
    let client = Client::new();
    let res = match client.post("http://localhost:8080/")
        .json(&json!(&request))
        .send()
        .await {
        Ok(res) => {
            // 请求成功，尝试获取响应文本
            match res.text().await {
                Ok(text) => text, // 将响应返回给原始请求者
                Err(_) => panic!("解析结果失败"),
            }
        }
        Err(_) => panic!("调用失败"),
    };
    res
}