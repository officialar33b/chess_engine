use crate::board::Board;

mod board;
mod pieces;

fn main() {
    let new_board = Board::new();

    println!("Drawing Board");

    new_board.draw_board();
}
