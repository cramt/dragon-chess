#[cfg(test)]
mod warrior {
    use crate::board::{Board};

    use crate::pieces::vector3::Vector3;
    use crate::player::Player;

    use crate::board::MoveType::{Move, Capture};
    use crate::tests::assert_grid::assert_grid;

    use maplit::*;
    use crate::pieces::warrior::Warrior;

    const PLAYER1: Player = Player::new(1);
    const PLAYER2: Player = Player::new(2);


    #[test]
    fn basic_move() {
        let v = Vector3::new(5, 5, 1);
        let warrior = Warrior::new(v, PLAYER1);
        let mut board = Board::new_specified(vec![Box::new(warrior)], PLAYER1, PLAYER2);
        let warrior = board.board_piece(v).unwrap();
        let moves = warrior.possible_moves();
        assert_grid(&moves, hashmap! {
            Vector3::new(5, 6, 1) => Move,
        });
    }


    #[test]
    fn basic_capture() {
        let v = Vector3::new(5, 5, 1);
        let warrior = Warrior::new(v, PLAYER1);
        let warrior2 = Warrior::new(Vector3::new(6, 6, 1), PLAYER2);
        let mut board = Board::new_specified(vec![Box::new(warrior), Box::new(warrior2)], PLAYER1, PLAYER2);
        let warrior = board.board_piece(v).unwrap();
        let moves = warrior.possible_moves();
        assert_grid(&moves, hashmap! {
            Vector3::new(5, 6, 1) => Move,
            Vector3::new(6, 6, 1) => Capture,
        });
    }
}
