use crate::pieces::vector3::Vector3;
use crate::pieces::Piece;
use crate::pieces::move_set::{MoveSet, MoveSetBuilder};

use crate::player::Player;

pub struct Mage {
    position: Vector3,
    player: Player,
}

impl Piece for Mage {
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
        if self.position.z != 1 {
            vec.push(
                MoveSetBuilder::new()
                    .mirrored()
                    .direction(Vector3::new(1, 1, 0))
                    .build()
            )
        }
        vec
    }

    fn capture_directions(&self) -> Vec<MoveSet> {
        let mut vec = vec![
            MoveSetBuilder::new()
                .mirrored()
                .repeated()
                .direction(Vector3::new(0, 0, 1))
                .direction(Vector3::new(0, 0, -1)).build()
        ];
        if self.position.z == 1 {
            vec.push(
                MoveSetBuilder::new()
                    .repeated()
                    .mirrored()
                    .direction(Vector3::new(1, 1, 0))
                    .direction(Vector3::new(1, 0, 0))
                    .direction(Vector3::new(0, 1, 0))
                    .build()
            )
        }
        vec
    }

    fn get_name(&self) -> &str {
        "mage"
    }

    fn clone(&self) -> Box<dyn Piece> {
        Box::new(Mage::new(self.position, self.player))
    }
}

impl Mage {
    pub fn new(position: Vector3, player: Player) -> Mage {
        Mage {
            position,
            player,
        }
    }
}
