use std::default::Default;

use crate::move_set::{MoveSet, MoveSetArg};
use crate::pieces::hero::Hero;
use crate::pieces::Piece;
use crate::player::Player;
use crate::vector3::Vector3;

pub struct Warrior {
    position: Vector3,
    player: Player,
}

impl Piece for Warrior {
    fn new(position: Vector3, player: Player) -> Warrior {
        Warrior { position, player }
    }

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
        }
        .build()]
    }

    fn capture_directions(&self) -> Vec<MoveSet> {
        vec![MoveSetArg {
            directions: vec![Vector3::new(1, 1, 0)],
            mirrored_y: true,
            ..Default::default()
        }
        .build()]
    }

    fn get_name(&self) -> &str {
        "warrior"
    }

    fn promote(&self) -> Option<Box<dyn Piece>> {
        Some(Box::new(Hero::new(self.position, self.player)))
    }

    fn internal_clone(&self) -> Box<dyn Piece> {
        Box::new(Warrior::new(self.position, self.player))
    }
}
