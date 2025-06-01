use clap::Parser;
use tonic::{Request, Response, Status, transport::Server};

use hello::greeter_server::{Greeter, GreeterServer};
use hello::{HelloReply, HelloRequest};

pub mod hello {
    tonic::include_proto!("hello"); // "hello"는 .proto의 package명
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(
        short = 'H',
        long,
        default_value = "[::1]",
        help = "host to listen on (default: [::1], must be writable by the user running this process)"
    )]
    host: String,

    #[arg(
        short = 'p',
        long,
        default_value = "50051",
        help = "port to listen on (default: 50051, must be writable by the user running this process)"
    )]
    port: String,
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let addr = format!("{}:{}", args.host, args.port).parse()?;
    let greeter = MyGreeter::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
