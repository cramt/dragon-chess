use crate::move_set::{MoveSet, MoveSetBuilder};
use crate::pieces::Piece;
use crate::player::Player;
use crate::vector3::Vector3;

pub struct Oliphant {
    position: Vector3,
    player: Player,
}

impl Piece for Oliphant {
    fn new(position: Vector3, player: Player) -> Oliphant {
        Oliphant { position, player }
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
        vec![MoveSetBuilder::new()
            .direction(Vector3::new(1, 0, 0))
            .direction(Vector3::new(0, 1, 0))
            .mirrored()
            .repeated()
            .build()]
    }

    fn capture_directions(&self) -> Vec<MoveSet> {
        self.move_directions()
    }

    fn get_name(&self) -> &str {
        "oliphant"
    }

    fn internal_clone(&self) -> Box<dyn Piece> {
        Box::new(Oliphant::new(self.position, self.player))
    }
}
