use std::fmt::Display;

use glam::{i64vec2, I64Vec2};

static DIRECTIONS_4: [I64Vec2; 4] = [
    //LEFT
    i64vec2(-1, 0),
    //RIGHT
    i64vec2(1, 0),
    //DOWN
    i64vec2(0, -1),
    //UP
    i64vec2(0, 1),
];

static DIRECTIONS_8: [I64Vec2; 8] = [
    //LEFT
    i64vec2(-1, 0),
    //RIGHT
    i64vec2(1, 0),
    //DOWN
    i64vec2(0, -1),
    //UP
    i64vec2(0, 1),
    //UP LEFT
    i64vec2(-1, 1),
    //UP RIGHT
    i64vec2(1, 1),
    //DOWN LEFT
    i64vec2(-1, -1),
    //DOWN RIGHT
    i64vec2(1, -1),
];

pub struct Neighborhood {
    pub north: Option<I64Vec2>,
    pub east: Option<I64Vec2>,
    pub south: Option<I64Vec2>,
    pub west: Option<I64Vec2>,
    pub north_east: Option<I64Vec2>,
    pub north_west: Option<I64Vec2>,
    pub south_east: Option<I64Vec2>,
    pub south_west: Option<I64Vec2>,
}

impl Neighborhood {
    pub fn exising_neighbors(&self, include_diagonal: bool) -> Vec<I64Vec2> {
        let mut result = vec![];
        if let Some(north) = self.north {
            result.push(north);
        }
        if let Some(east) = self.east {
            result.push(east);
        }
        if let Some(south) = self.south {
            result.push(south);
        }
        if let Some(west) = self.west {
            result.push(west);
        }

        if !include_diagonal {
            return result;
        }

        if let Some(north_east) = self.north_east {
            result.push(north_east);
        }
        if let Some(north_west) = self.north_west {
            result.push(north_west);
        }
        if let Some(south_east) = self.south_east {
            result.push(south_east);
        }
        if let Some(south_west) = self.south_west {
            result.push(south_west);
        }
        result
    }
}

pub trait FilterGrid<T: std::marker::Copy = Self>: Grid<T> {
    fn filter_positions(&self, predicate: impl Fn(&T) -> bool) -> Vec<I64Vec2> {
        let mut result: Vec<I64Vec2> = vec![];
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pos = i64vec2(x as i64, y as i64);
                let item = self.get_position(pos);
                if predicate(item) {
                    result.push(pos);
                }
            }
        }
        result
    }
}

pub trait Grid<T: Copy> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn values(&self) -> &[T];
    fn values_mut(&mut self) -> &mut [T];

    fn get_index(&self, pos: I64Vec2) -> usize {
        let x = pos.x as usize;
        let y = pos.y as usize;
        x + y * self.width()
    }

    fn position_from_index(&self, index: usize) -> I64Vec2 {
        let x = index % self.width();
        let y = index / self.width();
        i64vec2(x as i64, y as i64)
    }

    fn try_get_index(&self, pos: I64Vec2) -> Option<usize> {
        if self.has_position(pos) {
            Some(self.get_index(pos))
        } else {
            None
        }
    }

    fn has_position(&self, pos: I64Vec2) -> bool {
        !(pos.x < 0 || pos.x >= self.width() as i64 || pos.y < 0 || pos.y >= self.height() as i64)
    }

    fn get_position(&self, pos: I64Vec2) -> &T {
        &self.values()[self.get_index(pos)]
    }

    fn get_position_mut(&mut self, pos: I64Vec2) -> &mut T {
        let index = self.get_index(pos);
        &mut self.values_mut()[index]
    }

    fn set_position(&mut self, pos: I64Vec2, value: T) {
        let index = self.get_index(pos);
        self.values_mut()[index] = value;
    }

    fn get_row(&self, row: usize) -> Vec<T> {
        let mut result: Vec<T> = vec![];
        let y = row;
        for x in 0..self.width() {
            result.push(*self.get_position(i64vec2(x as i64, y as i64)));
        }
        result
    }

    fn get_column(&self, column: usize) -> Vec<T> {
        let mut result: Vec<T> = vec![];
        let x = column;
        for y in 0..self.height() {
            result.push(*self.get_position(i64vec2(x as i64, y as i64)));
        }
        result
    }

    fn try_get_position(&self, pos: I64Vec2) -> Option<&T> {
        if self.has_position(pos) {
            Some(self.get_position(pos))
        } else {
            None
        }
    }

    fn get_neighbours(&self, pos: I64Vec2, include_diagonal: bool) -> Vec<I64Vec2> {
        match include_diagonal {
            true => DIRECTIONS_8
                .into_iter()
                .map(|d| pos + d)
                .filter(|p| self.has_position(*p))
                .collect(),
            false => DIRECTIONS_4
                .into_iter()
                .map(|d| pos + d)
                .filter(|p| self.has_position(*p))
                .collect(),
        }
    }

    fn get_neighborhood(&self, pos: I64Vec2) -> Neighborhood {
        Neighborhood {
            north: self
                .has_position(pos + i64vec2(0, -1))
                .then(|| pos + i64vec2(0, -1)),
            east: self
                .has_position(pos + i64vec2(1, 0))
                .then(|| pos + i64vec2(1, 0)),
            south: self
                .has_position(pos + i64vec2(0, 1))
                .then(|| pos + i64vec2(0, 1)),
            west: self
                .has_position(pos + i64vec2(-1, 0))
                .then(|| pos + i64vec2(-1, 0)),
            north_east: self
                .has_position(pos + i64vec2(1, -1))
                .then(|| pos + i64vec2(1, -1)),
            north_west: self
                .has_position(pos + i64vec2(-1, -1))
                .then(|| pos + i64vec2(-1, -1)),
            south_east: self
                .has_position(pos + i64vec2(1, 1))
                .then(|| pos + i64vec2(1, 1)),
            south_west: self
                .has_position(pos + i64vec2(-1, 1))
                .then(|| pos + i64vec2(-1, 1)),
        }
    }
}

impl<T: std::fmt::Display + std::marker::Copy> std::fmt::Debug for dyn Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pos = i64vec2(x as i64, y as i64);
                let item = self.get_position(pos);
                write!(f, "{}", item)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: std::fmt::Display + std::marker::Copy> Display for dyn Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pos = i64vec2(x as i64, y as i64);
                let item = self.get_position(pos);
                write!(f, "{}", item)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
