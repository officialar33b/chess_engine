// This is the main game loop; I might change the file name to game.rs, that has the main function. I don't know if
// I can do that though

mod board;
mod game;

use board::Board;
use game::PawnMoves;

fn main() {
    let mut blank_board = Board::empty();
    let mut game_board = Board::new(&mut blank_board);

    game_board.draw_game_board();

    //moving pawn and redrawing.
    game_board.move_pawn(50, PawnMoves::FORWARD);
    game_board.move_pawn(50, PawnMoves::RIGHTFORWARD);
    game_board.draw_game_board();
}
