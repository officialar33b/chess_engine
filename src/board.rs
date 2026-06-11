use crate::pieces::{self, Piece, Roles, Side};

//define each tile. create a tile struct, and then use it to make the board somehow.
pub enum Occ {
    EMPTY,
    OCCUPIED,
}

pub struct Tile {
    num_index: u8,
    x_index: u8,
    y_index: u8,
    occupied: Occ,
}
// Create impl block, for creating new Tile, and add two functions to handle occupied boolean.
impl Tile {
    pub fn new(num_i: u8, x_i: u8, y_i: u8) -> Self {
        Tile {
            num_index: num_i,
            x_index: x_i,
            y_index: y_i,
            occupied: Occ::EMPTY,
        }
    }
    // checks whether the tile is occupied or not
    pub fn is_occupied(&self) -> bool {
        match self.occupied {
            Occ::OCCUPIED => true,
            Occ::EMPTY => false,
        }
    }
    // changes the occupancy based on a boolean input.
    pub fn set_occupied(&mut self, occupy: bool) {
        if occupy == true {
            self.occupied = Occ::OCCUPIED;
        } else {
            self.occupied = Occ::EMPTY;
        }
    }
}

pub struct Board {
    // tiles: [[Tile; 8]; 8], // a 2-dimensional array 8 by 8, which is the size of the chess board.
    tiles: Vec<Tile>,
    pieces: Vec<Piece>, // According to ChatGPT, much better actually!
}

impl Board {
    pub fn new() -> Self {
        let mut tiles = Vec::with_capacity(64); // 8x8
        // create the pieces as well.
        let mut white_pieces = Vec::with_capacity(16);
        let mut black_pieces = Vec::with_capacity(16);

        // maybe combine the pieces into one <Vec> type.
        let mut combine_pieces = Vec::with_capacity(32);
        // adding the tiles and then finally returning a Board Struct.
        for y in 0..8 {
            for x in 0..8 {
                let num_index = y * 8 + x;
                tiles.push(Tile::new(num_index, x, y));
            }
        }

        // for y in 0..2 {//two rows
        //     for x in 0..8{
        //         let num_index = y * 8 +x;
        //         black_pieces.push(Piece::new(num_index, x, y, role, side));
        //     }

        // I'll have to add the types of pieces seperately.
        // I'll create a trait function for adding the piece.

        // KING
        black_pieces.push(Piece::new(4, 4, 4, 0, Roles::KING, Side::BLACK));
        white_pieces.push(Piece::new(60, 60, 4, 7, Roles::KING, Side::WHITE));

        // QUEEN
        black_pieces.push(Piece::new(3, 3, 3, 0, Roles::QUEEN, Side::BLACK));
        white_pieces.push(Piece::new(59, 59, 3, 7, Roles::QUEEN, Side::WHITE));

        // ROOK
        black_pieces.push(Piece::new(0, 0, 0, 0, Roles::ROOK, Side::BLACK));
        black_pieces.push(Piece::new(7, 7, 7, 0, Roles::ROOK, Side::BLACK));

        white_pieces.push(Piece::new(56, 56, 0, 7, Roles::ROOK, Side::WHITE));
        white_pieces.push(Piece::new(63, 63, 7, 7, Roles::ROOK, Side::WHITE));

        // BISHOP
        black_pieces.push(Piece::new(2, 2, 2, 0, Roles::BISHOP, Side::BLACK));
        black_pieces.push(Piece::new(5, 5, 5, 0, Roles::BISHOP, Side::BLACK));

        white_pieces.push(Piece::new(58, 58, 2, 7, Roles::BISHOP, Side::WHITE));
        white_pieces.push(Piece::new(61, 61, 5, 7, Roles::BISHOP, Side::WHITE));

        // KNIGHT
        black_pieces.push(Piece::new(1, 1, 1, 0, Roles::KNIGHT, Side::BLACK));
        black_pieces.push(Piece::new(6, 6, 6, 0, Roles::KNIGHT, Side::BLACK));

        white_pieces.push(Piece::new(57, 57, 1, 7, Roles::KNIGHT, Side::WHITE));
        white_pieces.push(Piece::new(62, 62, 6, 7, Roles::KNIGHT, Side::WHITE));

        // PAWN
        // black PAWNS.
        for j in 0..=7 {
            // j is x, don't be confused.
            let y: u8 = 1;
            black_pieces.push(Piece::new(
                8 * y + j,
                8 * y + j,
                j,
                y,
                Roles::PAWN,
                Side::BLACK,
            ));
        }

        // white PAWNS
        for j in 0..=7 {
            let y: u8 = 6;
            white_pieces.push(Piece::new(
                8 * y + j,
                8 * y + j,
                j,
                y,
                Roles::PAWN,
                Side::WHITE,
            ));
        }

        // for pieces in white_pieces.iter(){
        //     combine_pieces.push(pieces);
        // }

        // for pieces in black_pieces.iter(){
        //     combine_pieces.push(pieces);
        // } incorrect

        combine_pieces.extend(white_pieces);
        combine_pieces.extend(black_pieces);
        Self {
            tiles,
            pieces: combine_pieces,
        }
        // Board { tiles }; this syntax also works.
    }

    // by number index
    pub fn get_tile(&self, index: usize) -> Option<&Tile> {
        return self.tiles.get(index);
    }

    pub fn get_tile_mut(&mut self, index: usize) -> Option<&mut Tile> {
        return self.tiles.get_mut(index);
    }

    // by x and y index.
    // Option type deals with result and none, Result type deals with result and error.
    pub fn get_tile_xy(&self, x: usize, y: usize) -> Option<&Tile> {
        if x < 8 && y < 8 {
            return self.tiles.get(y * 8 + x);
        }
        None
    }

    pub fn change_tile_occupancy(
        &mut self,
        index: usize,
        is_occupied: bool,
    ) -> Result<(), &'static str> {
        if let Some(tile) = self.get_tile_mut(index) {
            tile.set_occupied(is_occupied);
            Ok(())
        } else {
            Err("Invalid board postion")
        }
    }
    // Draw the chessboard showing which tiles are occupied and which aren't.
    pub fn draw_occupancy_board(&self) {
        println!("  0 1 2 3 4 5 6 7");

        for y in 0..8 {
            print!("{} ", y);

            for x in 0..8 {
                let tile = self.get_tile_xy(x, y).unwrap();

                let symbol = match tile.occupied {
                    Occ::EMPTY => '.',
                    Occ::OCCUPIED => 'X',
                };

                print!("{} ", symbol);
            }

            println!();
        }
    }

    pub fn draw_game_board(&mut self) {
        println!("  0 1 2 3 4 5 6 7");

        for y in 0..8 {
            print!("{}", y);

            for x in 0..8 {
                let mut symbol = ".";
                for i in 0..self.pieces.len() {
                    if self.pieces[i].x() == x && self.pieces[i].y() == y {
                        self.change_tile_occupancy(usize::from(8 * y + x), true)
                            .unwrap();

                        symbol = match self.pieces[i].role() {
                            Roles::KING => "♔",
                            Roles::QUEEN => "♕",
                            Roles::BISHOP => "♗",
                            Roles::KNIGHT => "♘",
                            Roles::ROOK => "♖",
                            Roles::PAWN => "♙",
                        };
                        // black looks white and white looks black for some reason.
                        if let Side::BLACK = self.pieces[i].side() {
                            symbol = match self.pieces[i].role() {
                                Roles::KING => "♚",
                                Roles::QUEEN => "♛",
                                Roles::BISHOP => "♝",
                                Roles::KNIGHT => "♞",
                                Roles::ROOK => "♜",
                                Roles::PAWN => "♟️",
                            };
                        }
                        break;
                    }
                }

                // for piece in self.pieces {
                //     if piece.x() == x && piece.y() == y {
                //         // self.change_tile_occupancy(usize::from(8 * y + x), true);
                //         // it only draws when the x and y match.
                //         symbol = match piece.role() {
                //             Roles::KING => "♔",
                //             Roles::QUEEN => "♕",
                //             Roles::BISHOP => "♗",
                //             Roles::KNIGHT => "♘",
                //             Roles::ROOK => "♖",
                //             Roles::PAWN => "♙",
                //         };
                //         // black looks white and white looks black for some reason.
                //         if let Side::BLACK = piece.side() {
                //             symbol = match piece.role() {
                //                 Roles::KING => "♚",
                //                 Roles::QUEEN => "♛",
                //                 Roles::BISHOP => "♝",
                //                 Roles::KNIGHT => "♞",
                //                 Roles::ROOK => "♜",
                //                 Roles::PAWN => "♟️",
                //             };
                //         }
                //         break;
                //     }
                // }
                print!("{} ", symbol);
            }
            println!()
        }
    }
    // add an piece_id, which will be a unique identifier for each individual piece.
    pub fn move_piece(&mut self, piece_index: u8, new_x: u8, new_y: u8) {
        // for loop to find the right index
        for piece in self.pieces.iter_mut() {
            if piece_index == piece.piece_id() {
                piece.change_position(new_x, new_y);
            }
        }
        // i also want to two things
        // 1. Check if another piece is already there.
        // 2. Update the tile.set_occupied somehow.
        self.change_tile_occupancy(usize::from(piece_index), false); // handle the error thing later.
    }
}
