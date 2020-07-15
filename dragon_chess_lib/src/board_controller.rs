use crate::board::Board;

struct BoardController {
    board: Board
}

impl BoardController {
    pub fn default() -> BoardController {
        BoardController {
            board: Board::new_default()
        }
    }
}
