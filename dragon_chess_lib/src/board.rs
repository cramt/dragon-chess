use crate::pieces::Piece;
use crate::grid::Grid;
use crate::pieces::vector3::Vector3;
use crate::grid::IndexValid::{DefaultValue, NonDefaultValue, OutOfBounds};
use crate::board::MoveType::{Move, Capture, RemoteCapture};
use std::rc::Rc;
use crate::board_piece::BoardPiece;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum MoveType {
    Capture,
    RemoteCapture,
    Move,
}

pub struct Board {
    pub grid: Grid<Option<Box<dyn Piece>>>,
    dead_pieces: Vec<Box<dyn Piece>>,
}

impl Board {
    pub fn new() -> Board {
        Board::new_specified(vec![])
    }

    pub fn new_specified(pieces: Vec<Box<dyn Piece>>) -> Board {
        Board {
            grid: Board::build_state(pieces),
            dead_pieces: vec![],
        }
    }

    pub(self) fn build_state(pieces: Vec<Box<dyn Piece>>) -> Grid<Option<Box<dyn Piece>>> {
        let mut res = Grid::new();
        for p in pieces {
            let position = p.get_position().clone();
            res[&position] = Some(p);
        }
        res
    }

    pub fn board_piece(self, pos: Vector3) -> Option<BoardPiece> {
        BoardPiece::new(pos, self)
    }

    pub fn possible_moves(&self, piece: &Box<dyn Piece>) -> Grid<Option<MoveType>> {
        let mut moves = Grid::new();
        let move_dirs = piece.move_directions();
        let cap_dirs = piece.capture_directions();
        for move_dir in move_dirs {
            if move_dir.repeated {
                for dir in &move_dir.directions {
                    let mut curr = *piece.get_position() + *dir;
                    loop {
                        if self.grid.valid(&curr) != DefaultValue {
                            break;
                        }
                        moves[&curr] = Some(Move);
                        curr = curr + *dir;
                    }
                }
            } else if move_dir.remote {
                panic!("you cant move remote");
            } else {
                let dir = move_dir.directions.into_iter().map(|v| v + *piece.get_position()).collect::<Vec<Vector3>>();
                for dir in dir {
                    if self.grid.valid(&dir) == DefaultValue {
                        moves[&dir] = Some(Move);
                    }
                }
            }
        }
        for cap_dir in cap_dirs {
            if cap_dir.repeated {
                for dir in &cap_dir.directions {
                    let mut curr = *piece.get_position() + *dir;
                    loop {
                        match self.grid.valid(&curr) {
                            OutOfBounds => break,
                            DefaultValue => {}
                            NonDefaultValue => {
                                if self.grid[&curr].as_ref().unwrap().get_player() != piece.get_player() {
                                    moves[&curr] = Some(Capture);
                                }
                                break;
                            }
                        };
                        curr = curr + *dir;
                    }
                }
            } else {
                let dir = cap_dir.directions.into_iter().map(|v| v + *piece.get_position()).collect::<Vec<Vector3>>();
                for dir in dir {
                    if self.grid.valid(&dir) == NonDefaultValue {
                        moves[&dir] = Some(if cap_dir.remote { RemoteCapture } else { Capture });
                    }
                }
            }
        }
        moves
    }

    pub fn move_piece(&mut self, from: Vector3, to: Vector3, possible_moves: Grid<Option<MoveType>>) -> Result<(), &str> {
        match &possible_moves[&to] {
            Some(move_type) => {
                let move_type = *move_type;
                if move_type != Move {
                    self.kill_piece(&to);
                }
                if move_type != RemoteCapture {
                    self.change_position(&from, &to);
                }
            }
            None => {
                return Err("not a possible move");
            }
        };
        Ok(())
    }

    fn change_position(&mut self, from_position: &Vector3, to_position: &Vector3) {
        let from = self.grid.swap(from_position, None);
        let to = self.grid.swap(to_position, None);
        if to.is_some() {
            let mut piece = to.unwrap();
            piece.set_position(*from_position);
            //piece.set_position(*from_position);
            self.grid.swap(from_position, Some(piece));
        }
        if from.is_some() {
            let mut piece = from.unwrap();
            piece.set_position(*to_position);
            //piece.set_position(*to_position);
            self.grid.swap(to_position, Some(piece));
        }
    }

    pub fn kill_piece(&mut self, position: &Vector3) {
        match self.grid.swap(position, None) {
            Some(piece) => self.dead_pieces.push(piece),
            None => {}
        }
    }
}
