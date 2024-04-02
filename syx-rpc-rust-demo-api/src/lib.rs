use serde::{Deserialize, Serialize};
use syx_rpc_rust_macro::syx_trait;

#[syx_trait(provider = "DemoServiceImpl")]
pub trait DemoServer {
    // 单参数验证
    async fn hello(&self, name: String) -> String;

    // 多参数验证
    async fn add(&self, a: i32, b: i32) -> i32;

    // 返回结构体验证
    async fn find_user(&self, id: i32) -> User;

    // 结构体入参验证
    async fn get_age(&self, user: User) -> i32;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub name: String,
    pub age: i32,
}