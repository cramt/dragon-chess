#[cfg(test)]
mod checkmate {
    use crate::board::Board;

    use crate::player::Player;
    use crate::vector3::Vector3;

    use crate::board::CheckStatus::{Check, CheckMate, Free};

    use crate::pieces::dragon::Dragon;
    use crate::pieces::dwarf::Dwarf;
    use crate::pieces::king::King;
    use crate::pieces::oliphant::Oliphant;
    use crate::pieces::Piece;

    const PLAYER1: Player = Player::new(1);
    const PLAYER2: Player = Player::new(2);

    #[test]
    fn free() {
        let board = Board::new_specified(
            vec![Box::new(King::new(Vector3::new(5, 5, 1), PLAYER1))],
            PLAYER1,
            PLAYER2,
        );
        assert_eq!(board.get_check_status(PLAYER1), Free);
    }

    #[test]
    fn checked() {
        let board = Board::new_specified(
            vec![
                Box::new(King::new(Vector3::new(5, 5, 1), PLAYER1)),
                Box::new(Oliphant::new(Vector3::new(5, 1, 1), PLAYER2)),
            ],
            PLAYER1,
            PLAYER2,
        );
        assert_eq!(board.get_check_status(PLAYER1), Check);
    }

    #[test]
    fn checkmated() {
        let board = Board::new_specified(
            vec![
                Box::new(King::new(Vector3::new(5, 5, 1), PLAYER1)),
                Box::new(Oliphant::new(Vector3::new(5, 1, 1), PLAYER2)),
                Box::new(Oliphant::new(Vector3::new(4, 1, 1), PLAYER2)),
                Box::new(Oliphant::new(Vector3::new(6, 1, 1), PLAYER2)),
                Box::new(Dragon::new(Vector3::new(4, 4, 2), PLAYER2)),
                Box::new(Dwarf::new(Vector3::new(6, 6, 0), PLAYER2)),
            ],
            PLAYER1,
            PLAYER2,
        );
        assert_eq!(board.get_check_status(PLAYER1), CheckMate);
    }
}
