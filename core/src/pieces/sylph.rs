use crate::move_set::{MoveSet, MoveSetBuilder};
use crate::pieces::Piece;
use crate::vector3::Vector3;

use crate::player::Player;

pub struct Sylph {
    position: Vector3,
    player: Player,
}

impl Piece for Sylph {
    fn new(position: Vector3, player: Player) -> Sylph {
        Sylph { position, player }
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
        if self.position.z != 2 {
            let sylph_standard_positions = vec![
                Vector3 { x: 0, y: 1, z: 2 },
                Vector3 { x: 2, y: 1, z: 2 },
                Vector3 { x: 4, y: 1, z: 2 },
                Vector3 { x: 6, y: 1, z: 2 },
                Vector3 { x: 8, y: 1, z: 2 },
                Vector3 { x: 10, y: 1, z: 2 },
            ]
            .into_iter()
            .map(|v| v - self.position)
            .collect::<Vec<Vector3>>();
            let mut moveable = sylph_standard_positions;
            moveable.push(Vector3::up());
            let r = vec![MoveSetBuilder::new().directions(moveable).build()];
            return r;
        }
        vec![MoveSetBuilder::new()
            .direction(Vector3::new(1, 1, 0))
            .mirrored_y()
            .build()]
    }

    fn capture_directions(&self) -> Vec<MoveSet> {
        if self.position.z != 2 {
            return vec![];
        }
        vec![MoveSetBuilder::new()
            .direction(Vector3::new(0, 1, 0))
            .direction(Vector3::new(0, 0, -1))
            .build()]
    }

    fn get_name(&self) -> &str {
        "sylph"
    }

    fn internal_clone(&self) -> Box<dyn Piece> {
        Box::new(Sylph::new(self.position, self.player))
    }
}
