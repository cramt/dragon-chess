#[cfg(test)]
mod unicorn {
    use crate::board::{Board};

    use crate::pieces::vector3::Vector3;
    use crate::player::Player;

    use crate::board::MoveType::{Move};
    use crate::tests::assert_grid::assert_grid;

    use maplit::*;

    use crate::pieces::unicorn::Unicorn;

    const PLAYER1: Player = Player::new(1);
    const PLAYER2: Player = Player::new(1);

    #[test]
    fn basic_move() {
        let v = Vector3::new(5, 5, 1);
        let unicorn = Unicorn::new(v, PLAYER1);
        let board = Board::new_specified(vec![Box::new(unicorn)]);
        let unicorn = board.board_piece(v).unwrap();
        let moves = unicorn.possible_moves();
        assert_grid(&moves, hashmap! {
            Vector3::new(4, 7, 1) => Move,
            Vector3::new(6, 7, 1) => Move,
            Vector3::new(7, 6, 1) => Move,
            Vector3::new(7, 4, 1) => Move,
            Vector3::new(6, 3, 1) => Move,
            Vector3::new(4, 3, 1) => Move,
            Vector3::new(3, 4, 1) => Move,
            Vector3::new(3, 6, 1) => Move,
        });
    }
}
