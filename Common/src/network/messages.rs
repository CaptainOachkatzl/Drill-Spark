use std::collections::BTreeMap;

use bevy_spicy_networking::*;
use nameof::name_of;
use serde::{Deserialize, Serialize};
use typetag::serde;
use xs_bevy_core_2d::Position;

use crate::{Buildings, MineTag, TileType, Resources};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RevealMessage {
  pub position: Position,
  pub tile_type: Option<TileType>, // None means tile is not revealed
}

#[typetag::serde]
impl NetworkMessage for RevealMessage {}

impl ClientMessage for RevealMessage {
  const NAME: &'static str = name_of!(type RevealMessage);
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MineTagMessage {
  pub mine_tags: Vec<(Position, MineTag)>,
}

#[typetag::serde]
impl NetworkMessage for MineTagMessage {}

impl ServerMessage for MineTagMessage {
  const NAME: &'static str = name_of!(type MineTagMessage);
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BuildRequestMessage {
  pub center: Position,
  pub blueprint: Buildings,
}

#[typetag::serde]
impl NetworkMessage for BuildRequestMessage {}

impl ServerMessage for BuildRequestMessage {
  const NAME: &'static str = name_of!(type BuildRequestMessage);
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResourceMessage {
  pub resources: BTreeMap<Resources, usize>
}

#[typetag::serde]
impl NetworkMessage for ResourceMessage {}

impl ClientMessage for ResourceMessage {
  const NAME: &'static str = name_of!(type ResourceMessage);
}
