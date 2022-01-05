use crate::isometric_game_grid::IsoGameGrid;
mod isometric_game_grid;

pub fn new_game_grid(cell_height: i32, cell_width: i32, rows: i32, columns: i32) -> IsoGameGrid
{
    return IsoGameGrid::instantiate_new(cell_height, cell_width, rows, columns);
}