#[cfg(test)]
mod hero {
    use crate::board::Board;
    use crate::pieces::vector3::Vector3;
    use crate::player::Player;
    use crate::board::MoveType::{Move};
    use crate::tests::assert_grid::assert_grid;

    use maplit::*;

    use crate::pieces::hero::Hero;
    use crate::pieces::king::King;


    const PLAYER1: Player = Player::new(1);
    const PLAYER2: Player = Player::new(1);

    #[wasm_bindgen_test::wasm_bindgen_test]
    #[test]
    fn basic_move() {
        let v = Vector3::new(5, 5, 1);
        let king = King::new(v, PLAYER1);
        let mut board = Board::new_specified(vec![Box::new(king)]);
        let king = board.board_piece(v).unwrap();
        let moves = king.possible_moves();
        assert_grid(&moves, hashmap! {
            //clockwise starting at 6,5
            Vector3::new(6, 5, 1) => Move,
            Vector3::new(6, 6, 1) => Move,
            Vector3::new(5, 6, 1) => Move,
            Vector3::new(4, 6, 1) => Move,
            Vector3::new(4, 5, 1) => Move,
            Vector3::new(4, 4, 1) => Move,
            Vector3::new(5, 4, 1) => Move,
            Vector3::new(6, 4, 1) => Move,

            //above
            Vector3::new(5, 5, 2) => Move,

            //bellow
            Vector3::new(5, 5, 0) => Move,
        });
    }

    #[wasm_bindgen_test::wasm_bindgen_test]
    #[test]
    fn basic_move_back() {
        let v = Vector3::new(5, 5, 1);
        let king = King::new(v, PLAYER1);
        let mut board = Board::new_specified(vec![Box::new(king)]);
        let mut king = board.board_piece(v).unwrap();
        king.move_piece(Vector3::new(5, 5, 2));
        let moves = king.possible_moves();

        assert_grid(&moves, hashmap! {
            Vector3::new(5, 5, 1) => Move
        })
    }
}
