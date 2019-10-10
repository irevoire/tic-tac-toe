#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cell {
    O,
    X,
    Empty,
}

pub struct Grid {
    pub grid: Vec<Vec<Cell>>,
    pub cursor: (usize, usize),
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            grid: vec![vec![Cell::Empty; 3]; 3],
            cursor: (1, 1),
        }
    }
}
