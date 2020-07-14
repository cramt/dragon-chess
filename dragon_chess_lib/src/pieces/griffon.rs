use crate::pieces::Piece;
use crate::pieces::move_set::{MoveSetArg, MoveSet, MoveSetBuilder};
use crate::pieces::vector3::Vector3;
use std::default::Default;
use crate::player::Player;

pub struct Griffon {
    position: Vector3,
    player: Player,
}

impl Piece for Griffon {
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
        if self.position.z == 2 {
            return vec![
                MoveSetBuilder::new()
                    .direction(Vector3::new(3, 1, 0))
                    .direction(Vector3::new(1, 3, 0))
                    .direction(Vector3::new(1, 1, -1))
                    .mirrored()
                    .build()
            ];
        }
        vec![
            MoveSetBuilder::new()
                .mirrored()
                .direction(Vector3::new(1, 1, 0))
                .direction(Vector3::new(1, 1, 1))
                .build()
        ]
    }

    fn capture_directions(&self) -> Vec<MoveSet> {
        self.move_directions()
    }

    fn get_name(&self) -> &str {
        "griffon"
    }

    fn get_char(&self) -> char {
        'G'
    }

    fn promote(&self) -> Option<Box<dyn Piece>> {
        None
    }
}

impl Griffon {
    pub fn new(position: Vector3, player: Player) -> Griffon {
        Griffon {
            position,
            player,
        }
    }
}
