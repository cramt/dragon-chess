use crate::board::{Board, MoveType};
use crate::pieces::vector3::Vector3;
use crate::pieces::Piece;
use crate::grid::Grid;
use wasm_bindgen::__rt::std::collections::HashMap;

pub struct BoardController {
    board: Board,
    selected: Option<Vector3>,
}

impl BoardController {
    pub fn default() -> BoardController {
        BoardController {
            board: Board::new_default(),
            selected: None,
        }
    }
    pub fn reset(&mut self) {
        self.board = Board::new_default();
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
    pub fn select(&mut self, position: Vector3) {
        if let Some(selected) = self.selected {
            if self.piece_info(selected).unwrap().1[&position].is_some() {
                let mut piece = self.board.board_piece(selected).unwrap();
                piece.move_piece(position);
                self.selected = None;
                return;
            }
        };
        self.selected = match self.board.grid[&position].as_ref() {
            Some(_) => Some(position),
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
}
