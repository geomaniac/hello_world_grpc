use hello_world_grpc::helloworld_grpc::*;
use hello_world_grpc::helloworld::*;
use grpc::ClientStubExt;

use std::io;

fn main() {
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("correct input");
        let client = GreeterClient::new_plain("192.168.2.185", 50099, Default::default()).unwrap();

        let mut req = HelloRequest::new();
        req.set_name(line);

        let resp = client.say_hello(grpc::RequestOptions::new(), req);

        println!("{:?}", resp.wait());
    }
}