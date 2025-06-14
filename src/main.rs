#[cfg(debug_assertions)]
use dotenv::dotenv;

use std::env;

use clap::Parser;
use sea_orm::Database;
use tonic::transport::Server;

mod common;
mod entity;

mod packet;
use packet::proto::packet_logging_service_server::PacketLoggingServiceServer;
use packet::service::PacketService;

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

    let packet_repository = packet::repository::PacketRepository::new(db.clone());
    let packet_service = PacketService::new(packet_repository);

    if args.authorization {
        let auth_interceptor = AuthInterceptor::new(
            env::var("AUTHORIZATION").unwrap_or_else(|_| "authorization".to_string()),
        );
        Server::builder()
            .add_service(PacketLoggingServiceServer::with_interceptor(
                packet_service,
                auth_interceptor.clone(),
            ))
            .serve(addr)
            .await?;
    } else {
        Server::builder()
            .add_service(PacketLoggingServiceServer::new(packet_service))
            .serve(addr)
            .await?;
    }

    Ok(())
}
