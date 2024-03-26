use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

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

pub fn add_service(name: &str, service: Box<dyn RpcService>) {
    let mut map = PROVIDERS.lock().unwrap();
    map.insert(name.to_string(), service);
}

pub fn invoke_service<F, R>(name: &String, f: F) -> Option<R>
    where F: FnOnce(&dyn RpcService) -> R {
    let map = PROVIDERS.lock().unwrap();
    map.get(name).map(|service| f(service.as_ref()))
}

/*pub fn get_service(name: &String) -> Option<&dyn RpcService> {
    let map = PROVIDERS.lock().unwrap();
    map.get(name).map(|service| service.as_ref())
}*/

// ====================consumer相关实现====================