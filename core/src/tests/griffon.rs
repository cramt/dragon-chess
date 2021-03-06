#[cfg(test)]
mod griffon {
    use crate::board::Board;
    use crate::board::MoveType::Move;
    use crate::pieces::griffon::Griffon;
    use crate::pieces::Piece;
    use crate::player::Player;
    use crate::tests::assert_grid::assert_grid;
    use crate::vector3::Vector3;
    use maplit::*;

    const PLAYER1: Player = Player::new(1);
    const PLAYER2: Player = Player::new(2);

    #[test]
    fn basic_move_from_upper() {
        let v = Vector3::new(5, 3, 2);
        let griffon = Griffon::new(v, PLAYER1);
        let mut board = Board::new_specified(vec![Box::new(griffon)], PLAYER1, PLAYER2);
        let griffon = board.board_piece(v).unwrap();
        let moves = griffon.possible_moves();

        assert_grid(
            &moves,
            hashmap! {
                //starts at bottom left (chess position 1e), and moves around counter clockwise
                Vector3::new(4, 0, 2) => Move,
                Vector3::new(6, 0, 2) => Move,
                Vector3::new(8, 2, 2) => Move,
                Vector3::new(8, 4, 2) => Move,
                Vector3::new(6, 6, 2) => Move,
                Vector3::new(4, 6, 2) => Move,
                Vector3::new(2, 4, 2) => Move,
                Vector3::new(2, 2, 2) => Move,


                Vector3::new(6, 4, 1) => Move,
                Vector3::new(4, 4, 1) => Move,
                Vector3::new(6, 2, 1) => Move,
                Vector3::new(4, 2, 1) => Move,
            },
        )
    }

    #[test]
    fn basic_move_from_middle() {
        let v = Vector3::new(5, 3, 1);
        let griffon = Griffon::new(v, PLAYER1);
        let mut board = Board::new_specified(vec![Box::new(griffon)], PLAYER1, PLAYER2);
        let griffon = board.board_piece(v).unwrap();
        let moves = griffon.possible_moves();
        assert_grid(
            &moves,
            hashmap! {
                Vector3::new(6, 4, 1) => Move,
                Vector3::new(4, 4, 1) => Move,
                Vector3::new(6, 2, 1) => Move,
                Vector3::new(4, 2, 1) => Move,
                Vector3::new(6, 4, 2) => Move,
                Vector3::new(4, 4, 2) => Move,
                Vector3::new(6, 2, 2) => Move,
                Vector3::new(4, 2, 2) => Move,
            },
        )
    }
}
