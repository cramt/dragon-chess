#[cfg(test)]
mod oliphant {
    use crate::board::{Board};

    use crate::pieces::vector3::Vector3;
    use crate::player::Player;

    use crate::board::MoveType::{Move};
    use crate::tests::assert_grid::assert_grid;

    use maplit::*;

    use crate::pieces::oliphant::Oliphant;

    const PLAYER1: Player = Player::new(1);
    const PLAYER2: Player = Player::new(2);

    #[wasm_bindgen_test::wasm_bindgen_test]
    #[test]
    fn basic_move() {
        let v = Vector3::new(5, 5, 1);
        let oliphant = Oliphant::new(v, PLAYER1);
        let mut board = Board::new_specified(vec![Box::new(oliphant)], PLAYER1, PLAYER2);
        let oliphant = board.board_piece(v).unwrap();
        let moves = oliphant.possible_moves();
        assert_grid(&moves, hashmap! {
            //up to down
            Vector3::new(5, 7, 1) => Move,
            Vector3::new(5, 6, 1) => Move,
            //oliphant position
            Vector3::new(5, 4, 1) => Move,
            Vector3::new(5, 3, 1) => Move,
            Vector3::new(5, 2, 1) => Move,
            Vector3::new(5, 1, 1) => Move,
            Vector3::new(5, 0, 1) => Move,

            //left to right
            Vector3::new(0, 5, 1) => Move,
            Vector3::new(1, 5, 1) => Move,
            Vector3::new(2, 5, 1) => Move,
            Vector3::new(3, 5, 1) => Move,
            Vector3::new(4, 5, 1) => Move,
            //oliphant position
            Vector3::new(6, 5, 1) => Move,
            Vector3::new(7, 5, 1) => Move,
            Vector3::new(8, 5, 1) => Move,
            Vector3::new(9, 5, 1) => Move,
            Vector3::new(10, 5, 1) => Move,
            Vector3::new(11, 5, 1) => Move,
        });
    }
}
