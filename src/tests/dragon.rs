#[cfg(test)]
mod griffon {
    use crate::pieces::vector3::Vector3;
    use crate::player::Player;
    use crate::board::Board;
    use crate::tests::assert_grid::assert_grid;
    use maplit::*;
    use crate::board::MoveType::{Move, RemoteCapture};
    use crate::pieces::dragon::Dragon;
    use crate::pieces::warrior::Warrior;

    const PLAYER1: Player = Player::new(1);
    const PLAYER2: Player = Player::new(2);

    #[wasm_bindgen_test::wasm_bindgen_test]
    #[test]
    fn basic_move() {
        let v = Vector3::new(7, 3, 2);
        let dragon = Dragon::new(v, PLAYER1);
        let mut board = Board::new_specified(vec![Box::new(dragon)], PLAYER1, PLAYER2);
        let dragon = board.board_piece(v).unwrap();
        let moves = dragon.possible_moves();

        assert_grid(&moves, hashmap! {
            //line top left to bottom right
            Vector3::new(3, 7, 2) => Move,
            Vector3::new(4, 6, 2) => Move,
            Vector3::new(5, 5, 2) => Move,
            Vector3::new(6, 4, 2) => Move,
            //dragons own position (7, 3, 2), and therefore empty
            Vector3::new(8, 2, 2) => Move,
            Vector3::new(9, 1, 2) => Move,
            Vector3::new(10, 0, 2) => Move,

            //line top right to bottom left
            Vector3::new(11, 7, 2) => Move,
            Vector3::new(10, 6, 2) => Move,
            Vector3::new(9, 5, 2) => Move,
            Vector3::new(8, 4, 2) => Move,
            //dragons own position (7, 3, 2), and therefore empty
            Vector3::new(6, 2, 2) => Move,
            Vector3::new(5, 1, 2) => Move,
            Vector3::new(4, 0, 2) => Move,

            //front back and sides
            Vector3::new(8, 3, 2) => Move,
            Vector3::new(6, 3, 2) => Move,
            Vector3::new(7, 2, 2) => Move,
            Vector3::new(7, 4, 2) => Move,
        })
    }

    #[wasm_bindgen_test::wasm_bindgen_test]
    #[test]
    fn basic_remote_capture() {
        let v = Vector3::new(7, 3, 2);
        let dragon = Dragon::new(v, PLAYER1);
        let mut board = Board::new_specified(vec![Box::new(dragon), Box::new(Warrior::new(Vector3::new(7, 3, 1), PLAYER2))], PLAYER1, PLAYER2);
        let dragon = board.board_piece(v).unwrap();
        let moves = dragon.possible_moves();

        assert_grid(&moves, hashmap! {
            //line top left to bottom right
            Vector3::new(3, 7, 2) => Move,
            Vector3::new(4, 6, 2) => Move,
            Vector3::new(5, 5, 2) => Move,
            Vector3::new(6, 4, 2) => Move,
            //dragons own position (7, 3, 2), and therefore empty
            Vector3::new(8, 2, 2) => Move,
            Vector3::new(9, 1, 2) => Move,
            Vector3::new(10, 0, 2) => Move,

            //line top right to bottom left
            Vector3::new(11, 7, 2) => Move,
            Vector3::new(10, 6, 2) => Move,
            Vector3::new(9, 5, 2) => Move,
            Vector3::new(8, 4, 2) => Move,
            //dragons own position (7, 3, 2), and therefore empty
            Vector3::new(6, 2, 2) => Move,
            Vector3::new(5, 1, 2) => Move,
            Vector3::new(4, 0, 2) => Move,

            //front back and sides
            Vector3::new(8, 3, 2) => Move,
            Vector3::new(6, 3, 2) => Move,
            Vector3::new(7, 2, 2) => Move,
            Vector3::new(7, 4, 2) => Move,

            //remote capture
            Vector3::new(7, 3, 1) => RemoteCapture
        })
    }
}
