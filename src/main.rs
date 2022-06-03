const GRID_SIZE: usize = 7; 
pub type Grid = [[bool; GRID_SIZE]; GRID_SIZE];

fn print_grid(grid: &Grid) { // TODO would this be better as a macro?
    for row in grid { 
        for col in row { 
            print!("{}", *col as u8);
        }
        print!("\n");
    }
    print!("\n");
}

fn alive_neighbor_count(grid: &Grid, point: (usize, usize)) -> u8 {
    let neighbors = vec![
        grid[point.0 + 1][point.1],
        grid[point.0][point.1 + 1],
        grid[point.0 - 1][point.1],
        grid[point.0][point.1 -1],
        grid[point.0 + 1][point.1 + 1],
        grid[point.0 - 1][point.1 - 1],
        grid[point.0 + 1][point.1 - 1],
        grid[point.0 - 1][point.1 + 1]
    ];

    neighbors.iter().filter(|&n| *n).count().try_into().unwrap()
}

fn calculate_next_step(grid: &Grid) -> Grid { 
    let mut copy_grid = grid.clone();

    for row in 1..GRID_SIZE - 1 { 
        for col in 1..GRID_SIZE - 1 { 
            let neighbor_count = alive_neighbor_count(&grid, (row, col));
            let current_cell = grid[row][col];

            if (current_cell) { // Live cells
                if (neighbor_count == 2 || neighbor_count == 3) { 
                    copy_grid[row][col] = true;
                } else { 
                    copy_grid[row][col] = false;
                }
            } else { // Dead cells  
                if (neighbor_count == 3) { 
                    copy_grid[row][col] = true;
                } else { 
                    copy_grid[row][col] = false;
                }
            }
        }
    }

    copy_grid
}


fn main() {
    let mut grid = [[false; GRID_SIZE]; GRID_SIZE];
    print_grid(&grid);
    let nextGrid = calculate_next_step(&grid);
    print_grid(&nextGrid);
}