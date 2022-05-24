use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Component, Clone, Copy, PartialEq, Eq, Debug)]
pub struct MineTag(pub bool);

impl From<bool> for MineTag {
  fn from(val: bool) -> Self {
    Self { 0: val }
  }
}

impl Into<bool> for MineTag {
    fn into(self) -> bool {
        self.0
    }
}

impl PartialEq<bool> for MineTag {
    fn eq(&self, other: &bool) -> bool {
        self.0 == *other
    }
}

impl PartialEq<bool> for &MineTag {
  fn eq(&self, other: &bool) -> bool {
      self.0 == *other
  }
}
