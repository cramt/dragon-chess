use crate::pieces::move_set::{MoveSet, MoveSetBuilder};
use crate::pieces::vector3::Vector3;
use crate::pieces::Piece;

use crate::player::Player;

pub struct Basilisk {
    position: Vector3,
    player: Player,
}

impl Piece for Basilisk {
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
        let mut vec = self.capture_directions();
        vec.push(
            MoveSetBuilder::new()
                .direction(Vector3::new(0, -1, 0))
                .build(),
        );
        vec
    }

    fn capture_directions(&self) -> Vec<MoveSet> {
        vec![MoveSetBuilder::new()
            .mirrored_y()
            .direction(Vector3::new(1, 1, 0))
            .direction(Vector3::new(0, 1, 0))
            .build()]
    }

    fn get_name(&self) -> &str {
        "basilisk"
    }

    fn freeze_zone(&self) -> Option<Vec<Vector3>> {
        Some(vec![Vector3::new(0, 0, 1)])
    }

    fn internal_clone(&self) -> Box<dyn Piece> {
        Box::new(Basilisk::new(self.position, self.player))
    }
}

impl Basilisk {
    pub fn new(position: Vector3, player: Player) -> Basilisk {
        Basilisk { position, player }
    }
}
