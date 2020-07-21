use crate::pieces::Piece;
use crate::board::{Board, MoveType};
use crate::grid::Grid;
use crate::pieces::vector3::Vector3;


pub struct BoardPiece<'a> {
    piece: Vector3,
    board: &'a mut Board,
}

impl<'a> BoardPiece<'a> {
    pub fn new(pos: Vector3, board: &'a mut Board) -> Option<BoardPiece::<'a>> {
        match board.grid[&pos].as_ref() {
            Some(_piece) => {
                Some(BoardPiece::<'a> {
                    piece: pos,
                    board,
                })
            }
            None => None
        }
    }

    pub fn get_piece(&self) -> &Box<dyn Piece> {
        self.board.grid[&self.piece].as_ref().unwrap()
    }

    pub fn possible_moves(&self) -> Grid<Option<MoveType>> {
        self.board.possible_moves(self.get_piece())
    }

    pub fn move_piece(&mut self, position: Vector3) -> Result<(), &str> {
        let possible_moves = self.possible_moves();
        self.move_piece_with_moves(position, possible_moves)
    }

    pub fn move_piece_with_moves(&mut self, position: Vector3, possible_moves: Grid<Option<MoveType>>) -> Result<(), &str> {
        let result = self.board.move_piece(self.piece, position, possible_moves);
        if result.is_ok() {
            self.piece = position;
        }
        result
    }
}
