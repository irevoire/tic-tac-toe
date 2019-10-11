use crate::grid::Cell;

pub fn play(grid: &mut crate::grid::Grid) {
    let mut tmp_grid = grid.grid.clone();
    let mut best_move = (10, 10);
    let mut score = None;

    for (y, line) in grid.grid.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if cell == &Cell::Empty {
                tmp_grid[y][x] = Cell::X;
                let tmp_score = minmax_player(&tmp_grid);
                // if score was never set we set it now
                // else check if it’s a better move
                if score.is_none() || tmp_score > score.unwrap() {
                    score = Some(tmp_score);
                    best_move = (x, y);
                }
                tmp_grid[y][x] = Cell::Empty;
            }
        }
    }

    grid.grid[best_move.1][best_move.0] = Cell::X;
}

fn minmax_player(grid: &Vec<Vec<Cell>>) -> i32 {
    match crate::game::finished(grid) {
        Some(Cell::Empty) => return 0,
        Some(Cell::X) => return 1,
        Some(Cell::O) => return -1,
        None => (),
    }

    let mut tmp_grid = grid.clone();
    let mut score: i32 = 0;

    for (y, line) in grid.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if cell == &Cell::Empty {
                tmp_grid[y][x] = Cell::O;
                score += minmax_ia(&tmp_grid);
                tmp_grid[y][x] = Cell::Empty;
            }
        }
    }
    score
}

fn minmax_ia(grid: &Vec<Vec<crate::grid::Cell>>) -> i32 {
    match crate::game::finished(grid) {
        Some(Cell::Empty) => return 0,
        Some(Cell::X) => return 1,
        Some(Cell::O) => return -1,
        None => (),
    }

    let mut tmp_grid = grid.clone();
    let mut score: i32 = 0;

    for (y, line) in grid.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if cell == &Cell::Empty {
                tmp_grid[y][x] = Cell::X;
                score += minmax_player(&tmp_grid);
                tmp_grid[y][x] = Cell::Empty;
            }
        }
    }
    score
}
