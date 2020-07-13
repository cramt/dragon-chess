#[cfg(test)]
mod griffon {

    use crate::pieces::vector3::Vector3;
    use crate::player::Player;
    use crate::board::Board;
    use crate::tests::assert_grid::assert_grid;
    use maplit::*;
    use crate::board::MoveType::{Move};
    use crate::pieces::dragon::Dragon;

    const PLAYER1: Player = Player::new(1);
    const PLAYER2: Player = Player::new(1);

    #[test]
    fn basic_move() {
        let dragon = Dragon::new(Vector3::new(7, 3, 2), PLAYER1);
        let board = Board::new_specified(vec![Box::new(dragon)]);
        let dragon = board.get_pieces()[0].clone();
        let moves = board.possible_moves(dragon);

        assert_grid(&moves, hashmap! {
            //line top left to bottom right
            Vector3::new(3, 7, 2) => Move,
            Vector3::new(4, 6, 2) => Move,
            Vector3::new(5, 5, 2) => Move,
            Vector3::new(6, 4, 2) => Move,
            //dragons own position (7, 3, 2), and therefore empty
            Vector3::new(8, 2, 2) => Move,
            Vector3::new(9, 1, 2) => Move,
            Vector3::new(10, 0, 2) => Move,

            //line top right to bottom left
            Vector3::new(11, 7, 2) => Move,
            Vector3::new(10, 6, 2) => Move,
            Vector3::new(9, 5, 2) => Move,
            Vector3::new(8, 4, 2) => Move,
            //dragons own position (7, 3, 2), and therefore empty
            Vector3::new(6, 2, 2) => Move,
            Vector3::new(5, 1, 2) => Move,
            Vector3::new(4, 0, 2) => Move,

            //front back and sides
            Vector3::new(8, 3, 2) => Move,
            Vector3::new(6, 3, 2) => Move,
            Vector3::new(7, 2, 2) => Move,
            Vector3::new(7, 4, 2) => Move,
        })
    }
}
