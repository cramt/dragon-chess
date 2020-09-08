use crate::pieces::move_set::{MoveSet, MoveSetBuilder};
use crate::pieces::vector3::Vector3;
use crate::pieces::Piece;

use crate::player::Player;

pub struct Dragon {
    position: Vector3,
    player: Player,
}

impl Piece for Dragon {
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
        vec![
            MoveSetBuilder::new()
                .direction(Vector3::new(1, 1, 0))
                .mirrored()
                .repeated()
                .build(),
            MoveSetBuilder::new()
                .direction(Vector3::new(1, 0, 0))
                .direction(Vector3::new(0, 1, 0))
                .mirrored()
                .build(),
        ]
    }

    fn capture_directions(&self) -> Vec<MoveSet> {
        let mut move_dir = self.move_directions();
        move_dir.push(
            MoveSetBuilder::new()
                .mirrored()
                .remote()
                .direction(Vector3::new(0, 0, -1))
                .direction(Vector3::new(1, 1, -1))
                .build(),
        );
        move_dir
    }

    fn get_name(&self) -> &str {
        "dragon"
    }

    fn get_char(&self) -> char {
        'R'
    }

    fn internal_clone(&self) -> Box<dyn Piece> {
        Box::new(Dragon::new(self.position, self.player))
    }
}

impl Dragon {
    pub fn new(position: Vector3, player: Player) -> Dragon {
        Dragon { position, player }
    }
}
