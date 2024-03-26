use syx_rpc_rust_core::RpcService;
use syx_rpc_rust_demo_api::DemoServer;
use syx_rpc_rust_macro::syx_provider;

// 服务名与服务map


pub struct DemoServiceImpl {}

#[syx_provider]
impl DemoServer for DemoServiceImpl {
    fn hello(&self, name: &str) -> String {
        format!("hello, {}", name)
    }
}

// 实现服务
/*impl RpcService for DemoServiceImpl {
    fn invoke(&self, method_sign: &str, args: &str) -> String {
        if method_sign == "hello" {
            return self.hello(args);
        };
        panic!("method not found");
    }
}*/