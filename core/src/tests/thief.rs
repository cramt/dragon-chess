#[cfg(test)]
mod thief {
    use crate::board::Board;

    use crate::player::Player;
    use crate::vector3::Vector3;

    use crate::board::MoveType::Move;
    use crate::tests::assert_grid::assert_grid;

    use crate::pieces::thief::Thief;
    use crate::pieces::Piece;
    use maplit::*;

    const PLAYER1: Player = Player::new(1);
    const PLAYER2: Player = Player::new(2);

    #[test]
    fn basic_move() {
        let v = Vector3::new(5, 5, 1);
        let thief = Thief::new(v, PLAYER1);
        let mut board = Board::new_specified(vec![Box::new(thief)], PLAYER1, PLAYER2);
        let thief = board.board_piece(v).unwrap();
        let moves = thief.possible_moves();
        assert_grid(
            &moves,
            hashmap! {
                //top left to bottom right
                Vector3::new(3, 7, 1) => Move,
                Vector3::new(4, 6, 1) => Move,
                //thief position
                Vector3::new(6, 4, 1) => Move,
                Vector3::new(7, 3, 1) => Move,
                Vector3::new(8, 2, 1) => Move,
                Vector3::new(9, 1, 1) => Move,
                Vector3::new(10, 0, 1) => Move,

                //top right to bottom left
                Vector3::new(7, 7, 1) => Move,
                Vector3::new(6, 6, 1) => Move,
                //thief position
                Vector3::new(4, 4, 1) => Move,
                Vector3::new(3, 3, 1) => Move,
                Vector3::new(2, 2, 1) => Move,
                Vector3::new(1, 1, 1) => Move,
                Vector3::new(0, 0, 1) => Move,
            },
        );
    }
}
