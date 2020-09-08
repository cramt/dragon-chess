use crate::move_set::MoveSet;
use crate::pieces::Piece;
use crate::vector3::Vector3;

use crate::player::Player;

pub struct Dummy {
    position: Vector3,
    player: Player,
}

impl Piece for Dummy {
    fn new(position: Vector3, player: Player) -> Dummy {
        Dummy { position, player }
    }

    fn get_position(&self) -> &Vector3 {
        return &self.position;
    }

    fn set_position(&mut self, pos: Vector3) {
        self.position = pos;
    }

    fn get_player(&self) -> &Player {
        return &self.player;
    }

    fn set_player(&mut self, player: Player) {
        self.player = player;
    }

    fn move_directions(&self) -> Vec<MoveSet> {
        vec![]
    }

    fn capture_directions(&self) -> Vec<MoveSet> {
        vec![]
    }

    fn get_name(&self) -> &str {
        "dummy"
    }

    fn get_char(&self) -> char {
        '_'
    }

    fn internal_clone(&self) -> Box<dyn Piece> {
        Box::new(Dummy::new(self.position, self.player))
    }
}
