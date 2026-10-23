use crate::pieces::{Colour, Piece};

pub struct Board {
    squares: [[bool; 8]; 8],
    white_pieces: Vec<Piece>,
    black_pieces: Vec<Piece>,
}
impl Board {
    pub fn new(board_layout: [[String; 8]; 8]) -> Self {
        let new_squares = [[false; 8]; 8];

        let mut s = Self {
            squares: new_squares,
            white_pieces: Vec::new(),
            black_pieces: Vec::new(),
        };
        s.setup(board_layout);
        s
    }

    pub fn parse_layout(&self, piece_string: &str) -> Piece {
        let mut s: Vec<&str> = piece_string.split(" ").collect();
        Piece::new(
            String::from(s.pop().unwrap()),
            String::from(s.pop().unwrap()),
        )
        .unwrap()
    }

    fn setup(&mut self, board_layout: [[String; 8]; 8]) {
        for row in 0..8 {
            for col in 0..8 {
                let mut s = self.parse_layout(&board_layout[row][col]);
                s.move_piece(col, row);
                self.squares[row][col] = true;
                match s.colour {
                    Colour::White => self.white_pieces.push(s),
                    Colour::Black => self.black_pieces.push(s),
                }
            }
        }
    }
}
