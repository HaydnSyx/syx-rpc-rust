use std::collections::HashMap;
use std::sync::Mutex;
use actix_web::{App, HttpServer, post, web};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::json;
use reqwest::Client;

// ====================核心定义====================
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RpcRequest {
    pub service: String,
    pub method_sign: String,
    pub args: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RpcResponse {
    pub code: i32,
    pub msg: String,
    pub data: String,
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

pub struct RpcProviderBootstrap {
    // pub providers: HashMap<String, Box<dyn RpcService>>,
    pub port: u16,
}

impl RpcProviderBootstrap {
    pub fn new(port: u16) -> Self {
        RpcProviderBootstrap {
            // providers: HashMap::new(),
            port
        }
    }

    // 增加注册方法
    pub fn register(self, name: &str, service: Box<dyn RpcService>) -> RpcProviderBootstrap {
        let mut map = PROVIDERS.lock().unwrap();
        map.insert(name.to_string(), service);
        self
    }

    pub async fn start(self) -> Result<(), std::io::Error> {
        let server_port = self.port;
        // 启动 HTTP 服务器并等待直到它停止
        HttpServer::new(move || {
            App::new()
                .service(invoker)
        })
            .bind(("localhost", server_port))?
            .run()
            .await
    }
}

#[post("/")]
async fn invoker(req: web::Json<RpcRequest>) -> actix_web::Result<web::Json<RpcResponse>> {
    println!("provider接收到的请求: {:?}", req.0);
    let response = invoke_service(&req).await;
    println!("provider返回的结果: {:?}", response);
    Ok::<web::Json<RpcResponse>, actix_web::Error>(web::Json(response))
}

pub async fn invoke_service(req: &RpcRequest) -> RpcResponse {
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

pub struct RpcConsumerClientConfig {
    _host: String,
    _port: u16,
    client: Client,
    url: String
}

impl RpcConsumerClientConfig {
    pub fn new(host: String, port: u16) -> Self {
        RpcConsumerClientConfig {
            _host: host.clone(),
            _port: port.clone(),
            client: Client::new(),
            url: format!("http://{}:{}/", host.clone(), port),
        }
    }
}

pub async fn invoke_provider(request: &RpcRequest, config: &RpcConsumerClientConfig) -> String {
    let client = &config.client;
    let res = match client.post(&config.url)
        .json(&json!(&request))
        .send()
        .await {
        Ok(res) => {
            // 请求成功，尝试获取响应文本
            match res.text().await {
                Ok(text) => {
                    println!("provider响应信息: {}", text);
                    text
                }, // 将响应返回给原始请求者
                Err(_) => panic!("解析结果失败"),
            }
        }
        Err(_) => panic!("调用失败"),
    };
    res
}