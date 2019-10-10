mod game;
mod grid;
mod ia;
mod input;
mod print;

fn main() {
    let mut grid = grid::Grid::new();

    while game::finished(&grid) == grid::Cell::Empty {
        print::print(&grid);
        input::handle_input(&mut grid);
        ia::play(&mut grid);
    }

    grid.cursor = (3, 3); // hack to hide the cursor
    print::print(&grid);
    print::end();
    match game::finished(&grid) {
        grid::Cell::O => println!("Victory"),
        grid::Cell::X => println!(":("),
        _ => println!("wtf"),
    }
}