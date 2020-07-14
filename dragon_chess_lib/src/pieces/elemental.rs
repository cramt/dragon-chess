use crate::pieces::vector3::Vector3;
use crate::pieces::Piece;
use crate::pieces::move_set::{MoveSet, MoveSetBuilder};

use crate::player::Player;

pub struct Elemental {
    position: Vector3,
    player: Player,
}

impl Piece for Elemental {
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
        if self.position.z != 0 {
            return self.move_and_cap_above();
        }
        vec![
            MoveSetBuilder::new()
                .direction(Vector3::new(1, 1, 0))
                .mirrored()
                .build(),
            self.move_and_cap()
        ]
    }

    fn capture_directions(&self) -> Vec<MoveSet> {
        if self.position.z != 0 {
            return self.move_and_cap_above();
        }
        vec![
            MoveSetBuilder::new()
                .direction(Vector3::new(1, 0, 1))
                .direction(Vector3::new(0, 1, 1))
                .mirrored()
                .build(),
            self.move_and_cap()
        ]
    }

    fn get_name(&self) -> &str {
        "elemental"
    }
}

impl Elemental {
    pub fn new(position: Vector3, player: Player) -> Elemental {
        Elemental {
            position,
            player,
        }
    }

    fn move_and_cap(&self) -> MoveSet {
        MoveSetBuilder::new()
            .direction(Vector3::new(1, 0, 0))
            .direction(Vector3::new(0, 1, 0))
            .direction(Vector3::new(2, 0, 0))
            .direction(Vector3::new(0, 2, 0))
            .mirrored()
            .build()
    }

    fn move_and_cap_above(&self) -> Vec<MoveSet> {
        vec![
            MoveSetBuilder::new()
                .direction(Vector3::new(0, 0, -1))
                .build()
        ]
    }
}
