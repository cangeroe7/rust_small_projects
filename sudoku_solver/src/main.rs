use std::env::args;

use functions::{check_input, str_to_grid};

use crate::functions::{first_check, solve};
mod functions;

fn main() {

    let input = args().nth(1).expect("no input sudoku given");
    if !check_input(&input) { panic!("the input should be 81 digits straight") }

    let mut grid = str_to_grid(&input); 

    println!("\nChecking solution for the following grid:\n");
    grid.iter().for_each(|row| println!("{:?}", row));

    println!("\nChecking for duplicates or lack of revealed squares errors...\n");
    if !first_check(&grid) { 
        println!("There are duplicates in rows, columns, or boxes, or there are less than 17 known squares\n");
        return;
    } else {
        println!("The grid has no duplicates and has a solvable amount of revealed squares\n")
    }

    let mut count = 0;
    let mut last_grid = vec![vec![]];
    solve(&mut grid, &mut count, &mut last_grid);
    println!("Checking if the sudoku has a unique solution... \n");
    match count {
        0 => { println!("The sudoku grid has no solution")},
        1 => {
            println!("The sudoku grid has a unique solution:\n");
            last_grid.iter().for_each(|row| println!("{:?}", row))
        },
        _ => { println!("The sudoku has multiple solutions and is not valid")}

    };
}
