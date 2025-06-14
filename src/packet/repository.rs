use serde_json::from_str;
use tonic::Status;

use crate::entity::packet_log::ActiveModel;
use crate::entity::prelude::*;
use crate::packet::proto::Packet;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(Debug)]
pub struct PacketRepository {
    db: DatabaseConnection,
}

impl PacketRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, packet: Packet) -> Result<i32, Status> {
        let json_value = from_str(packet.des_json.as_str())
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let packet_log = ActiveModel {
            packet_id: NotSet,
            packet_type: Set(packet.sender),
            event_name: Set(packet.event),
            des_json: Set(json_value),
            created_at: NotSet,
        };

        let result = PacketLog::insert(packet_log)
            .exec(&self.db)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(result.last_insert_id as i32)
    }
}
