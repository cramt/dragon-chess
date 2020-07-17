use crate::pieces::Piece;
use crate::grid::Grid;
use crate::pieces::vector3::Vector3;
use crate::grid::IndexValid::{DefaultValue, NonDefaultValue, OutOfBounds};
use crate::board::MoveType::{Move, Capture, RemoteCapture};

use crate::board_piece::BoardPiece;
use crate::pieces::move_set::MoveSet;
use crate::pieces::griffon::Griffon;
use crate::player::Player;
use crate::pieces::sylph::Sylph;
use crate::pieces::dragon::Dragon;
use crate::pieces::warrior::Warrior;
use crate::pieces::oliphant::Oliphant;
use crate::pieces::unicorn::Unicorn;
use crate::pieces::hero::Hero;
use crate::pieces::thief::Thief;
use crate::pieces::mage::Mage;
use crate::pieces::cleric::Cleric;
use crate::pieces::king::King;
use crate::pieces::paladin::Paladin;
use crate::pieces::dwarf::Dwarf;
use crate::pieces::basilisk::Basilisk;
use crate::pieces::elemental::Elemental;
use crate::pieces::dummy::Dummy;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum MoveType {
    Capture,
    RemoteCapture,
    Move,
}

pub struct Board {
    white: Player,
    black: Player,
    pub grid: Grid<Option<Box<dyn Piece>>>,
    dead_pieces: Vec<Box<dyn Piece>>,
}

impl Board {
    pub fn new_specified(pieces: Vec<Box<dyn Piece>>, white: Player, black: Player) -> Board {
        Board {
            white,
            black,
            grid: Board::build_state(pieces),
            dead_pieces: vec![],
        }
    }

    pub fn new_default_with_players(white: Player, black: Player) -> Board {
        Board::new_specified(vec![
            //griffon white
            Box::new(Griffon::new(Vector3::new(2, 0, 2), white)),
            Box::new(Griffon::new(Vector3::new(10, 0, 2), white)),
            //griffon black
            Box::new(Griffon::new(Vector3::new(2, 7, 2), black)),
            Box::new(Griffon::new(Vector3::new(10, 7, 2), black)),
            //sylph white
            Box::new(Sylph::new(Vector3::new(0, 1, 2), white)),
            Box::new(Sylph::new(Vector3::new(2, 1, 2), white)),
            Box::new(Sylph::new(Vector3::new(4, 1, 2), white)),
            Box::new(Sylph::new(Vector3::new(6, 1, 2), white)),
            Box::new(Sylph::new(Vector3::new(8, 1, 2), white)),
            Box::new(Sylph::new(Vector3::new(10, 1, 2), white)),
            //sylph black
            Box::new(Sylph::new(Vector3::new(0, 6, 2), black)),
            Box::new(Sylph::new(Vector3::new(2, 6, 2), black)),
            Box::new(Sylph::new(Vector3::new(4, 6, 2), black)),
            Box::new(Sylph::new(Vector3::new(6, 6, 2), black)),
            Box::new(Sylph::new(Vector3::new(8, 6, 2), black)),
            Box::new(Sylph::new(Vector3::new(10, 6, 2), black)),
            //dragons
            Box::new(Dragon::new(Vector3::new(6, 0, 2), white)),
            Box::new(Dragon::new(Vector3::new(6, 7, 2), black)),
            //warrior white
            Box::new(Warrior::new(Vector3::new(0, 1, 1), white)),
            Box::new(Warrior::new(Vector3::new(1, 1, 1), white)),
            Box::new(Warrior::new(Vector3::new(2, 1, 1), white)),
            Box::new(Warrior::new(Vector3::new(3, 1, 1), white)),
            Box::new(Warrior::new(Vector3::new(4, 1, 1), white)),
            Box::new(Warrior::new(Vector3::new(5, 1, 1), white)),
            Box::new(Warrior::new(Vector3::new(6, 1, 1), white)),
            Box::new(Warrior::new(Vector3::new(7, 1, 1), white)),
            Box::new(Warrior::new(Vector3::new(8, 1, 1), white)),
            Box::new(Warrior::new(Vector3::new(9, 1, 1), white)),
            Box::new(Warrior::new(Vector3::new(10, 1, 1), white)),
            Box::new(Warrior::new(Vector3::new(11, 1, 1), white)),
            //warrior black
            Box::new(Warrior::new(Vector3::new(0, 6, 1), black)),
            Box::new(Warrior::new(Vector3::new(1, 6, 1), black)),
            Box::new(Warrior::new(Vector3::new(2, 6, 1), black)),
            Box::new(Warrior::new(Vector3::new(3, 6, 1), black)),
            Box::new(Warrior::new(Vector3::new(4, 6, 1), black)),
            Box::new(Warrior::new(Vector3::new(5, 6, 1), black)),
            Box::new(Warrior::new(Vector3::new(6, 6, 1), black)),
            Box::new(Warrior::new(Vector3::new(7, 6, 1), black)),
            Box::new(Warrior::new(Vector3::new(8, 6, 1), black)),
            Box::new(Warrior::new(Vector3::new(9, 6, 1), black)),
            Box::new(Warrior::new(Vector3::new(10, 6, 1), black)),
            Box::new(Warrior::new(Vector3::new(11, 6, 1), black)),
            //oliphant white
            Box::new(Oliphant::new(Vector3::new(0, 0, 1), white)),
            Box::new(Oliphant::new(Vector3::new(11, 0, 1), white)),
            //oliphant black
            Box::new(Oliphant::new(Vector3::new(0, 7, 1), black)),
            Box::new(Oliphant::new(Vector3::new(11, 7, 1), black)),
            //unicorn white
            Box::new(Unicorn::new(Vector3::new(1, 0, 1), white)),
            Box::new(Unicorn::new(Vector3::new(10, 0, 1), white)),
            //unicorn black
            Box::new(Unicorn::new(Vector3::new(1, 7, 1), black)),
            Box::new(Unicorn::new(Vector3::new(10, 7, 1), black)),
            //hero white
            Box::new(Hero::new(Vector3::new(2, 0, 1), white)),
            Box::new(Hero::new(Vector3::new(9, 0, 1), white)),
            //hero black
            Box::new(Hero::new(Vector3::new(2, 7, 1), black)),
            Box::new(Hero::new(Vector3::new(9, 7, 1), black)),
            //thief white
            Box::new(Thief::new(Vector3::new(3, 0, 1), white)),
            Box::new(Thief::new(Vector3::new(8, 0, 1), white)),
            //thief black
            Box::new(Thief::new(Vector3::new(3, 7, 1), black)),
            Box::new(Thief::new(Vector3::new(8, 7, 1), black)),
            //clerics
            Box::new(Cleric::new(Vector3::new(4, 0, 1), white)),
            Box::new(Cleric::new(Vector3::new(4, 7, 1), black)),
            //mages
            Box::new(Mage::new(Vector3::new(5, 0, 1), white)),
            Box::new(Mage::new(Vector3::new(5, 7, 1), black)),
            //kings
            Box::new(King::new(Vector3::new(6, 0, 1), white)),
            Box::new(King::new(Vector3::new(6, 7, 1), black)),
            //paladins
            Box::new(Paladin::new(Vector3::new(7, 0, 1), white)),
            Box::new(Paladin::new(Vector3::new(7, 7, 1), black)),
            //dwarf white
            Box::new(Dwarf::new(Vector3::new(1, 1, 0), white)),
            Box::new(Dwarf::new(Vector3::new(3, 1, 0), white)),
            Box::new(Dwarf::new(Vector3::new(5, 1, 0), white)),
            Box::new(Dwarf::new(Vector3::new(7, 1, 0), white)),
            Box::new(Dwarf::new(Vector3::new(9, 1, 0), white)),
            Box::new(Dwarf::new(Vector3::new(11, 1, 0), white)),
            //dwarf black
            Box::new(Dwarf::new(Vector3::new(1, 6, 0), black)),
            Box::new(Dwarf::new(Vector3::new(3, 6, 0), black)),
            Box::new(Dwarf::new(Vector3::new(5, 6, 0), black)),
            Box::new(Dwarf::new(Vector3::new(7, 6, 0), black)),
            Box::new(Dwarf::new(Vector3::new(9, 6, 0), black)),
            Box::new(Dwarf::new(Vector3::new(11, 6, 0), black)),
            //basilisk white
            Box::new(Basilisk::new(Vector3::new(2, 0, 0), white)),
            Box::new(Basilisk::new(Vector3::new(10, 0, 0), white)),
            //basilisk black
            Box::new(Basilisk::new(Vector3::new(2, 7, 0), black)),
            Box::new(Basilisk::new(Vector3::new(10, 7, 0), black)),
            //elementals
            Box::new(Elemental::new(Vector3::new(6, 0, 0), white)),
            Box::new(Elemental::new(Vector3::new(6, 7, 0), black)),
        ], white, black)
    }

    pub fn new_default() -> Board {
        let white = Player::new(1);
        let black = Player::new(2);
        Board::new_default_with_players(white, black)
    }

    pub(self) fn build_state(pieces: Vec<Box<dyn Piece>>) -> Grid<Option<Box<dyn Piece>>> {
        let mut state = Grid::new();
        for p in pieces {
            let position = p.get_position().clone();
            state[&position] = Some(p);
        }
        state
    }

    pub fn board_piece(&mut self, pos: Vector3) -> Option<BoardPiece> {
        BoardPiece::new(pos, self)
    }

    pub fn possible_moves(&self, piece: &Box<dyn Piece>) -> Grid<Option<MoveType>> {
        let freeze_zone = self.enemy_freeze_zone(piece.get_player());
        if freeze_zone[piece.get_position()].is_some() {
            return Grid::new();
        }
        let mut moves = Grid::new();
        let move_dirs = self.fix_directions(piece.move_directions(), piece.get_player());
        let cap_dirs = self.fix_directions(piece.capture_directions(), piece.get_player());
        self.unwrap_from_move_dirs(move_dirs, *piece.get_position(), &mut moves);
        self.unwrap_from_capture_dirs(cap_dirs, *piece.get_position(), piece.get_player(), &mut moves);
        if piece.is_king() {
            let taking_candidate = self.grid.flat().into_iter()
                .filter(|x| x.is_some())
                .map(|x| x.as_ref().unwrap())
                .filter(|x| x.get_player() != piece.get_player())
                .map(|x| self.possible_capture(x))
                .fold(vec![], |mut acc, mut x| {
                    acc.append(&mut x);
                    acc
                });
            for candidate in taking_candidate {
                moves[&candidate] = None
            }
        }
        moves
    }

    fn fix_directions(&self, move_set: Vec<MoveSet>, player: &Player) -> Vec<MoveSet> {
        if self.black == *player {
            move_set.into_iter().map(|mut x| {
                x.directions = x.directions.into_iter().map(|mut v| {
                    v.y *= -1;
                    v
                }).collect();
                x
            }).collect()
        } else {
            move_set
        }
    }

    fn enemy_freeze_zone(&self, player: &Player) -> Grid<Option<()>> {
        self.grid.flat().iter()
            .filter(|x| x.is_some())
            .map(|x| x.as_ref().unwrap())
            .filter(|x| x.get_player() != player)
            .filter(|x| x.freeze_zone().is_some())
            .map(|x| (x.freeze_zone().unwrap(), *x.get_position()))
            .fold(Grid::new(), |mut acc, (dirs, pos)| {
                let mut grid = Grid::new();
                for dir in dirs {
                    let abs = dir + pos;
                    grid[&abs] = Some(());
                }
                acc.concat(grid);
                acc
            })
    }

    fn unwrap_from_capture_dirs(&self, cap_dirs: Vec<MoveSet>, piece_position: Vector3, player: &Player, moves: &mut Grid<Option<MoveType>>) {
        for cap_dir in cap_dirs {
            if cap_dir.repeated {
                for dir in &cap_dir.directions {
                    let mut curr = piece_position + *dir;
                    loop {
                        match self.grid.valid(&curr) {
                            OutOfBounds => break,
                            DefaultValue => {}
                            NonDefaultValue => {
                                if self.grid[&curr].as_ref().unwrap().get_player() != player {
                                    moves[&curr] = Some(Capture);
                                }
                                break;
                            }
                        };
                        curr = curr + *dir;
                    }
                }
            } else {
                let dir = cap_dir.directions.into_iter().map(|v| v + piece_position).collect::<Vec<Vector3>>();
                for dir in dir {
                    if self.grid.valid(&dir) == NonDefaultValue && self.grid[&dir].as_ref().unwrap().get_player() != player {
                        moves[&dir] = Some(if cap_dir.remote { RemoteCapture } else { Capture });
                    }
                }
            }
        }
    }

    fn unwrap_from_move_dirs(&self, move_dirs: Vec<MoveSet>, piece_position: Vector3, moves: &mut Grid<Option<MoveType>>) {
        for move_dir in move_dirs {
            if move_dir.repeated {
                for dir in &move_dir.directions {
                    let mut curr = piece_position + *dir;
                    loop {
                        if self.grid.valid(&curr) != DefaultValue {
                            break;
                        }
                        moves[&curr] = Some(Move);
                        curr = curr + *dir;
                    }
                }
            } else if move_dir.remote {
                panic!("you cant move remote");
            } else {
                let dir = move_dir.directions.into_iter().map(|v| v + piece_position).collect::<Vec<Vector3>>();
                for dir in dir {
                    if self.grid.valid(&dir) == DefaultValue {
                        moves[&dir] = Some(Move);
                    }
                }
            }
        }
    }

    fn possible_capture(&self, piece: &Box<dyn Piece>) -> Vec<Vector3> {
        let dirs = self.fix_directions(piece.capture_directions(), piece.get_player());
        let mut v = vec![];
        for dir in dirs {
            if dir.repeated {
                for d in &dir.directions {
                    let mut curr = *piece.get_position() + *d;
                    loop {
                        if self.grid.valid(&curr) == DefaultValue {
                            v.push(curr);
                            curr = curr + *d;
                        }
                        else {
                            break;
                        }
                    }
                }
            } else {
                for d in &dir.directions {
                    let d = *piece.get_position() + *d;
                    if self.grid.valid(&d) == DefaultValue {
                        v.push(d);
                    }
                }
            }
        }
        v
    }

    pub fn move_piece(&mut self, from: Vector3, to: Vector3, possible_moves: Grid<Option<MoveType>>) -> Result<(), &str> {
        match &possible_moves[&to] {
            Some(move_type) => {
                let move_type = *move_type;
                if move_type != Move {
                    self.kill_piece(&to);
                }
                if move_type != RemoteCapture {
                    self.change_position(&from, &to);
                }
            }
            None => {
                return Err("not a possible move");
            }
        };
        Ok(())
    }

    fn change_position(&mut self, from_position: &Vector3, to_position: &Vector3) {
        let from = self.grid.swap(from_position, None);
        let to = self.grid.swap(to_position, None);
        if to.is_some() {
            let mut piece = to.unwrap();
            piece.set_position(*from_position);
            //piece.set_position(*from_position);
            self.grid.swap(from_position, Some(piece));
        }
        if from.is_some() {
            let mut piece = from.unwrap();
            piece.set_position(*to_position);
            //piece.set_position(*to_position);
            self.grid.swap(to_position, Some(piece));
        }
    }

    pub fn kill_piece(&mut self, position: &Vector3) {
        match self.grid.swap(position, None) {
            Some(piece) => self.dead_pieces.push(piece),
            None => {}
        }
    }
}
