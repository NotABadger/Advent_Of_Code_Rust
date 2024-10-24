

#[derive(Debug)]
pub struct Tile {

}

#[derive(Debug)]
pub enum TileType {
    ground,
    pipe(PipeType),
    start_pos,
}

#[derive(Debug)]
pub enum PipeType {
    vertical,
    horizontal,
    corner_n_e,
    corner_n_w,
    corner_z_e,
    corner_z_w,
}