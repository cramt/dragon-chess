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
    const PLAYER2: Player = Player::new(1);

    #[test]
    fn basic_move_from_upper() {
        let sylph = Sylph::new(Vector3::new(5, 5, 2), PLAYER1);
        let board = Board::new_specified(vec![Box::new(sylph)]);
        let sylph = board.get_pieces()[0].clone();
        let moves = board.possible_moves(sylph);
        assert_grid(&moves, hashmap! {
            Vector3::new(6, 6, 2) => Move,
            Vector3::new(4, 6, 2) => Move,
            Vector3::new(5, 5, 1) => Move,
        });
    }

    #[test]
    fn basic_move_from_middle() {
        let sylph = Sylph::new(Vector3::new(5, 5, 1), PLAYER1);
        let board = Board::new_specified(vec![Box::new(sylph)]);
        let sylph = board.get_pieces()[0].clone();
        let moves = board.possible_moves(sylph);
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
        let sylph = Sylph::new(Vector3::new(5, 5, 2), PLAYER1);
        let sylph2 = Sylph::new(Vector3::new(5, 6, 2), PLAYER2);
        let board = Board::new_specified(vec![Box::new(sylph), Box::new(sylph2)]);
        let sylph = board.get_pieces()[0].clone();
        let moves = board.possible_moves(sylph);
        assert_grid(&moves, hashmap! {
            Vector3::new(6, 6, 2) => Move,
            Vector3::new(4, 6, 2) => Move,
            Vector3::new(5, 5, 1) => Move,
            Vector3::new(5, 6, 2) => Capture,
        });
    }
}
