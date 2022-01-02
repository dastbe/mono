use std::env;
use std::str::FromStr;
use std::thread;
use rust_grpc::echo_grpc::*;
use rust_grpc::echo::*;

struct EchoImpl;

impl Echo for EchoImpl {
    fn echo(
        &self,
        _m: grpc::RequestOptions,
        req: EchoRequest,
    ) -> grpc::SingleResponse<EchoResponse> {
        let mut r = EchoResponse::new();
        let msg = if req.get_message().is_empty() {
            "unknown"
        } else {
            req.get_message()
        };
        println!("message was {}", msg);
        r.set_message("pong".to_string());
        grpc::SingleResponse::completed(r)
    }
}

fn main() {
    let mut server = grpc::ServerBuilder::<tls_api_stub::TlsAcceptor>::new();
    let port = u16::from_str(&env::args().nth(1).unwrap_or_else(|| "50051".to_owned())).unwrap();
    server.http.set_port(port);
    server.add_service(EchoServer::new_service_def(EchoImpl));
    server.http.set_cpu_pool_threads(4);
    let server = server.build().expect("server");
    let port = server.local_addr().port().unwrap();
    println!("echo server started on port {}", port);

    loop {
        thread::park();
    }
}
