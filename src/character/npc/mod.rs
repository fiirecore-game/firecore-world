use firecore_util::Coordinate;
use firecore_util::Entity;
use firecore_util::Position;
use firecore_util::TinyStr16;
use firecore_util::text::Message;
use serde::{Deserialize, Serialize};

use crate::default_true;

use super::CharacterProperties;
use super::movement::MovementType;
use self::trainer::Trainer;

pub mod npc_type;

pub mod trainer;

pub mod character;
pub mod interact;

pub type NPCId = u8;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NPC {

    #[serde(default = "default_true")]
    pub alive: bool,

    pub identifier: NPCIdentifier,

    pub position: Position,

    #[serde(default)]
    pub properties: NPCProperties,

    pub trainer: Option<Trainer>,

}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NPCIdentifier {

    pub index: NPCId,

    pub name: String,
    
    #[serde(rename = "type")]
    pub npc_type: TinyStr16,

}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct NPCProperties {

    #[serde(default)]
    pub character: CharacterProperties,

    #[serde(default)]
    pub movement: MovementType,
    #[serde(skip, default)]
    pub origin: Option<Coordinate>,

    pub message: Option<Vec<Message>>,

}

impl Entity for NPC {
    fn spawn(&mut self) {
        self.alive = true;
    }

    fn despawn(&mut self) {
        self.alive = false;
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}