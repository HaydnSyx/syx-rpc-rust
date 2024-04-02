## 使用rust实现简易rpc框架

通信协议：`http`

序列化：`json`

以下是demo示意

### 接口定义

模块: syx-rpc-rust-demo-api

#### 1.定义接口

```rust
pub trait DemoServer {
    // 单参数场景
    async fn hello(&self, name: String) -> String;
    // 多参数场景
    async fn add(&self, a: i32, b: i32) -> i32;
    // 返回结构体场景
    async fn find_user(&self, id: i32) -> User;
    // 结构体入参场景
    async fn get_age(&self, user: User) -> i32;
}
```

#### 2.在接口上标注宏`syx_trait`

```rust
#[syx_trait(provider = "DemoServiceImpl")]
pub trait DemoServer {
    ...
}
```

### 服务端处理
模块: syx-rpc-rust-demo-provider

#### 1.服务端接口实现

```rust
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
```

#### 2.启动服务端监听

```rust
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 注册服务
    let service = Box::new(DemoServiceImpl {});
    RpcProviderBootstrap::new(8888)
        .register("DemoServiceImpl", service)
        .start()
        .await
}
```

### 消费端处理
模块: syx-rpc-rust-demo-consumer

#### 1.定义全局配置

```rust
lazy_static! {
    static ref CONFIG: RpcConsumerClientConfig = RpcConsumerClientConfig::new(
        "localhost".to_string(), 8888
    );
}
```

#### 2.调用服务

```rust
// hello方法调用
let rpc = syx_rpc_rust_demo_api::DemoServerRpc::new(&CONFIG);
let res = rpc.hello("syx".to_string()).await;
// add方法调用
let rpc = syx_rpc_rust_demo_api::DemoServerRpc::new(&CONFIG);
let res = rpc.add(1, 2).await;
// find_user方法调用
let rpc = syx_rpc_rust_demo_api::DemoServerRpc::new(&CONFIG);
let res = rpc.find_user(10).await;
// get_age方法调用
let user = User {
    name: "syx".to_string(),
    age: 18,
};
let rpc = syx_rpc_rust_demo_api::DemoServerRpc::new(&CONFIG);
let res = rpc.get_age(user).await;
```