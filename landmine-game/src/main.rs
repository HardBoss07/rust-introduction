use rand::Rng;
use std::io;

struct Player {
    pos: (usize, usize), // (row, col)
    face: char,
}

#[derive(Clone)]
struct Field {
    is_mine: bool,
}

fn main() {
    loop {
        let mut rng = rand::rng();

        let number_of_mines = 5;
        let grid_width = 9;
        let grid_height = 15;

        // make empty grid
        let mut grid = vec![vec![Field { is_mine: false }; grid_width]; grid_height];

        // put mines
        let mut placed = 0;
        while placed < number_of_mines {
            let r = rng.random_range(0..grid_height);
            let c = rng.random_range(0..grid_width);
            if !grid[r][c].is_mine && r != 0 {
                grid[r][c].is_mine = true;
                placed += 1;
            }
        }

        let mut player = Player {
            pos: (0, grid_width / 2),
            face: 'v',
        };

        println!("Welcome to Mines! Reach the bottom without hitting a mine.");
        print_grid(&grid, &player);

        // game loop
        loop {
            // check win
            if player.pos.0 == grid_height - 1 {
                println!("You win!");
                break;
            }

            println!("Move (l = left, r = right, d = down): ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            let (mut row, mut col) = player.pos;

            if input == "l" && col > 0 {
                col -= 1;
                player.face = '<';
            } else if input == "r" && col < grid_width - 1 {
                col += 1;
                player.face = '>';
            } else if input == "d" && row < grid_height - 1 {
                row += 1;
                player.face = 'v';
            } else {
                println!("Invalid input.");
                continue;
            }

            player.pos = (row, col);

            if grid[row][col].is_mine {
                println!("BOOM! You hit a mine!");
                // show mines
                for r in 0..grid_height {
                    for c in 0..grid_width {
                        if player.pos == (r, c) {
                            print!("{}", player.face);
                        } else if grid[r][c].is_mine {
                            print!("*");
                        } else {
                            print!(".");
                        }
                    }
                    println!();
                }
                break;
            }

            print_grid(&grid, &player);
        }

        // play again?
        println!("Play again? (y/n): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() != "y" {
            println!("Goodbye!");
            break;
        }
    }
}

fn print_grid(grid: &Vec<Vec<Field>>, player: &Player) {
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if player.pos == (r, c) {
                print!("{}", player.face);
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}
