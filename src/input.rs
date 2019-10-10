use std::io::{stdin, stdout};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

// run until the player do a valid move
pub fn handle_input(grid: &mut crate::grid::Grid) {
    let stdin = stdin();
    let stdout = stdout().into_raw_mode().unwrap();
    for key in stdin.keys() {
        match key.unwrap() {
            Key::Right => {
                grid.cursor.0 += 1;
                if grid.cursor.0 > 2 {
                    grid.cursor.0 = 2
                }
            }
            Key::Left => grid.cursor.0 = grid.cursor.0.saturating_sub(1),
            Key::Down => {
                grid.cursor.1 += 1;
                if grid.cursor.1 > 2 {
                    grid.cursor.1 = 2
                }
            }
            Key::Up => grid.cursor.1 = grid.cursor.1.saturating_sub(1),
            Key::Char(' ') => {
                let cell = &mut grid.grid[grid.cursor.1][grid.cursor.0];
                if *cell == crate::grid::Cell::Empty {
                    *cell = crate::grid::Cell::O;
                    break; // exit the loop
                }
            }
            Key::Ctrl('c') => {
                crate::print::end();
                std::process::exit(1);
            }
            _ => (),
        }
        crate::print::print(&grid);
    }
}
