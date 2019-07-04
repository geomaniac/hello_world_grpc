use std::thread;

use hello_world_grpc::helloworld::*;
use hello_world_grpc::helloworld_grpc::*;

struct GreeterImpl;

impl Greeter for GreeterImpl {
    fn say_hello(
        &self,
        _m: grpc::RequestOptions,
        req: HelloRequest,
    ) -> grpc::SingleResponse<HelloReply> {
        let mut r = HelloReply::new();
        let name = if req.get_name().is_empty() {
            "world"
        } else {
            req.get_name()
        };
        r.set_message(format!("Hello {}", name));
        grpc::SingleResponse::completed(r)
    }
}

fn main() {
    env_logger::init();
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_addr("127.0.0.1:50123").unwrap();
    server.add_service(GreeterServer::new_service_def(GreeterImpl));
    server.http.set_cpu_pool_threads(4);
    let _server = server.build().expect("server");

    loop {
        thread::park();
    }
}
