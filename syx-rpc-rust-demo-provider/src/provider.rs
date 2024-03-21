use syx_rpc_rust_core::RpcService;
use syx_rpc_rust_demo_api::DemoServer;
use std::collections::HashMap;

// 服务名与服务map


pub struct DemoServiceImpl {}

impl DemoServer for DemoServiceImpl {
    fn hello(&self, name: &str) -> String {
        format!("hello, {}", name)
    }
}

// 实现服务
impl RpcService for DemoServiceImpl {
    fn invoke(&self, args: &str) -> String {
        self.hello(args)
    }
}