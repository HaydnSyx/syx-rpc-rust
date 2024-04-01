use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::json;
use reqwest::Client;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RpcRequest {
    pub service: String,
    pub method_sign: String,
    pub args: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RpcResponse<T: Serialize> {
    pub code: i32,
    pub msg: String,
    pub data: T,
}

// 通用服务暴露接口
#[async_trait::async_trait]
pub trait RpcService: Send + Sync {
    async fn invoke(&self, method_sign: &str, args: Vec<String>) -> String;
}

// ====================provider相关实现====================

lazy_static! {
    static ref PROVIDERS: Mutex<HashMap<String, Box<dyn RpcService>>> = Mutex::new(HashMap::new());
}

pub fn add_service(name: &str, service: Box<dyn RpcService>) {
    let mut map = PROVIDERS.lock().unwrap();
    map.insert(name.to_string(), service);
}

pub async fn invoke_service(req: &RpcRequest) -> RpcResponse<String> {
    let request = req.clone();
    let name = request.service;
    let method_sign = request.method_sign;
    let param = request.args;

    // 执行方法
    let map = PROVIDERS.lock().unwrap();
    let service = map.get(&name);
    if let Some(service) = service {
        let result_future = service.invoke(&method_sign, param);
        match result_future.await {
            result => RpcResponse {
                code: 0,
                msg: String::from("success"),
                data: result,
            },
        }
    } else {
        RpcResponse {
            code: 1,
            msg: String::from("Service not found"),
            data: String::new(),
        }
    }
}

// ====================consumer相关实现====================

pub async fn invoke_provider(request: &RpcRequest) -> String {
    let client = Client::new();
    let res = match client.post("http://localhost:8888/")
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