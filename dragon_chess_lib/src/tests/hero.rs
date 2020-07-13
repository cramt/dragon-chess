#[cfg(test)]
mod hero {
    use crate::board::Board;
    use crate::pieces::vector3::Vector3;
    use crate::player::Player;
    use crate::board::MoveType::{Move};
    use crate::tests::assert_grid::assert_grid;
    
    use maplit::*;
    
    use crate::pieces::hero::Hero;
    

    const PLAYER1: Player = Player::new(1);
    const PLAYER2: Player = Player::new(1);

    #[test]
    fn basic_move() {
        let hero = Hero::new(Vector3::new(5, 5, 1), PLAYER1);
        let board = Board::new_specified(vec![Box::new(hero)]);
        let hero = board.get_pieces()[0];
        let moves = board.possible_moves(hero);
        assert_grid(&moves, hashmap! {
            //top left to bottom right
            Vector3::new(3, 7, 1) => Move,
            Vector3::new(4, 6, 1) => Move,
            //hero position
            Vector3::new(6, 4, 1) => Move,
            Vector3::new(7, 3, 1) => Move,

            //top right to bottom left
            Vector3::new(7, 7, 1) => Move,
            Vector3::new(6, 6, 1) => Move,
            //hero position
            Vector3::new(4, 4, 1) => Move,
            Vector3::new(3, 3, 1) => Move,

            //above
            Vector3::new(6, 6, 2) => Move,
            Vector3::new(6, 4, 2) => Move,
            Vector3::new(4, 6, 2) => Move,
            Vector3::new(4, 4, 2) => Move,

            //bellow
            Vector3::new(6, 6, 0) => Move,
            Vector3::new(6, 4, 0) => Move,
            Vector3::new(4, 6, 0) => Move,
            Vector3::new(4, 4, 0) => Move,
        });
    }

    #[test]
    fn basic_move_back() {
        let hero = Hero::new(Vector3::new(5, 5, 1), PLAYER1);
        let board = Board::new_specified(vec![Box::new(hero)]);
        let _hero = board.get_pieces()[0];
        //board.move_piece(hero, Vector3::new(6,6,1));

    }
}
