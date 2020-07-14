use crate::pieces::vector3::Vector3;
use crate::pieces::Piece;
use crate::pieces::move_set::{MoveSet, MoveSetBuilder};

use crate::player::Player;

pub struct Thief {
    position: Vector3,
    player: Player,
}

impl Piece for Thief {
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
            .direction(Vector3::new(1,1,0))
            .mirrored()
            .repeated()
            .build()]
    }

    fn capture_directions(&self) -> Vec<MoveSet> {
        self.move_directions()
    }

    fn get_name(&self) -> &str {
        "thief"
    }
}

impl Thief {
    pub fn new(position: Vector3, player: Player) -> Thief {
        Thief {
            position,
            player,
        }
    }
}
