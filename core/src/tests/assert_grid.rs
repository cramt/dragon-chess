use crate::grid::Grid;
use std::collections::HashMap;
use crate::pieces::vector3::Vector3;
use std::fmt::Debug;

pub fn assert_optional_grid<T>(grid: &Grid<T>, conditional_map: HashMap<Vector3, T>) where T: Default + PartialEq + Debug {
    let expression = Grid::from_map(conditional_map);
    let expression = &expression.array;
    let cond = &grid.array;
    assert_eq!(cond, expression)
}

pub fn assert_grid<T>(grid: &Grid<Option<T>>, conditional_map: HashMap<Vector3, T>) where T: PartialEq + Debug {
    let expression = Grid::from_map(conditional_map.into_iter().map(|(key, value)| (key, Some(value))).collect());
    let expression = &expression.array;
    let cond = &grid.array;
    assert_eq!(cond, expression)
}
