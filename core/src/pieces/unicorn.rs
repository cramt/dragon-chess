use crate::pieces::move_set::{MoveSet, MoveSetBuilder};
use crate::pieces::vector3::Vector3;
use crate::pieces::Piece;
use crate::player::Player;

pub struct Unicorn {
    position: Vector3,
    player: Player,
}

impl Piece for Unicorn {
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
        vec![MoveSetBuilder::new()
            .mirrored()
            .direction(Vector3::new(1, 2, 0))
            .direction(Vector3::new(2, 1, 0))
            .build()]
    }

    fn capture_directions(&self) -> Vec<MoveSet> {
        self.move_directions()
    }

    fn get_name(&self) -> &str {
        "unicorn"
    }

    fn internal_clone(&self) -> Box<dyn Piece> {
        Box::new(Unicorn::new(self.position, self.player))
    }
}

impl Unicorn {
    pub fn new(position: Vector3, player: Player) -> Unicorn {
        Unicorn { position, player }
    }
}
