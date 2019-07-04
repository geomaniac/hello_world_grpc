use grpc::ClientStub;
use hello_world_grpc::helloworld::*;
use hello_world_grpc::helloworld_grpc::*;
use std::sync::Arc;

fn main() {
    env_logger::init();
    let executor = tokio_core::reactor::Core::new().unwrap();
    let client = GreeterClient::with_client(Arc::new(
        grpc::Client::new_explicit_plain(
            "127.0.0.1",
            50123,
            Default::default(),
            // replace this line with `None` to make it work again.
            Some(executor.remote()),
        )
        .unwrap(),
    ));

    let mut req = HelloRequest::new();
    req.set_name("World!".to_string());

    let resp = client.say_hello(grpc::RequestOptions::new(), req);

    println!("{:?}", resp.wait_drop_metadata().unwrap().message);
}
