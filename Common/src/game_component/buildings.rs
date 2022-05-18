use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Copy, PartialEq, Eq)]
pub enum Buildings {
  WarpGate,
  Tower,
  DrillDepot,
}
