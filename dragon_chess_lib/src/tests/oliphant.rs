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
    const PLAYER2: Player = Player::new(1);

    #[test]
    fn basic_move() {
        let oliphant = Oliphant::new(Vector3::new(5, 5, 1), PLAYER1);
        let board = Board::new_specified(vec![Box::new(oliphant)]);
        let oliphant = board.get_pieces()[0].clone();
        let moves = board.possible_moves(oliphant);
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
