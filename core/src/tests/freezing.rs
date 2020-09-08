#[cfg(test)]
mod freezing {
    use crate::board::Board;

    use crate::pieces::vector3::Vector3;
    use crate::player::Player;

    use crate::tests::assert_grid::assert_grid;

    use crate::pieces::basilisk::Basilisk;
    use crate::pieces::oliphant::Oliphant;
    use std::collections::HashMap;

    const PLAYER1: Player = Player::new(1);
    const PLAYER2: Player = Player::new(2);

    #[test]
    fn freezes_opponent() {
        let v = Vector3::new(5, 5, 1);
        let oliphant = Oliphant::new(v, PLAYER1);
        let basilisk = Basilisk::new(Vector3::new(5, 5, 0), PLAYER2);
        let mut board = Board::new_specified(
            vec![Box::new(oliphant), Box::new(basilisk)],
            PLAYER1,
            PLAYER2,
        );
        let oliphant = board.board_piece(v).unwrap();
        let moves = oliphant.possible_moves();
        assert_grid(&moves, HashMap::new());
    }
}
