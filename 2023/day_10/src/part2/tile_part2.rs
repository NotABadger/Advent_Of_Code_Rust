pub use crate::tile::TileType;
pub use crate::tile::PipeType;
use crate::error::MyError;

#[derive(Debug)]
pub struct Tile {
    traveled_pipe: bool,
    inside_shape: bool,
    tile_type: TileType,
}

impl Tile {
    pub fn from_tile_type(tile_type: &TileType)-> Self {
        Self { traveled_pipe: false, inside_shape: false, tile_type: *tile_type }
    }
    pub fn get_tile_type(&self) -> TileType {
        self.tile_type
    }
    pub fn set_traveled(&mut self) {
        self.traveled_pipe = true;
    }
    pub fn get_traveled(&self) -> bool {
        self.traveled_pipe
    }
    pub fn set_inside_shape(&mut self) {
        self.inside_shape = true;
    }
    pub fn get_inside_shape(&self) -> bool {
        self.inside_shape
    }
}