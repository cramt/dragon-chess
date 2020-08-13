#[cfg(test)]
mod sylph {
    use crate::board::{Board};
    use crate::pieces::sylph::Sylph;
    use crate::pieces::vector3::Vector3;
    use crate::player::Player;

    use crate::board::MoveType::{Move, Capture};
    use crate::tests::assert_grid::assert_grid;

    use maplit::*;

    const PLAYER1: Player = Player::new(1);
    const PLAYER2: Player = Player::new(2);


    #[test]
    fn basic_move_from_upper() {
        let v = Vector3::new(5, 5, 2);
        let sylph = Sylph::new(v, PLAYER1);
        let mut board = Board::new_specified(vec![Box::new(sylph)], PLAYER1, PLAYER2);
        let sylph = board.board_piece(v).unwrap();
        let moves = sylph.possible_moves();
        assert_grid(&moves, hashmap! {
            Vector3::new(6, 6, 2) => Move,
            Vector3::new(4, 6, 2) => Move,
        });
    }


    #[test]
    fn basic_move_from_middle() {
        let v = Vector3::new(5, 5, 1);
        let sylph = Sylph::new(v, PLAYER1);
        let mut board = Board::new_specified(vec![Box::new(sylph)], PLAYER1, PLAYER2);
        let sylph = board.board_piece(v).unwrap();
        let moves = sylph.possible_moves();
        assert_grid(&moves, hashmap! {
            Vector3::new(5, 5, 2) => Move,
            Vector3::new(0, 1, 2) => Move,
            Vector3::new(2, 1, 2) => Move,
            Vector3::new(4, 1, 2) => Move,
            Vector3::new(6, 1, 2) => Move,
            Vector3::new(8, 1, 2) => Move,
            Vector3::new(10, 1, 2) => Move,
        })
    }


    #[test]
    fn basic_capture() {
        let v = Vector3::new(5, 5, 2);
        let sylph = Sylph::new(v, PLAYER1);
        let sylph2 = Sylph::new(Vector3::new(5, 6, 2), PLAYER2);
        let mut board = Board::new_specified(vec![Box::new(sylph), Box::new(sylph2)], PLAYER1, PLAYER2);
        let sylph = board.board_piece(v).unwrap();
        let moves = sylph.possible_moves();
        assert_grid(&moves, hashmap! {
            Vector3::new(6, 6, 2) => Move,
            Vector3::new(4, 6, 2) => Move,
            Vector3::new(5, 6, 2) => Capture,
        });
    }
}
