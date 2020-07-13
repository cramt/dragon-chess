use crate::pieces::vector3::Vector3;
use crate::pieces::Piece;
use crate::pieces::move_set::{MoveSetArg, MoveSet};
use std::default::Default;
use crate::player::Player;

pub struct Sylph {
    position: Vector3,
    player: Player,
}

impl Piece for Sylph {
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
        if self.position.z != 2 {
            let sylph_standard_positions = vec![
                Vector3 { x: 0, y: 1, z: 2 },
                Vector3 { x: 2, y: 1, z: 2 },
                Vector3 { x: 4, y: 1, z: 2 },
                Vector3 { x: 6, y: 1, z: 2 },
                Vector3 { x: 8, y: 1, z: 2 },
                Vector3 { x: 10, y: 1, z: 2 },
            ].into_iter().map(|v| v - self.position).collect::<Vec<Vector3>>();
            let mut moveable = sylph_standard_positions;
            moveable.push(Vector3::up());
            let r = vec![MoveSetArg { directions: moveable, ..Default::default() }.build()];
            return r;
        }
        lazy_static! {
            static ref R: Vec<MoveSet> = vec![MoveSetArg { directions: vec![Vector3::lf(), Vector3::down()], mirrored_y: true, ..Default::default() }.build()];
        }
        return R.to_vec();
    }

    fn capture_directions(&self) -> Vec<MoveSet> {
        if self.position.z != 2 {
            lazy_static! {
                static ref R: Vec<MoveSet> = vec![];
            }
            return R.to_vec();
        }
        lazy_static! {
            static ref R: Vec<MoveSet> = vec![MoveSetArg { directions: vec![Vector3::down(), Vector3::forward()], ..Default::default() }.build()];
        }
        return R.to_vec();
    }

    fn get_name(&self) -> &str {
        "sylph"
    }

    fn get_char(&self) -> char {
        'S'
    }

    fn promote(&self) -> Option<Box<dyn Piece>> {
        None
    }
}

impl Sylph {
    pub fn new(position: Vector3, player: Player) -> Sylph {
        Sylph {
            position,
            player,
        }
    }
}
