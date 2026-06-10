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
        }
    }
}

pub struct Board {
    // tiles: [[Tile; 8]; 8], // a 2-dimensional array 8 by 8, which is the size of the chess board.
    tiles: Vec<Tile>, // According to ChatGPT, much better actually!
}

impl Board {
    pub fn new() -> Self {
        let mut tiles = Vec::with_capacity(64); // 8x8

        // adding the tiles and then finally returning a Board Struct.
        for y in 0..8 {
            for x in 0..8 {
                let num_index = y * 8 + x;
                tiles.push(Tile::new(num_index, x, y));
            }
        }
        return Self { tiles };
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

    pub fn change_tile_occupancy(&mut self, index: usize) -> Result<(), &'static str> {
        if let Some(tile) = self.get_tile_mut(index) {
            tile.set_occupied(true);
            Ok(())
        } else {
            Err("Invalid board postion")
        }
    }
    // Draw the chessboard.
    pub fn draw_board(&self) {
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
}
