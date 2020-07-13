use crate::pieces::vector3::Vector3;
use crate::pieces::Piece;
use crate::pieces::move_set::{MoveSet, MoveSetBuilder};

use crate::player::Player;

pub struct Dwarf {
    position: Vector3,
    player: Player,
}

impl Piece for Dwarf {
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
        let mut vec = vec![MoveSetBuilder::new()
            .direction(Vector3::new(1, 0, 0))
            .direction(Vector3::new(0, 1, 0))
            .mirrored_y()
            .build()];
        if self.position.z == 1 {
            vec.push(
                MoveSetBuilder::new()
                    .direction(Vector3::new(0, 0, -1))
                    .build()
            )
        }
        vec
    }

    fn capture_directions(&self) -> Vec<MoveSet> {
        let mut vec = vec![MoveSetBuilder::new()
            .direction(Vector3::new(1, 1, 0))
            .mirrored_y()
            .build()];
        if self.position.z == 0 {
            vec.push(
                MoveSetBuilder::new()
                    .direction(Vector3::new(0, 0, 1))
                    .build()
            )
        }
        vec
    }

    fn get_name(&self) -> &str {
        "dwarf"
    }

    fn promote(&self) -> Option<Box<dyn Piece>> {
        None
    }
}

impl Dwarf {
    pub fn new(position: Vector3, player: Player) -> Dwarf {
        Dwarf {
            position,
            player,
        }
    }
}
