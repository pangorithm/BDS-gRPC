#[cfg(debug_assertions)]
use dotenv::dotenv;

use std::env;

use clap::Parser;
use sea_orm::Database;
use tonic::{Request, Response, Status, transport::Server};

pub mod hello {
    tonic::include_proto!("hello");
}

use hello::greeter_service_server::{GreeterService, GreeterServiceServer};
use hello::{SayHelloRequest, SayHelloResponse};

pub mod key_value {
    tonic::include_proto!("key_value");
}

use key_value::key_value_store_service_server::{KeyValueStoreService, KeyValueStoreServiceServer};
use key_value::{GetRequest, GetResponse, KeyValue, PutRequest, PutResponse};

mod interceptor;
use interceptor::auth::AuthInterceptor;

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

    #[arg(
        short = 'a',
        long,
        default_value = "false",
        help = "use authorization (default: false)"
    )]
    authorization: bool,
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl GreeterService for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<SayHelloRequest>,
    ) -> Result<Response<SayHelloResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = SayHelloResponse {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    dotenv().ok();

    let args = Args::parse();

    let addr = format!("{}:{}", args.host, args.port).parse()?;

    println!(
        "Server listening on {}:{}. use authorization is {}",
        args.host, args.port, args.authorization
    );

    let postgres_con_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        env::var("POSTGRES_BDS_USER").unwrap(),
        env::var("POSTGRES_BDS_PASSWORD").unwrap(),
        env::var("POSTGRES_HOST").unwrap(),
        env::var("POSTGRES_PORT").unwrap(),
        env::var("POSTGRES_BDS_DB").unwrap()
    );
    let db = Database::connect(postgres_con_url)
        .await
        .expect("Failed to connect");
    println!("✅ Successfully connected to the database!");

    let greeter = MyGreeter::default();
    if args.authorization {
        let auth_interceptor = AuthInterceptor::new(
            env::var("AUTHORIZATION").unwrap_or_else(|_| "authorization".to_string()),
        );
        Server::builder()
            .add_service(GreeterServiceServer::with_interceptor(
                greeter,
                auth_interceptor,
            ))
            .serve(addr)
            .await?;
    } else {
        Server::builder()
            .add_service(GreeterServiceServer::new(greeter))
            .serve(addr)
            .await?;
    }

    Ok(())
}
