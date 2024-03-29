mod ai;
mod game;
mod grid;
mod input;
mod print;

fn main() {
    let mut grid = grid::Grid::new();

    while game::finished(&grid.grid).is_none() {
        print::print(&grid);
        input::handle_input(&mut grid);
        if game::finished(&grid.grid).is_some() {
            break;
        }
        ai::play(&mut grid);
    }

    grid.cursor = (3, 3); // hack to hide the cursor
    print::print(&grid);
    print::end();
    match game::finished(&grid.grid) {
        Some(grid::Cell::O) => println!("Victory"),
        Some(grid::Cell::X) => println!(":("),
        Some(grid::Cell::Empty) => println!("Ex aequo"),
        _ => println!("wtf"),
    }
}
