#[derive(PartialEq)]
pub enum Roles {
    KING,
    QUEEN,
    ROOK,
    BISHOP,
    KNIGHT,
    PAWN,
    //1 King, 1 Queen, 2 Rooks, 2 Bishops, 2 Knights, and 8 Pawns
}

#[derive(PartialEq)]
pub enum Side {
    BLACK,
    WHITE,
}

pub struct LegalMove {
    x: u8,
    y: u8,
}

impl LegalMove {
    pub fn new(input_x: u8, input_y: u8) -> Self {
        LegalMove {
            x: input_x,
            y: input_y,
        }
    }
}

pub struct Piece {
    piece_id: u8, // the piece_id will remain unchanged, will be the same as index during initialization, and the index will change, but piece_id will remain the same.
    index: u8,
    x: u8,
    y: u8,
    role: Roles,
    side: Side,
}

impl Piece {
    pub fn new(piece_id: u8, i: u8, x: u8, y: u8, role: Roles, side: Side) -> Self {
        Piece {
            piece_id: piece_id,
            index: i,
            x: x,
            y: y,
            role: role,
            side: side,
        }
    }
    // I had two options, either I make the constructor with pub values, or I add a pub function for scope visibility.
    pub fn piece_id(&self) -> u8 {
        self.piece_id
    }

    pub fn x(&self) -> u8 {
        self.x
    }

    pub fn y(&self) -> u8 {
        self.y
    }
    pub fn role(&self) -> &Roles {
        &self.role
    }
    pub fn side(&self) -> &Side {
        &self.side
    }

    pub fn change_position(&mut self, new_x: u8, new_y: u8) {
        // use a match statement to check for which type of piece and then implement the associeate legal_<Piece>_move.

        self.index = 8 * new_y + new_x;
        self.x = new_x;
        self.y = new_y;
    }

    // Create a legal move checker, that basically checks if the next move
    // is legal or not, should return a boolean or maybe an array actually.
    //
    // For Pawn, the move logic differs because the starting move, allows for a very specific
    pub fn legal_pawn_move(
        &self,
        front_tile_occupied: bool,
        right_diagonal_tile_occupied: bool,
        left_diaogonal_tile_occupied: bool,
        first_move: bool,
    ) -> Vec<LegalMove> {
        //should return a result type, i guess
        // let mut legal_moves: &[LegalMove];
        let mut legal_moves = Vec::new();
        let mut white_moves = Vec::new();
        let mut black_moves = Vec::new();
        if self.role() == &Roles::PAWN {
            if first_move == true {
                //move forward 2 steps.
                // Check if it is white or black (not racist).
                if self.side() == &Side::WHITE {
                    for i in 1..3 {
                        // I
                        white_moves.push(LegalMove::new(self.x, self.y - i));
                    }
                } else if self.side() == &Side::BLACK {
                    for i in 1..3 {
                        black_moves.push(LegalMove::new(self.x, self.y + i));
                    }
                }
            } else if first_move == false {
                // There are two options here, if the next tile is not occupied, it can move one space.
                if self.side() == &Side::WHITE {
                    //forward condition
                    if front_tile_occupied == false {
                        white_moves.push(LegalMove::new(self.x, self.y - 1));
                    } else if right_diagonal_tile_occupied == true {
                        white_moves.push(LegalMove::new(self.x + 1, self.y - 1));
                    } else if left_diaogonal_tile_occupied == true {
                        white_moves.push(LegalMove::new(self.x - 1, self.y - 1));
                    }
                } else if self.side() == &Side::BLACK {
                    if front_tile_occupied == false {
                        black_moves.push(LegalMove::new(self.x, self.y + 1));
                    } else if right_diagonal_tile_occupied == true {
                        black_moves.push(LegalMove::new(self.x - 1, self.y + 1)); // right will be left from black pawn pov.
                    } else if left_diaogonal_tile_occupied == true {
                        black_moves.push(LegalMove::new(self.x + 1, self.y + 1));
                    }
                }

                // if the diagonal tile is occupied, only then can it move space
            }
        }
        // add them all.
        legal_moves.extend(white_moves);
        legal_moves.extend(black_moves);
        return legal_moves; // for now.
    }
    // ROOK
    // pub fn legal_rook_move(
    //     &self,
    //     front_tile_occupied: bool,
    //     back_tile_occupied: bool,
    //     right_tile_occupied: bool,
    //     left_tile_occupied: bool,
    // ) -> Vec<LegalMove> {
    //     let mut legal_moves = Vec::new();
    //     if self.role() == &Roles::ROOK {
    //         if self.side() == &Side::WHITE{

    //             }
    //         }
    //     }

    // }
    // KNIGHT
    // BISHOP
    // QUEEN
    // KING
}
