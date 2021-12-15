use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Y(pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct X(pub usize);

#[derive(Debug)]
pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
}

impl<T> Grid<T> {
    pub fn from_iter<I>(iter: I, width: usize) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Grid::new(iter.into_iter().collect(), width)
    }

    pub fn new(data: Vec<T>, width: usize) -> Self {
        assert_eq!(data.len() % width, 0);
        Grid { data, width }
    }

    pub fn height(&self) -> usize {
        self.data.len() / self.width
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn num_cells(&self) -> usize {
        self.data.len()
    }

    pub fn neighbors_4(&self, x: X, y: Y) -> AdjacentIter {
        AdjacentIter {
            x,
            y,
            height: self.data.len() / self.width,
            width: self.width,
            states: NEIGHBORS_8[0..4].iter().copied(),
        }
    }

    pub fn neighbors_diag(&self, x: X, y: Y) -> AdjacentIter {
        AdjacentIter {
            x,
            y,
            height: self.data.len() / self.width,
            width: self.width,
            states: NEIGHBORS_8[4..8].iter().copied(),
        }
    }

    pub fn neighbors_8(&self, x: X, y: Y) -> AdjacentIter {
        AdjacentIter {
            x,
            y,
            height: self.data.len() / self.width,
            width: self.width,
            states: NEIGHBORS_8.iter().copied(),
        }
    }

    pub fn points_values(&self) -> impl Iterator<Item = ((X, Y), &T)> + '_ {
        self.data.iter().enumerate().map(|(idx, v)| {
            let x = X(idx % self.width);
            let y = Y(idx / self.width);
            ((x, y), v)
        })
    }
}

impl<T> Index<(X, Y)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (X, Y)) -> &Self::Output {
        &self.data[y.0 * self.width + x.0]
    }
}

impl<T> IndexMut<(X, Y)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (X, Y)) -> &mut Self::Output {
        &mut self.data[y.0 * self.width + x.0]
    }
}

const NEIGHBORS_8: &[AdjacentIterState] = &[
    AdjacentIterState::Above,
    AdjacentIterState::Left,
    AdjacentIterState::Below,
    AdjacentIterState::Right,
    AdjacentIterState::AboveLeft,
    AdjacentIterState::BelowLeft,
    AdjacentIterState::BelowRight,
    AdjacentIterState::AboveRight,
];

#[derive(Debug)]
pub struct AdjacentIter {
    states: std::iter::Copied<std::slice::Iter<'static, AdjacentIterState>>,
    x: X,
    y: Y,
    height: usize,
    width: usize,
}
impl Iterator for AdjacentIter {
    type Item = (X, Y);
    fn next(&mut self) -> Option<Self::Item> {
        match self.states.next() {
            None => None,
            Some(AdjacentIterState::Above) => {
                if self.y.0 > 0 {
                    Some((self.x, Y(self.y.0 - 1)))
                } else {
                    self.next()
                }
            }
            Some(AdjacentIterState::AboveLeft) => {
                if self.y.0 > 0 && self.x.0 > 0 {
                    Some((X(self.x.0 - 1), Y(self.y.0 - 1)))
                } else {
                    self.next()
                }
            }
            Some(AdjacentIterState::Left) => {
                if self.x.0 > 0 {
                    Some((X(self.x.0 - 1), self.y))
                } else {
                    self.next()
                }
            }
            Some(AdjacentIterState::BelowLeft) => {
                if self.x.0 > 0 && self.y.0 + 1 < self.height {
                    Some((X(self.x.0 - 1), Y(self.y.0 + 1)))
                } else {
                    self.next()
                }
            }
            Some(AdjacentIterState::Below) => {
                if self.y.0 + 1 < self.height {
                    Some((self.x, Y(self.y.0 + 1)))
                } else {
                    self.next()
                }
            }
            Some(AdjacentIterState::BelowRight) => {
                if self.y.0 + 1 < self.height && self.x.0 + 1 < self.width {
                    Some((X(self.x.0 + 1), Y(self.y.0 + 1)))
                } else {
                    self.next()
                }
            }
            Some(AdjacentIterState::Right) => {
                if self.x.0 + 1 < self.width {
                    Some((X(self.x.0 + 1), self.y))
                } else {
                    self.next()
                }
            }
            Some(AdjacentIterState::AboveRight) => {
                if self.x.0 + 1 < self.width && self.y.0 > 0 {
                    Some((X(self.x.0 + 1), Y(self.y.0 - 1)))
                } else {
                    self.next()
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum AdjacentIterState {
    Above,
    AboveLeft,
    Left,
    BelowLeft,
    Below,
    BelowRight,
    Right,
    AboveRight,
}
