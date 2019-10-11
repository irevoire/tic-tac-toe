use crate::grid::Cell;

/// Return the winner of the game, return Empty if
/// there is currently no winner.
/// What if two persons win at the same time?
/// This should not happens.
pub fn finished(grid: &Vec<Vec<Cell>>) -> Option<Cell> {
    // check if there is a line full
    if grid.iter().any(|l| l.iter().all(|c| c == &Cell::O)) {
        return Some(Cell::O);
    }
    if grid.iter().any(|l| l.iter().all(|c| c == &Cell::X)) {
        return Some(Cell::X);
    }

    // check if there is a column full
    if grid[0][0] == grid[1][0] && grid[0][0] == grid[2][0] && grid[0][0] != Cell::Empty {
        return Some(grid[0][0]);
    }
    if grid[0][1] == grid[1][1] && grid[0][1] == grid[2][1] && grid[0][1] != Cell::Empty {
        return Some(grid[0][1]);
    }
    if grid[0][2] == grid[1][2] && grid[0][2] == grid[2][2] && grid[0][2] != Cell::Empty {
        return Some(grid[0][2]);
    }

    // check if there is a diagonal full
    if grid[0][0] == grid[1][1] && grid[0][0] == grid[2][2] && grid[0][0] != Cell::Empty {
        return Some(grid[0][0]);
    }
    if grid[0][2] == grid[1][1] && grid[0][2] == grid[2][0] && grid[0][2] != Cell::Empty {
        return Some(grid[0][2]);
    }

    // now no one has win, is there a ex aequo?
    // is the grid full?
    if grid.iter().all(|l| l.iter().all(|c| c != &Cell::Empty)) {
        return Some(Cell::Empty);
    }

    // still possible to play
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::Cell::*;

    #[test]
    fn test_finish_line() {
        let g = vec![
            vec![X, X, X],
            vec![Empty, Empty, Empty],
            vec![Empty, Empty, Empty],
        ];
        assert_eq!(finished(&g), Some(X));
        let g = vec![vec![X, O, X], vec![O, O, O], vec![X, X, O]];
        assert_eq!(finished(&g), Some(O));
    }

    #[test]
    fn test_finish_column() {
        let g = vec![
            vec![X, Empty, Empty],
            vec![X, Empty, Empty],
            vec![X, Empty, Empty],
        ];
        assert_eq!(finished(&g), Some(X));
        let g = vec![vec![X, O, Empty], vec![X, O, X], vec![Empty, O, X]];
        assert_eq!(finished(&g), Some(O));
    }

    #[test]
    fn test_finish_diagonals() {
        let g = vec![
            vec![X, Empty, Empty],
            vec![Empty, X, Empty],
            vec![Empty, Empty, X],
        ];
        assert_eq!(finished(&g), Some(X));
        let g = vec![vec![X, Empty, X], vec![Empty, X, Empty], vec![X, Empty, X]];
        assert_eq!(finished(&g), Some(X));
        let g = vec![vec![O, X, X], vec![Empty, O, Empty], vec![X, X, O]];
        assert_eq!(finished(&g), Some(O));
        let g = vec![vec![Empty, Empty, O], vec![Empty, O, X], vec![O, X, X]];
        assert_eq!(finished(&g), Some(O));
    }

    #[test]
    fn test_finished_ex_aequo() {
        let g = vec![vec![X, O, X], vec![X, O, O], vec![O, X, X]];
        assert_eq!(finished(&g), Some(Empty));
    }

    #[test]
    fn test_finished_continue() {
        let g = vec![
            vec![Empty, Empty, Empty],
            vec![Empty, X, Empty],
            vec![Empty, Empty, X],
        ];
        assert_eq!(finished(&g), None);
        let g = vec![
            vec![O, Empty, Empty],
            vec![Empty, Empty, Empty],
            vec![Empty, Empty, X],
        ];
        assert_eq!(finished(&g), None);
        let g = vec![
            vec![Empty, Empty, Empty],
            vec![Empty, Empty, Empty],
            vec![Empty, Empty, Empty],
        ];
        assert_eq!(finished(&g), None);
    }
}
