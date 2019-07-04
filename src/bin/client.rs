use futures::future::Future;
use grpc::ClientStub;
use hello_world_grpc::helloworld::*;
use hello_world_grpc::helloworld_grpc::*;
use std::sync::Arc;

fn main() {
    env_logger::init();
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let client = GreeterClient::with_client(Arc::new(
        grpc::Client::new_explicit_plain(
            "127.0.0.1",
            50123,
            Default::default(),
            // replace this line with `None` to make it work again.
            Some(core.remote()),
        )
        .unwrap(),
    ));

    let mut req = HelloRequest::new();
    req.set_name("World!".to_string());

    let resp = client.say_hello(grpc::RequestOptions::new(), req);

    core.run(
        resp.drop_metadata()
            .map(|resp| println!("{:?}", resp.message)),
    )
    .unwrap();
}
