use grpc::ClientStubExt;

use rust_grpc::echo_grpc::*;
use rust_grpc::echo::*;

fn main() {
    let client = EchoClient::new_plain("::1", 50051, Default::default()).unwrap();
    let mut req = EchoRequest::new();
    req.set_message("ping".to_string());
    let resp = client.echo(grpc::RequestOptions::new(), req);
    println!("{:?}", resp.wait());
}
