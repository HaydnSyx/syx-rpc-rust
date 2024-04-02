use syx_rpc_rust_core::RpcService;
use syx_rpc_rust_demo_api::{DemoServer, User};
use syx_rpc_rust_macro::syx_provider;

pub struct DemoServiceImpl {}

#[syx_provider]
impl DemoServer for DemoServiceImpl {
    async fn hello(&self, name: String) -> String {
        format!("hello, {}", name)
    }

    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    async fn find_user(&self, id: i32) -> User {
        User {
            name: "haha".to_string(),
            age: id + 10,
        }
    }

    async fn get_age(&self, user: User) -> i32 {
        user.age
    }
}