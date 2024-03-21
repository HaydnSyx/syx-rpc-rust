pub trait DemoServer {
    fn hello(&self, name: &str) -> String;
}