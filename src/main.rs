// This is the main game loop; I might change the file name to game.rs, that has the main function. I don't know if
// I can do that though

mod board;

use board::Board;

fn main() {
    let mut board = Board::empty();
    let board = Board::new(&mut board);

    board.draw_game_board();
}
