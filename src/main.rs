// This is the main game loop; I might change the file name to game.rs, that has the main function. I don't know if
// I can do that though

use crate::board::Board;

mod board;
mod pieces;

fn main() {
    let mut new_board = Board::new();

    // try printing a standard board.
    println!("Drawing Board");

    // new_board.draw_occupancy_board();
    new_board.draw_game_board();
    new_board.draw_occupancy_board();
    // now try doing the same but change the occupied to true for some.
    // new_board.change_tile_occupancy(8);
    // new_board.change_tile_occupancy(9);
    // new_board.change_tile_occupancy(10);

    println!("New Board");

    // new_board.draw_occupancy_board();
    new_board.move_piece(50, 2, 4);
    new_board.draw_game_board();
    new_board.draw_occupancy_board();
}
