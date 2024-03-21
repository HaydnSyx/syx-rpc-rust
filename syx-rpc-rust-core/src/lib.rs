use std::collections::HashMap;
use std::sync::{Arc, Mutex};
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
    fn invoke(&self, args: &str) -> String;
}

// ====================provider相关实现====================

lazy_static! {
    static ref PROVIDERS: Mutex<HashMap<String, Arc<dyn RpcService>>> = Mutex::new(HashMap::new());
}

/*pub struct Registry {
    pub map: HashMap<String, Arc<dyn RpcService>>,
}

impl Registry {
    pub fn register(&mut self, name: &str, service: Arc<dyn RpcService>) {
        PROVIDERS.insert(name.to_string(), service);
    }

    pub fn get_service(&self, name: &String) -> &Arc<dyn RpcService> {
        match self.map.get(name) {
            Some(service) => service,
            None => panic!("not find service by {}", name),
        }
    }
}*/

pub fn register(name: &str, service: Arc<dyn RpcService>) {
    let mut map = PROVIDERS.lock().unwrap();
    map.insert(name.to_string(), service);
}

pub fn get_service(name: &String) -> &Arc<dyn RpcService> {
    let mut map = PROVIDERS.lock().unwrap();
    match map.get(name) {
        Some(service) => service,
        None => panic!("not find service by {}", name),
    }
}


// ====================consumer相关实现====================