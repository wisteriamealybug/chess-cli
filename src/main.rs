use chess_cli::add;

struct Piece {
    x: usize,
    y: usize,
    piece_type: String,
    color: String,
}
impl Piece {
    fn new(new_type: String, new_color: String) -> Self {
        Self {
            x: 0,
            y: 0,
            piece_type: new_type,
            color: new_color,
        }
    }
}

struct Square<'a> {
    x: usize,
    y: usize,
    color: String,
    piece: Option<&'a Piece>,
}
impl<'a> Square<'a> {
    fn new(x_new: usize, y_new: usize) -> Self {
        Self {
            x: x_new,
            y: y_new,
            color: if x_new.is_multiple_of(2) && !y_new.is_multiple_of(2) {
                String::from("white")
            } else {
                String::from("black")
            },
            piece: None,
        }
    }
}

struct Board<'a> {
    squares: Vec<Vec<Square<'a>>>,
    white_pieces: Vec<Piece>,
    black_pieces: Vec<Piece>,
}
impl<'a> Board<'a> {
    fn new() -> Self {
        let mut squares_create = Vec::new();
        for i in 0..8 {
            squares_create.push(Vec::new());
            for j in 0..8 {
                squares_create[i].push(Square::new(j, i));
            }
        }
        Self {
            squares: squares_create,
            white_pieces: Vec::new(),
            black_pieces: Vec::new(),
        }
    }

    fn setup(&mut self) {
        if self.white_pieces.is_empty() && self.black_pieces.is_empty() {
            for i in 0..8 {
                let mut _piece = Piece::new(String::from("pawn"), String::from("white"));
                self.move_piece(i, 1, _piece);
                self.white_pieces.push(_piece);

                let mut _piece = self.move_piece(
                    i,
                    1,
                    Piece::new(String::from("pawn"), String::from("black")),
                );
                self.black_pieces.push(_piece);
            }
        }
    }

    fn move_piece(&mut self, x: usize, y: usize, mut piece: Piece) -> Piece {
        self.squares[piece.y][piece.x].piece = None;

        piece.x = x;
        piece.y = y;

        self.squares[y][x].piece = Some(&piece);
        piece
    }
}

fn main() {
    println!("Hello rustaceans");
    println!("{}", add(10, 13));
}
