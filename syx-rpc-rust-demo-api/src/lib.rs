use syx_rpc_rust_macro::rpc_trait;

#[rpc_trait(provider="DemoServiceImpl")]
pub trait DemoServer {
    async fn hello(&self, name: String) -> String;
}