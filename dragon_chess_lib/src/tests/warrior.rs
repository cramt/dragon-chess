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
    const PLAYER2: Player = Player::new(1);

    #[test]
    fn basic_move() {
        let warrior = Warrior::new(Vector3::new(5, 5, 1), PLAYER1);
        let board = Board::new_specified(vec![Box::new(warrior)]);
        let warrior = board.get_pieces()[0].clone();
        let moves = board.possible_moves(warrior);
        assert_grid(&moves, hashmap! {
            Vector3::new(5, 6, 1) => Move,
        });
    }

    #[test]
    fn basic_capture() {
        let warrior = Warrior::new(Vector3::new(5, 5, 1), PLAYER1);
        let warrior2 = Warrior::new(Vector3::new(6, 6, 1), PLAYER2);
        let board = Board::new_specified(vec![Box::new(warrior), Box::new(warrior2)]);
        let warrior = board.get_pieces()[0].clone();
        let moves = board.possible_moves(warrior);
        assert_grid(&moves, hashmap! {
            Vector3::new(5, 6, 1) => Move,
            Vector3::new(6, 6, 1) => Capture,
        });
    }
}
