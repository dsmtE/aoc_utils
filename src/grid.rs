use crate::cartesian::{Point2, p2, Cartesian2};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grid <T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

pub fn parse_char_grid(input: &str) -> Grid<char> {
    parse_grid::<char>(input, |c| c)
}

pub fn parse_grid<T>(input: &str, mapping: impl Fn(char) -> T) -> Grid<T> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let map = input.lines().flat_map(|line| line.chars()).map(mapping).collect::<Vec<_>>();
    return Grid { data: map, width, height };
}

impl<T> std::ops::Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, idx: (usize, usize)) -> &T {
        &self.data[idx.1 * self.width + idx.0]
    }
}

impl<T> std::ops::Index<usize> for Grid<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &T {
        &self.data[idx]
    }
}

impl<T> std::ops::Index<&Point2> for Grid<T> {
    type Output = T;
    fn index(&self, pos: &Point2) -> &T {
        &self.data[pos.y() as usize * self.width + pos.x() as usize]
    }
}

impl<T> std::ops::IndexMut<&Point2> for Grid<T> {
    fn index_mut(&mut self, pos: &Point2) -> &mut Self::Output {
        &mut self.data[pos.y() as usize * self.width + pos.x() as usize]
    }
}

impl <T> Grid<T> {
    pub fn index_to_point(&self, index: usize) -> Point2 {
        index_to_point(index, self.width)
    }
    pub fn in_bound(&self, pos: &Point2) -> bool {
        in_bound(pos, self.width, self.height)
    }

    pub fn same_size_with<U: Copy>(&self, value: U) -> Grid<U> {
        Grid {
            width: self.width,
            height: self.height,
            data: vec![value; (self.width * self.height) as usize],
        }
    }
}

pub fn index_to_point(index: usize, width: usize) -> Point2 {
    let width = width as i64;
    let index = index as i64;
    p2(index % width, index / width)
}

pub fn in_bound(pos: &Point2, width: usize, height: usize) -> bool {
    pos.x() >= 0 && pos.x() < width as i64 && pos.y() >= 0 && pos.y() < height as i64
}

pub fn in_bound_cardinal_neighbors_index(index: usize, width: usize, height: usize) -> Vec<usize> {
    let mut cardinal_neighbors = Vec::new();
    let (x, y) = (index % width, index / width);
    if x > 0 {
        cardinal_neighbors.push(index - 1);
    }
    if x < width - 1 {
        cardinal_neighbors.push(index + 1);
    }
    if y > 0 {
        cardinal_neighbors.push(index - width);
    }
    if y < height - 1 {
        cardinal_neighbors.push(index + width);
    }
    cardinal_neighbors
}