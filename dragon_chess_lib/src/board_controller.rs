use crate::board::{Board, MoveType};
use crate::pieces::vector3::Vector3;
use crate::pieces::Piece;
use crate::grid::Grid;

pub struct BoardController {
    board: Board
}

impl BoardController {
    pub fn default() -> BoardController {
        BoardController {
            board: Board::new_default()
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
}
