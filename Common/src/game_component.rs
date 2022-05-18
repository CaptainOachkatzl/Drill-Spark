pub mod ores;
pub use ores::{Ores, get_random_ore_type};

pub mod tile;
pub use tile::{Tile, TileType};

pub mod resources;
pub use resources::*;

pub mod buildings;
pub use buildings::*;

pub mod mine_tag;
pub use mine_tag::*;