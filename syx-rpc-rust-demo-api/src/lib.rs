// use syx_rpc_rust_macro::rpc_trait;

// #[rpc_trait]
pub trait DemoServer {
    fn hello(&self, name: String) -> String;
}