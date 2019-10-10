use crate::grid::Cell;

/// Return the winner of the game, return Empty if
/// there is currently no winner.
/// What if two persons win at the same time?
/// This should not happens.
pub fn finished(grid: &crate::grid::Grid) -> Cell {
    let grid = &grid.grid;
    // check if there is a line full
    if grid.iter().any(|l| l.iter().all(|c| c == &Cell::O)) {
        return Cell::O;
    }
    if grid.iter().any(|l| l.iter().all(|c| c == &Cell::X)) {
        return Cell::X;
    }

    // check if there is a column full
    if grid[0][0] == grid[1][0] && grid[0][0] == grid[2][0] {
        return grid[0][0];
    }
    if grid[0][1] == grid[1][1] && grid[0][1] == grid[2][1] {
        return grid[0][1];
    }
    if grid[0][2] == grid[1][2] && grid[0][2] == grid[2][2] {
        return grid[0][2];
    }

    // check if there is a diagonal full
    if grid[0][0] == grid[1][1] && grid[0][0] == grid[2][2] {
        return grid[0][0];
    }
    if grid[0][2] == grid[1][1] && grid[0][2] == grid[2][0] {
        return grid[0][2];
    }
    Cell::Empty
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::Cell::*;

    #[test]
    fn test_finish_line() {
        let g = crate::grid::Grid {
            grid: vec![
                vec![X, X, X],
                vec![Empty, Empty, Empty],
                vec![Empty, Empty, Empty],
            ],
            cursor: (0, 0),
        };
        assert_eq!(finished(&g), X);
        let g = crate::grid::Grid {
            grid: vec![vec![X, O, X], vec![O, O, O], vec![X, X, O]],
            cursor: (0, 0),
        };
        assert_eq!(finished(&g), O);
        let g = crate::grid::Grid {
            grid: vec![
                vec![Empty, Empty, Empty],
                vec![Empty, Empty, Empty],
                vec![Empty, Empty, Empty],
            ],
            cursor: (0, 0),
        };
        assert_eq!(finished(&g), Empty);
    }

    #[test]
    fn test_finish_column() {
        let g = crate::grid::Grid {
            grid: vec![
                vec![X, Empty, Empty],
                vec![X, Empty, Empty],
                vec![X, Empty, Empty],
            ],
            cursor: (0, 0),
        };
        assert_eq!(finished(&g), X);
        let g = crate::grid::Grid {
            grid: vec![vec![X, O, Empty], vec![X, O, X], vec![Empty, O, X]],
            cursor: (0, 0),
        };
        assert_eq!(finished(&g), O);
        let g = crate::grid::Grid {
            grid: vec![
                vec![Empty, Empty, Empty],
                vec![Empty, Empty, Empty],
                vec![Empty, Empty, Empty],
            ],
            cursor: (0, 0),
        };
        assert_eq!(finished(&g), Empty);
    }

    #[test]
    fn test_finish_diagonals() {
        let g = crate::grid::Grid {
            grid: vec![
                vec![X, Empty, Empty],
                vec![Empty, X, Empty],
                vec![Empty, Empty, X],
            ],
            cursor: (0, 0),
        };
        assert_eq!(finished(&g), X);
        let g = crate::grid::Grid {
            grid: vec![vec![X, Empty, X], vec![Empty, X, Empty], vec![X, Empty, X]],
            cursor: (0, 0),
        };
        assert_eq!(finished(&g), X);
        let g = crate::grid::Grid {
            grid: vec![vec![O, X, X], vec![Empty, O, Empty], vec![X, X, O]],
            cursor: (0, 0),
        };
        assert_eq!(finished(&g), O);
        let g = crate::grid::Grid {
            grid: vec![
                vec![Empty, Empty, Empty],
                vec![Empty, X, Empty],
                vec![Empty, Empty, X],
            ],
            cursor: (0, 0),
        };
        assert_eq!(finished(&g), Empty);
    }
}
