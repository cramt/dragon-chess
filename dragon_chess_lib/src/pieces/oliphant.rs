use std::default::Default;

use crate::pieces::move_set::{MoveSet, MoveSetArg};
use crate::pieces::Piece;
use crate::pieces::vector3::Vector3;
use crate::player::Player;

pub struct Oliphant {
    position: Vector3,
    player: Player,
}

impl Piece for Oliphant {
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
        vec![MoveSetArg {
            directions: vec![
                Vector3::new(0, 1, 0),
                Vector3::new(1, 0, 0)
            ],
            mirrored_y: true,
            mirrored_x: true,
            repeated: true,
            ..Default::default()
        }.build()]
    }

    fn capture_directions(&self) -> Vec<MoveSet> {
        self.move_directions()
    }

    fn get_name(&self) -> &str {
        "oliphant"
    }

    fn get_char(&self) -> char {
        'O'
    }

    fn promote(&self) -> Option<Box<dyn Piece>> {
        None
    }
}

impl Oliphant {
    pub fn new(position: Vector3, player: Player) -> Oliphant {
        Oliphant {
            position,
            player,
        }
    }
}