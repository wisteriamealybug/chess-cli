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

    fn parse_layout(&self, piece_string: &str) -> Piece {
        let mut s: Vec<&str> = piece_string.split(" ").collect();
        Piece::new(
            String::from(s.pop().unwrap()),
            String::from(s.pop().unwrap()),
        )
        .unwrap()
    }

    pub fn setup(&mut self, board_layout: [[String; 8]; 8]) {
        for row in 0..8 {
            for col in 0..8 {
                let mut s = self.parse_layout(&board_layout[row][col]);
                s.set_piece_loc(col, row);
                self.squares[row][col] = true;
                match s.colour {
                    Colour::White => self.white_pieces.push(s),
                    Colour::Black => self.black_pieces.push(s),
                }
            }
        }
    }

    pub fn move_piece(&mut self, piece: &mut Piece, end_pos: (usize, usize)) -> bool {
        let start_pos = (piece.x, piece.y);
        if piece.move_piece(end_pos) {
            self.squares[start_pos.1][start_pos.0] = false;
            if self.squares[end_pos.1][end_pos.0] {
                match piece.colour {
                    Colour::Black => {
                        self.white_pieces.swap_remove(
                            self.white_pieces
                                .iter()
                                .position(|enemy| enemy.x == end_pos.0 && enemy.y == end_pos.1)
                                .unwrap(),
                        );
                    }
                    Colour::White => {
                        self.black_pieces.swap_remove(
                            self.black_pieces
                                .iter()
                                .position(|enemy| enemy.x == end_pos.0 && enemy.y == end_pos.1)
                                .unwrap(),
                        );
                    }
                }
            }
            return true;
        }
        return false;
    }
}
