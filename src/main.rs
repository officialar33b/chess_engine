use crate::board::{Board, Tile};

mod board;
mod pieces;

fn main() {
    let mut new_board = Board::new();

    // try printing a standard board.
    println!("Drawing Board");

    new_board.draw_board();

    // now try doing the same but change the occupied to true for some.
    new_board.change_tile_occupancy(8);
    new_board.change_tile_occupancy(9);
    new_board.change_tile_occupancy(10);

    println!("New Board");

    new_board.draw_board();
}
