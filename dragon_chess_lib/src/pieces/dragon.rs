use crate::pieces::Piece;
use crate::pieces::move_set::{MoveSetArg, MoveSet};
use crate::pieces::vector3::Vector3;
use std::default::Default;
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
        lazy_static! {
            static ref R: Vec<MoveSet> = vec![MoveSetArg {
                directions: vec![
                    Vector3::new(1, 1, 0),
                ],
                mirrored_x: true,
                mirrored_y: true,
                repeated: true,
                ..Default::default()
            }.build(), MoveSetArg {
                directions: vec![
                    Vector3::new(1, 0, 0),
                    Vector3::new(0, 1, 0),
                ],
                mirrored_x: true,
                mirrored_y: true,
                ..Default::default()
            }.build()];
        }
        return R.to_vec();
    }

    fn capture_directions(&self) -> Vec<MoveSet> {
        let mut move_dir = self.move_directions();
        move_dir.push(MoveSetArg {
            directions: vec![
                Vector3::new(0, 0, -1),
                Vector3::new(1, 1, -1),
            ],
            mirrored_x: true,
            mirrored_y: true,
            remote: true,
            ..Default::default()
        }.build());
        move_dir
    }

    fn get_name(&self) -> &str {
        "dragon"
    }

    fn get_char(&self) -> char {
        'R'
    }

    fn promote(&self) -> Option<Box<dyn Piece>> {
        None
    }
}

impl Dragon {
    pub fn new(position: Vector3, player: Player) -> Dragon {
        Dragon {
            position,
            player,
        }
    }
}
