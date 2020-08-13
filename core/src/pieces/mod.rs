pub mod dummy;
pub mod elemental;
pub mod basilisk;
pub mod dwarf;
pub mod paladin;

use crate::pieces::vector3::Vector3;
use std::vec::Vec;

pub mod king;
pub mod mage;
pub mod cleric;
pub mod thief;
pub mod hero;
pub mod unicorn;
pub mod oliphant;
pub mod warrior;
pub mod dragon;
pub mod vector3;
pub mod move_set;
pub mod sylph;
pub mod griffon;


use crate::player::Player;

use crate::pieces::move_set::MoveSet;

pub trait Piece {
    fn get_position(&self) -> &Vector3;

    fn set_position(&mut self, pos: Vector3);

    fn get_player(&self) -> &Player;

    fn set_player(&mut self, player: Player);

    fn move_directions(&self) -> Vec<MoveSet>;

    fn capture_directions(&self) -> Vec<MoveSet>;

    fn get_name(&self) -> &str;

    fn get_char(&self) -> char {
        self.get_name().to_uppercase().chars().collect::<Vec<char>>()[0]
    }

    fn promote(&self) -> Option<Box<dyn Piece>> {
        None
    }

    fn freeze_zone(&self) -> Option<Vec<Vector3>> {
        None
    }

    fn is_king(&self) -> bool {
        false
    }

    fn internal_clone(&self) -> Box<dyn Piece>;
}

impl std::fmt::Debug for dyn Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(format!("Piece({})", self.get_name()).as_str())
            .field("position", self.get_position())
            .finish()
    }
}

impl PartialEq for dyn Piece {
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name() && self.get_position() == other.get_position()
    }
}

impl std::clone::Clone for Box<dyn Piece> {
    fn clone(&self) -> Self {
        self.internal_clone()
    }
}
