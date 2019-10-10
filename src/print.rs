use crate::grid::Cell::*;
use termion::color;

pub fn print(grid: &crate::grid::Grid) {
    print!("{}", termion::cursor::Hide);
    println!("+—+—+—+\r");
    for (y, line) in grid.grid.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            let cell = match cell {
                O => format!("{}O", color::Fg(color::Green)),
                X => format!("{}X", color::Fg(color::Red)),
                Empty => String::from(" "),
            };
            print!("{}|", color::Fg(color::Reset));
            if (x, y) == grid.cursor {
                print!("{}", color::Bg(color::White));
            }
            print!("{}{}", cell, color::Bg(color::Reset));
        }
        println!("|\r");
        println!("+—+—+—+\r");
    }
    print!("{}", termion::cursor::Up(7));
}

pub fn end() {
    print!("{}", termion::cursor::Down(7));
    print!("{}", termion::cursor::Show);
}
