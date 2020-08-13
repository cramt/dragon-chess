use std::vec::Vec;
use crate::pieces::vector3::Vector3;


#[derive(Clone)]
pub struct MoveSetArg {
    pub(crate) directions: Vec<Vector3>,
    pub(crate) repeated: bool,
    pub(crate) mirrored_x: bool,
    pub(crate) mirrored_y: bool,
    pub(crate) remote: bool,
}

impl std::default::Default for MoveSetArg {
    fn default() -> Self {
        Self {
            directions: Vec::new(),
            repeated: false,
            mirrored_y: false,
            mirrored_x: false,
            remote: false,
        }
    }
}

impl MoveSetArg {
    pub(crate) fn build(self) -> MoveSet {
        let mut directions = self.directions;
        let remote = self.remote;
        let repeated = self.repeated;
        if self.mirrored_y {
            directions.append(&mut directions.clone().into_iter().map(|mut v| {
                v.x *= -1;
                v
            }).collect::<Vec<Vector3>>());
        }
        if self.mirrored_x {
            directions.append(&mut directions.clone().into_iter().map(|mut v| {
                v.y *= -1;
                v
            }).collect::<Vec<Vector3>>());
        }
        MoveSet {
            directions,
            remote,
            repeated,
        }
    }
}

pub struct MoveSetBuilder {
    directions: Vec<Vector3>,
    repeated: bool,
    mirrored_x: bool,
    mirrored_y: bool,
    remote: bool,
}

impl MoveSetBuilder {
    pub fn new() -> MoveSetBuilder {
        MoveSetBuilder {
            directions: Vec::new(),
            repeated: false,
            mirrored_y: false,
            mirrored_x: false,
            remote: false,
        }
    }
    pub fn direction(mut self, direction: Vector3) -> Self {
        self.directions.push(direction);
        self
    }
    pub fn directions(mut self, directions: Vec<Vector3>) -> Self {
        self.directions = directions;
        self
    }
    pub fn repeated(mut self) -> Self {
        self.repeated = true;
        self
    }
    pub fn mirrored_y(mut self) -> Self {
        self.mirrored_y = true;
        self
    }
    pub fn mirrored_x(mut self) -> Self {
        self.mirrored_x = true;
        self
    }
    pub fn mirrored(mut self) -> Self {
        self.mirrored_x = true;
        self.mirrored_y = true;
        self
    }
    pub fn remote(mut self) -> Self {
        self.remote = true;
        self
    }
    pub fn build(self) -> MoveSet {
        MoveSetArg {
            directions: self.directions,
            repeated: self.repeated,
            mirrored_y: self.mirrored_y,
            mirrored_x: self.mirrored_x,
            remote: self.remote,
        }.build()
    }
}

#[derive(Clone, Debug)]
pub struct MoveSet {
    pub(crate) directions: Vec<Vector3>,
    pub(crate) repeated: bool,
    pub(crate) remote: bool,
}
