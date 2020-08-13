use std::ops::{Index, IndexMut};
use crate::pieces::vector3::Vector3;
use crate::grid::IndexValid::{OutOfBounds, NonDefaultValue, DefaultValue};
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum IndexValid {
    OutOfBounds,
    DefaultValue,
    NonDefaultValue,
}

#[derive(Debug, Clone)]
pub struct Grid<T> where T: Default + PartialEq {
    pub(crate) array: [[[T; 12]; 8]; 3]
}

impl<T> Grid<T> where T: Default + PartialEq {
    pub fn from_map(map: HashMap<Vector3, T>) -> Grid<T> {
        let mut grid = Grid::new();
        let map = map.into_iter().collect::<Vec<(Vector3, T)>>();
        for (v, t) in map {
            grid[&v] = t;
        }
        grid
    }
    pub fn new() -> Grid<T> {
        Grid {
            array: Default::default()
        }
    }

    pub fn flat(&self) -> Vec<&T> {
        self.flat_with_index().into_iter().map(|v| v.1).collect()
    }

    pub fn flat_with_index(&self) -> Vec<(Vector3, &T)> {
        let mut v = vec![];
        for x in 0..self.array.len() {
            for y in 0..self.array[x].len() {
                for z in 0..self.array[x][y].len() {
                    let ve = Vector3::new(z as i32, y as i32, x as i32);
                    let t = &self[&ve];
                    v.push((ve, t));
                }
            }
        }
        v
    }

    pub fn flat_with_index_owned(&self) -> Vec<(Vector3, T)> where T: Copy {
        let mut v = vec![];
        for x in 0..self.array.len() {
            for y in 0..self.array[x].len() {
                for z in 0..self.array[x][y].len() {
                    let ve = Vector3::new(z as i32, y as i32, x as i32);
                    let t = self[&ve];
                    v.push((ve, t));
                }
            }
        }
        v
    }

    pub(crate) fn valid(&self, index: &Vector3) -> IndexValid {
        if !self.within_bounds(index) {
            return OutOfBounds;
        }
        if self[index] == Default::default() {
            return DefaultValue;
        }
        NonDefaultValue
    }

    pub fn within_bounds(&self, index: &Vector3) -> bool {
        index.x > -1 && index.x < 12 && index.y > -1 && index.y < 8 && index.z > -1 && index.z < 3
    }

    pub fn swap(&mut self, position: &Vector3, mut value: T) -> T {
        std::mem::swap(&mut self[position], &mut value);
        value
    }

    pub fn swap_position(&mut self, from: &Vector3, to: &Vector3) {
        let f = self.swap(from, Default::default());
        let t = self.swap(to, Default::default());
        self.swap(to, f);
        self.swap(from, t);
    }

    pub fn concat(&mut self, other: Grid<T>) where T: Copy {
        for x in 0..self.array.len() {
            for y in 0..self.array[x].len() {
                for z in 0..self.array[x][y].len() {
                    if other.array[x][y][z] != Default::default() {
                        self.array[x][y][z] = other.array[x][y][z];
                    }
                }
            }
        }
    }
}

impl<T> Index<&'_ Vector3> for Grid<T> where T: Default + PartialEq {
    type Output = T;

    fn index(&self, index: &Vector3) -> &Self::Output {
        &self.array[index.z as usize][index.y as usize][index.x as usize]
    }
}

impl<T> IndexMut<&'_ Vector3> for Grid<T> where T: Default + PartialEq {
    fn index_mut(&mut self, index: &Vector3) -> &mut Self::Output {
        &mut self.array[index.z as usize][index.y as usize][index.x as usize]
    }
}
