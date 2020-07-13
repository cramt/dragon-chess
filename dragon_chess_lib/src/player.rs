#[derive(Copy, Clone, Hash)]
pub struct Player {
    id: u64
}

impl Player {
    pub const fn new(id: u64) -> Player {
        Player { id }
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
