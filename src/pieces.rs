pub struct Piece {
    x: usize,
    y: usize,
    piece_type: String,
    color: String,
}
impl Piece {
    pub fn new(new_type: String, new_color: String) -> Self {
        Self {
            x: 0,
            y: 0,
            piece_type: new_type,
            color: new_color,
        }
    }
    pub fn move_piece(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
}
