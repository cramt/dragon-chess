use std::default::Default;

use crate::pieces::move_set::{MoveSet, MoveSetArg};
use crate::pieces::Piece;
use crate::pieces::vector3::Vector3;
use crate::player::Player;
use crate::pieces::hero::Hero;

pub struct Warrior {
    position: Vector3,
    player: Player,
}

impl Piece for Warrior {
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
            directions: vec![Vector3::new(0, 1, 0)],
            ..Default::default()
        }.build()]
    }

    fn capture_directions(&self) -> Vec<MoveSet> {
        vec![MoveSetArg {
            directions: vec![Vector3::new(1, 1, 0)],
            mirrored_y: true,
            ..Default::default()
        }.build()]
    }

    fn get_name(&self) -> &str {
        "warrior"
    }

    fn get_char(&self) -> char {
        'W'
    }

    fn promote(&self) -> Option<Box<dyn Piece>> {
        Some(Box::new(Hero::new(self.position, self.player)))
    }
}

impl Warrior {
    pub fn new(position: Vector3, player: Player) -> Warrior {
        Warrior {
            position,
            player,
        }
    }
}
