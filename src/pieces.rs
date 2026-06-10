pub enum Roles {
    KING,
    QUEEN,
    ROOK,
    BISHOP,
    KNIGHT,
    PAWN,
    //1 King, 1 Queen, 2 Rooks, 2 Bishops, 2 Knights, and 8 Pawns
}

pub enum Side {
    BLACK,
    WHITE,
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
        self.index = 8 * new_y + new_x;
        self.x = new_x;
        self.y = new_y;
    }
}
