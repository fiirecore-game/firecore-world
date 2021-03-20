use serde::{Deserialize, Serialize};

use crate::script::ScriptId;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Condition {

    Scripts(Vec<ScriptCondition>),

    PlayerHasPokemon(bool),

    // PlayerPokemon(MatchCondition, Vec<PokemonId>),

    // PlayerHasItem
    // PlayerHasBadge(u8),

}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScriptCondition {

    pub identifier: ScriptId,
    pub happened: bool,

}