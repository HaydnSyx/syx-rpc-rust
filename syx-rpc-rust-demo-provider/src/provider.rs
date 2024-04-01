use syx_rpc_rust_core::RpcService;
use syx_rpc_rust_demo_api::DemoServer;
use syx_rpc_rust_macro::syx_provider;

pub struct DemoServiceImpl {}

#[syx_provider]
impl DemoServer for DemoServiceImpl {
    async fn hello(&self, name: String) -> String {
        format!("hello, {}", name)
    }
}