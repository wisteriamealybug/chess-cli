use std::{str::FromStr, usize};

pub enum Colour {
    White,
    Black,
}

enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    King,
    Queen,
}
impl PieceType {
    fn valid_moves(&self, x: usize, y: usize, colour: &Colour) -> Vec<(usize, usize)> {
        // Awful match statement consider another method
        let mut moves = Vec::new();
        match self {
            Self::Pawn => match colour {
                Colour::White => {
                    moves.push((x, y + 1));
                    if y == 1 {
                        moves.push((x, y + 2));
                    }
                }
                Colour::Black => {
                    moves.push((x, y - 1));
                    if y == 6 {
                        moves.push((x, y - 2));
                    }
                }
            },
            Self::Rook => {
                for i in 0..8 {
                    if i != x {
                        moves.push((i, y));
                    }
                    if i != y {
                        moves.push((x, i));
                    }
                }
            }
            Self::Knight => {
                // Really messy way to do this, change later
                if x - 2 >= 0 {
                    if y + 1 < 8 {
                        moves.push((x - 2, y + 1));
                    }
                    if y - 1 >= 0 {
                        moves.push((x - 2, y - 1));
                    }
                }
                if x + 2 < 8 {
                    if y + 1 < 8 {
                        moves.push((x + 2, y + 1));
                    }
                    if y - 1 >= 0 {
                        moves.push((x + 2, y - 1));
                    }
                }

                if y - 2 >= 0 {
                    if x + 1 < 8 {
                        moves.push((x + 1, y + 2));
                    }
                    if x - 1 >= 0 {
                        moves.push((x - 1, y + 2));
                    }
                }
                if y + 2 < 8 {
                    if x + 1 < 8 {
                        moves.push((x + 1, y - 2));
                    }
                    if x - 1 >= 0 {
                        moves.push((x - 1, y - 2));
                    }
                }
            }
            Self::Bishop => {
                for i in 0..8 {
                    if x + i < 8 && y + i < 8 {
                        moves.push((x + i, y + i));
                    }
                    if x - i >= 0 && y + i < 8 {
                        moves.push((x - i, y + i));
                    }
                    if x + i < 8 && y - i >= 0 {
                        moves.push((x + i, y - i));
                    }
                    if x - i >= 0 && y - i >= 0 {
                        moves.push((x - i, y - i));
                    }
                }
            }
            Self::King => {
                for i in -1..=1 {
                    for j in -1..=1 {
                        if !(i == 0 && j == 0) {
                            moves.push((x + (i as usize), y + (j as usize)));
                        }
                    }
                }
            }
            Self::Queen => {
                for i in 0..8 {
                    if i != x {
                        moves.push((i, y));
                    }
                    if i != y {
                        moves.push((x, i));
                    }
                    if x + i < 8 && y + i < 8 {
                        moves.push((x + i, y + i));
                    }
                    if x - i >= 0 && y + i < 8 {
                        moves.push((x - i, y + i));
                    }
                    if x + i < 8 && y - i >= 0 {
                        moves.push((x + i, y - i));
                    }
                    if x - i >= 0 && y - i >= 0 {
                        moves.push((x - i, y - i));
                    }
                }
            }
        }
        moves
    }
}

#[derive(Debug)]
pub struct UnknownPieceError;

impl FromStr for PieceType {
    type Err = UnknownPieceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "pawn" => Ok(Self::Pawn),
            "rook" => Ok(Self::Rook),
            "knight" => Ok(Self::Knight),
            "bishop" => Ok(Self::Bishop),
            "king" => Ok(Self::King),
            "queen" => Ok(Self::Queen),
            _ => Err(UnknownPieceError),
        }
    }
}

pub struct Piece {
    pub x: usize,
    pub y: usize,
    pub piece_type: PieceType,
    pub colour: Colour,
}
impl Piece {
    pub fn new(new_type: String, new_colour: String) -> Result<Self, UnknownPieceError> {
        match PieceType::from_str(new_type.as_str()) {
            Ok(t) => Ok(Self {
                x: 0,
                y: 0,
                piece_type: t,
                colour: if new_colour.to_lowercase().trim() == "white" {
                    Colour::White
                } else {
                    Colour::Black
                },
            }),
            Err(e) => Err(e),
        }
    }
    pub fn set_piece_loc(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
    pub fn move_piece(&mut self, end_pos: (usize, usize)) -> bool {
        let valid_moves = self.piece_type.valid_moves(self.x, self.y, &self.colour);
        if valid_moves.contains(&end_pos) {
            self.x = end_pos.0;
            self.y = end_pos.1;
            return true;
        }
        return false;
    }
}
