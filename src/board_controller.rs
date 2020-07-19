use crate::board::{Board, MoveType, CheckStatus};
use crate::pieces::vector3::Vector3;
use crate::pieces::Piece;
use crate::grid::Grid;
use crate::board_controller::PieceColor::{White, Black};
use std::collections::HashMap;
use crate::player::Player;
use core::fmt;
use crate::board::CheckStatus::Free;


#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    pub fn flip(&self) -> PieceColor {
        match self {
            White => Black,
            Black => White
        }
    }
}

impl fmt::Display for PieceColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

pub struct BoardController {
    board: Board,
    selected: Option<Vector3>,
    turn: PieceColor,
    check_mate: CheckStatus
}

impl BoardController {
    pub fn default() -> BoardController {
        BoardController {
            board: Board::new_default(),
            selected: None,
            turn: White,
            check_mate: Free,
        }
    }
    pub fn reset(&mut self) {
        self.board = Board::new_default();
    }
    pub fn check_mate(&self) -> (PieceColor, CheckStatus){
        (self.turn, self.check_mate)
    }
    pub fn piece_info(&self, position: Vector3) -> Option<(&Box<dyn Piece>, Grid<Option<MoveType>>)> {
        match self.board.grid[&position].as_ref() {
            Some(piece) => Some((piece, self.board.possible_moves(piece))),
            None => None
        }
    }
    pub fn pieces_info(&self) -> Vec<&Box<dyn Piece>> {
        self.board.grid.flat().into_iter().filter(|x| x.is_some()).map(|x| x.as_ref().unwrap()).collect()
    }
    fn get_current_player(&self) -> Player {
        if self.turn == White {
            self.board.white
        } else {
            self.board.black
        }
    }
    pub fn select(&mut self, position: Vector3) {
        if let Some(selected) = self.selected {
            if self.piece_info(selected).unwrap().1[&position].is_some() {
                let mut piece = self.board.board_piece(selected).unwrap();
                piece.move_piece(position);
                self.turn = self.turn.flip();
            }
            self.check_mate = self.board.get_check_status(self.get_current_player());
            self.selected = None;
            return;
        };
        self.selected = match self.board.grid[&position].as_ref() {
            Some(piece) => {
                if self.color_of_piece(piece) == self.turn {
                    Some(position)
                } else {
                    None
                }
            }
            None => None
        };
    }
    pub fn possible_moves(&self) -> HashMap<Vector3, MoveType> {
        match self.selected {
            None => HashMap::new(),
            Some(position) => match self.piece_info(position) {
                None => HashMap::new(),
                Some(piece) => piece.1.flat_with_index_owned()
                    .into_iter()
                    .filter(|(v, m)| m.is_some())
                    .map(|(v, m)| (v, m.unwrap()))
                    .collect::<HashMap<Vector3, MoveType>>()
            }
        }
    }
    pub fn color_of_piece(&self, piece: &Box<dyn Piece>) -> PieceColor {
        if self.board.white == *piece.get_player() {
            White
        } else if self.board.black == *piece.get_player() {
            Black
        } else {
            panic!("piece is neither black or white")
        }
    }
}
