use crate::packet::repository::PacketRepository;

use tonic::{Request, Response, Status};

use super::proto::packet_logging_service_server::PacketLoggingService;
use super::proto::{Int32Value, Packet};

#[derive(Debug)]
pub struct PacketService {
    repository: PacketRepository,
}

impl PacketService {
    pub fn new(repository: PacketRepository) -> Self {
        Self { repository }
    }
}

#[tonic::async_trait]
impl PacketLoggingService for PacketService {
    async fn put(&self, request: Request<Packet>) -> Result<Response<Int32Value>, Status> {
        let packet: Packet = request.into_inner();

        let reply = Int32Value {
            value: self.repository.insert(packet).await?,
        };

        Ok(Response::new(reply))
    }
}
