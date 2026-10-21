use crate::pieces::Piece;

struct Board {
    squares: [[bool; 8]; 8],
    white_pieces: Vec<Piece>,
    black_pieces: Vec<Piece>,
    intial_layout: [[String; 8]; 8],
}
impl Board {
    fn new(board_layout: [[String; 8]; 8]) -> Self {
        let squares = [[false; 8]; 8];

        Self {
            squares: squares,
            white_pieces: Vec::new(),
            black_pieces: Vec::new(),
            intial_layout: board_layout,
        }
    }

    fn setup(&mut self) {
        if self.white_pieces.is_empty() && self.black_pieces.is_empty() {
            for i in 0..8 {
                let mut _piece = Piece::new(String::from("pawn"), String::from("white"));
                _piece.move_piece(i, 1);
                self.white_pieces.push(_piece);

                let mut _piece = Piece::new(String::from("pawn"), String::from("black"));
                _piece.move_piece(i, 6);
                self.black_pieces.push(_piece);
            }
        }

        for i in 0..8 {
            for j in 0..1 {
                self.squares[j][i] = true;
            }
            for j in 6..8 {
                self.squares[j][i] = true;
            }
        }
    }
}
