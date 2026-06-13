/*
 * Instead of making a seperate struct for pieces and the tiles themselves, I'm going to create board struct that has both inside it.
 * Hopefully this simplifies my approach, and functionality.
 *
 *
 */

use crate::game::PawnMoves;

#[derive(PartialEq)]
pub enum Color {
    BLACK,
    WHITE,
}

pub enum Role {
    KING,
    QUEEN,
    BISHOP,
    ROOK,
    KNIGHT,
    PAWN,
}

pub struct Piece {
    piece_id: u8,
    tile_id: u8,
    color: Color,
    role: Role,
}

impl Piece {
    pub fn new(piece_id: u8, tile_id: u8, color: Color, role: Role) -> Self {
        Self {
            piece_id,
            tile_id,
            color,
            role,
        }
    }

    pub fn role(&self) -> &Role {
        &self.role
    }

    pub fn piece_id(self) -> u8 {
        return self.piece_id;
    }
}

pub struct Tile {
    tile_id: u8,
    is_occupied: bool,
}

impl Tile {
    pub fn new(tile_id: u8, is_occupied: bool) -> Self {
        Self {
            tile_id,
            is_occupied,
        }
    }
}

// main <Board> struct that holds a vector of these two, change the data type from vector, once the logic starts, making sense.
pub struct Board {
    pieces: Vec<Piece>,
    tiles: Vec<Tile>,
}

impl Board {
    pub fn empty() -> Self {
        Self {
            pieces: Vec::new(),
            tiles: Vec::new(),
        }
    }
    pub fn new(&mut self) -> Self {
        // Create the tiles first, so that the occupancy is made correctly.
        Self {
            pieces: self.create_pieces(),
            tiles: self.create_tiles(),
        }
    }

    // Create the pieces.
    fn create_pieces(&mut self) -> Vec<Piece> {
        // initialize create_<Role> functions, and add them to a single Vec<Piece> type.
        let mut game_pieces = Vec::new();
        game_pieces.extend(self.create_kings());
        game_pieces.extend(self.create_queens());
        game_pieces.extend(self.create_bishops());
        game_pieces.extend(self.create_rooks());
        game_pieces.extend(self.create_knights());
        game_pieces.extend(self.create_pawns());

        return game_pieces;
    }

    fn create_kings(&mut self) -> Vec<Piece> {
        let black_king_id: u8 = 4;
        let white_king_id: u8 = 60;

        self.occupy(black_king_id);
        self.occupy(white_king_id);

        let mut kings_vec = Vec::new();
        kings_vec.push(Piece::new(
            black_king_id,
            black_king_id,
            Color::BLACK,
            Role::KING,
        ));
        kings_vec.push(Piece::new(
            white_king_id,
            white_king_id,
            Color::WHITE,
            Role::KING,
        ));

        return kings_vec;
    }

    // Queens
    fn create_queens(&mut self) -> Vec<Piece> {
        let black_queen_id: u8 = 3;
        let white_queen_id: u8 = 59;

        self.occupy(black_queen_id);
        self.occupy(white_queen_id);

        let mut queens_vec = Vec::new();
        queens_vec.push(Piece::new(
            black_queen_id,
            black_queen_id,
            Color::BLACK,
            Role::QUEEN,
        ));
        queens_vec.push(Piece::new(
            white_queen_id,
            white_queen_id,
            Color::WHITE,
            Role::QUEEN,
        ));

        return queens_vec;
    }

    // Rooks.
    fn create_rooks(&mut self) -> Vec<Piece> {
        let black_rook_id: [u8; 2] = [0, 7];
        let white_rook_id: [u8; 2] = [56, 63];

        let mut rooks_vec = Vec::new();
        for i in black_rook_id {
            rooks_vec.push(Piece::new(i, i, Color::BLACK, Role::ROOK));
            self.occupy(i);
        }
        for i in white_rook_id {
            rooks_vec.push(Piece::new(i, i, Color::WHITE, Role::ROOK));
            self.occupy(i);
        }
        return rooks_vec;
    }

    // Bishops
    fn create_bishops(&mut self) -> Vec<Piece> {
        let black_bishops_id: [u8; 2] = [2, 5];
        let white_bishops_id: [u8; 2] = [58, 61];

        let mut bishops_vec = Vec::new();
        for i in black_bishops_id {
            bishops_vec.push(Piece::new(i, i, Color::BLACK, Role::BISHOP));
            self.occupy(i);
        }
        for i in white_bishops_id {
            bishops_vec.push(Piece::new(i, i, Color::WHITE, Role::BISHOP));
            self.occupy(i);
        }
        return bishops_vec;
    }
    // Knights.
    fn create_knights(&mut self) -> Vec<Piece> {
        let black_knights_id: [u8; 2] = [1, 6];
        let white_knights_id: [u8; 2] = [57, 62];

        let mut knights_vec = Vec::new();
        for i in black_knights_id {
            knights_vec.push(Piece::new(i, i, Color::BLACK, Role::KNIGHT));
            self.occupy(i);
        }
        for i in white_knights_id {
            knights_vec.push(Piece::new(i, i, Color::WHITE, Role::KNIGHT));
            self.occupy(i);
        }
        return knights_vec;
    }
    // Pawns
    fn create_pawns(&mut self) -> Vec<Piece> {
        let mut pawns_vec = Vec::new();
        //black pawns.
        for i in 8..16 {
            pawns_vec.push(Piece::new(i, i, Color::BLACK, Role::PAWN));
            self.occupy(i);
        }

        for i in 48..56 {
            pawns_vec.push(Piece::new(i, i, Color::WHITE, Role::PAWN));
            self.occupy(i);
        }

        return pawns_vec;
    }

    // Create the tiles using tiles itself.
    fn create_tiles(&self) -> Vec<Tile> {
        let mut game_tiles = Vec::new();
        for i in 0..64 {
            game_tiles.push(Tile::new(i, false));
        }

        return game_tiles;
    }
    // change to occupy function.
    fn occupy(&mut self, tile_id: u8) {
        for tile in self.tiles.iter_mut() {
            // set the occupancy to true.
            if tile.tile_id == tile_id {
                tile.is_occupied = true;
            }
        }
    }

    fn de_occupy(&mut self, tile_id: u8) {
        for tile in self.tiles.iter_mut() {
            if tile.tile_id == tile_id {
                tile.is_occupied = false;
            }
        }
    }
    //draw_board_function.
    pub fn draw_game_board(&self) {
        for y in 0..8 {
            for x in 0..8 {
                let i = 8 * y + x;
                let mut symbol = String::from("_");
                for game_piece in &self.pieces {
                    if game_piece.tile_id == i {
                        if game_piece.color == Color::WHITE {
                            match game_piece.role() {
                                Role::KING => symbol = String::from("♔"),
                                Role::QUEEN => symbol = String::from("♕"),
                                Role::BISHOP => symbol = String::from("♗"),
                                Role::KNIGHT => symbol = String::from("♘"),
                                Role::ROOK => symbol = String::from("♖"),
                                Role::PAWN => symbol = String::from("♙"),
                            }
                        } else {
                            match game_piece.role() {
                                Role::KING => symbol = String::from("♚"),
                                Role::QUEEN => symbol = String::from("♛"),
                                Role::BISHOP => symbol = String::from("♝"),
                                Role::KNIGHT => symbol = String::from("♞"),
                                Role::ROOK => symbol = String::from("♜"),
                                Role::PAWN => symbol = String::from("♟"),
                            }
                        }
                    }
                }
                print!("{} ", symbol);
            }
            println!();
        }
    }

    // movement itself.
    // Let's start with Pawns.
    pub fn move_pawn(&mut self, piece_id: u8, movement: PawnMoves) {
        // Find the pawn first.
        let piece_index = self
            .pieces
            .iter()
            .position(|p| p.piece_id == piece_id)
            .expect("Piece not found");

        let piece = &self.pieces[piece_index];

        // Calculate destination.
        let target_tile = match piece.color {
            Color::BLACK => match movement {
                PawnMoves::FORWARD => piece.tile_id + 8,
                PawnMoves::RIGHTFORWARD => piece.tile_id + 9,
                PawnMoves::LEFTFORWARD => piece.tile_id + 7,
            },
            Color::WHITE => match movement {
                PawnMoves::FORWARD => piece.tile_id - 8,
                PawnMoves::RIGHTFORWARD => piece.tile_id - 7,
                PawnMoves::LEFTFORWARD => piece.tile_id - 9,
            },
        };

        let occupied = self.pieces.iter().any(|p| p.tile_id == target_tile);

        match movement {
            // Forward move requires an empty square.
            PawnMoves::FORWARD => {
                if !occupied {
                    self.pieces[piece_index].tile_id = target_tile;
                } else {
                    println!("Occupied");
                }
            }

            // Diagonal moves require a piece on the target square.
            PawnMoves::RIGHTFORWARD | PawnMoves::LEFTFORWARD => {
                if occupied {
                    self.pieces[piece_index].tile_id = target_tile;
                } else {
                    println!("Illegal Move.");
                }
            }
        }
    }
}
