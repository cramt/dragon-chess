

use crate::pieces::move_set::{MoveSet, MoveSetBuilder};
use crate::pieces::Piece;
use crate::pieces::vector3::Vector3;
use crate::player::Player;

pub struct Hero {
    position: Vector3,
    player: Player,
    old_middle_position: Vector3,
}

impl Piece for Hero {
    fn get_position(&self) -> &Vector3 {
        return &self.position;
    }

    fn set_position(&mut self, pos: Vector3) {
        if pos.z != 1 {
            self.old_middle_position = self.position;
        }
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
            if self.position.z != 1 {
                if self.old_middle_position == Vector3::min() {
                    panic!("the hero does not have a position to go back to")
                }
                MoveSetBuilder::new()
                    .direction(self.old_middle_position - self.position)
                    .build()
            } else {
                MoveSetBuilder::new()
                    .direction(Vector3::new(1, 1, 0))
                    .direction(Vector3::new(2, 2, 0))
                    .direction(Vector3::new(1, 1, 1))
                    .direction(Vector3::new(1, 1, -1))
                    .mirrored()
                    .build()
            }
        ]
    }

    fn capture_directions(&self) -> Vec<MoveSet> {
        self.move_directions()
    }

    fn get_name(&self) -> &str {
        "hero"
    }

    fn get_char(&self) -> char {
        'H'
    }

    fn promote(&self) -> Option<Box<dyn Piece>> {
        None
    }
}

impl Hero {
    pub fn new(position: Vector3, player: Player) -> Hero {
        Hero {
            position,
            player,
            old_middle_position: Vector3::min(),
        }
    }
}
