use std::{fmt::Display, ops::{Add, AddAssign, Mul, Neg, Sub}};

use crate::core::common::direction::Direction;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x : isize,
    pub y : isize,
}
impl Neg for Coordinate {
    type Output = Coordinate;

    fn neg(self) -> Self::Output {
        let x = -self.x;
        let y = -self.y;

        Self {
            x,
            y
        }
    }
}
impl Add::<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Coordinate) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;

        Self {
            x,
            y
        }
    }
}

impl Sub::<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: Coordinate) -> Self::Output {
        let neg = -rhs;
        let result = self + neg;
        result
    }
}


impl AddAssign::<Coordinate> for Coordinate {
    fn add_assign(&mut self, rhs: Coordinate) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}


impl Mul::<isize> for Coordinate {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;

        Self::new(x, y)
    }
}


impl From<&Direction> for Coordinate {
    fn from(value : &Direction) -> Self {
        match value {
            Direction::North => Coordinate::new(0, -1),
            Direction::East => Coordinate::new(1, 0),
            Direction::South => Coordinate::new(0, 1),
            Direction::West => Coordinate::new(-1, 0),
        }
    }
}


impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_string = format!("({},{})", self.x, self.y);
        f.write_str(&as_string)?;

        Ok(())
    }
}

impl<'a> Coordinate {
    pub fn new(x : isize, y : isize) -> Self {
        Self { x, y }
    }


    pub fn right() -> Coordinate {
        Self::new(1, 0)
    }


    pub fn left() -> Coordinate {
        Self::new(-1, 0)
    }


    pub fn down() -> Coordinate {
        Self::new(0, -1)
    }


    pub fn up() -> Coordinate {
        Self::new(0, 1)
    }


    pub fn try_from_value(value : usize, map_dim_x : usize) -> Option<Self> {
        let div = value / map_dim_x;
        let rem = value % map_dim_x;

        let x = isize::try_from(rem).ok()?;
        let y = isize::try_from(div).ok()?;

        let result = Self {
            x,
            y,
        };

        Some(result)
    }


    pub fn try_to_index(&self, map_dim_x : usize, map_dim_y : usize) -> Option<usize> {
        let x = usize::try_from(self.x).ok()?;
        let y = usize::try_from(self.y).ok()?;

        if x >= map_dim_x || y >= map_dim_y {
            return None;
        }

        let index = y * map_dim_x + x;

        Some(index)
    }


    pub fn manhattan_distance(&self) -> usize {
        let x : usize = self.x.abs().try_into().unwrap();
        let y : usize = self.y.abs().try_into().unwrap();

        x + y
    }


    pub fn rotate_right(&mut self) {
        let x = -self.y;
        let y = self.x;

        self.x = x;
        self.y = y;
    }


    pub fn rotate_left(&mut self) {
        let x = self.y;
        let y = -self.x;

        self.x = x;
        self.y = y;
    }


    pub fn abs(self) -> Self {
        let x = self.x.abs();
        let y = self.y.abs();

        let c = Self::new(x, y);

        c
    }


    pub fn times_to_rotate_right_to_become_target(mut self, other : &Coordinate) -> Option<usize> {
        for i in 0..4 {
            if &self == other {
                return Some(i);
            }

            self.rotate_right();
        }

        None
    }


    pub fn get_coordinates_within_distance(&self, distance : usize) -> Vec<Coordinate> {
        let reach = isize::try_from(distance).unwrap();

        let mut coordinates = Vec::new();

        for x in -reach..=reach {
            for y in -reach..=reach {
                let vector = Coordinate::new(x, y);
                let length = vector.manhattan_distance();
                if length <= distance {
                    let target = self.clone() + vector;
                    coordinates.push(target);
                }
            }
        }

        coordinates
    }


    pub fn to_debug_coordinate(&'a self) -> DebugCoordinate<'a> {
        DebugCoordinate {
            coordinate : self,
        }
    }
}


// IHasCoordinates

pub trait IHasCoordinates {
    type Item;

    fn get_index_and_content_by_coordinate(&self, coordinate : &Coordinate) -> Option<(usize, &Self::Item)>;
}


// DebugCoordinate

pub struct DebugCoordinate<'a> {
    coordinate : &'a Coordinate,
}


impl<'a> Display for DebugCoordinate<'a> {
    fn fmt(&self, f : &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line = self.coordinate.y + 1;
        let col = self.coordinate.x + 1;

        write!(f, "(Ln {}, Col {})", &line, &col)
    }
}
