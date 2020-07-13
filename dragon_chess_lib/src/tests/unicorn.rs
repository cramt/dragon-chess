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
        let unicorn = Unicorn::new(Vector3::new(5, 5, 1), PLAYER1);
        let board = Board::new_specified(vec![Box::new(unicorn)]);
        let unicorn = board.get_pieces()[0];
        let moves = board.possible_moves(unicorn);
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
