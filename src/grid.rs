#[derive(Debug, PartialEq, Eq)]
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