use tonic::Status;

use crate::common::struct_to_value;
use serde_json::Value as JsonValue;

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
        let json_value = packet
            .des
            .map(|s| struct_to_value(s))
            .unwrap_or(JsonValue::Null);

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
