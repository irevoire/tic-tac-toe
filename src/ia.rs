pub fn play(grid: &mut crate::grid::Grid) {
    for line in grid.grid.iter_mut() {
        for cell in line.iter_mut() {
            if cell == &crate::grid::Cell::Empty {
                *cell = crate::grid::Cell::X;
                return;
            }
        }
    }
}
