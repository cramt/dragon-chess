use crate::pieces::vector3::Vector3;
use crate::pieces::Piece;
use crate::pieces::move_set::{MoveSet, MoveSetBuilder};

use crate::player::Player;

pub struct Cleric {
    position: Vector3,
    player: Player,
}

impl Piece for Cleric {
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
        vec![MoveSetBuilder::new()
            .direction(Vector3::new(1, 1, 0))
            .direction(Vector3::new(1, 0, 0))
            .direction(Vector3::new(0, 1, 0))
            .direction(Vector3::new(0, 0, 1))
            .direction(Vector3::new(1, 0, 1))
            .direction(Vector3::new(0, 1, 1))
            .direction(Vector3::new(0, 0, -1))
            .direction(Vector3::new(1, 0, -1))
            .direction(Vector3::new(0, 1, -1))
            .mirrored()
            .build()]
    }

    fn capture_directions(&self) -> Vec<MoveSet> {
        self.move_directions()
    }

    fn get_name(&self) -> &str {
        "cleric"
    }

    fn clone(&self) -> Box<dyn Piece> {
        Box::new(Cleric::new(self.position, self.player))
    }
}

impl Cleric {
    pub fn new(position: Vector3, player: Player) -> Cleric {
        Cleric {
            position,
            player,
        }
    }
}
