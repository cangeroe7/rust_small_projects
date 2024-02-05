use std::collections::HashSet;

pub fn check_input(input: &str) -> bool {
    if input.len() != 81 { return false; }
    if input.chars().any(|kar| !kar.is_digit(10)) { return false; }
    true
}

// convert str of numbers to grid of usize
pub fn str_to_grid(str_grid: &str) -> Vec<Vec<usize>> {
    let mut grid: Vec<Vec<usize>> = vec![vec![0; 9]; 9];
    str_grid.char_indices().for_each(|(idx, kar)| {
        let row = idx / 9;
        let col = idx % 9;
        grid[row][col] = kar.to_string().parse::<usize>().unwrap();
    });
    grid
}

// check if the initial size is big enough and if it already has overlapping values
pub fn first_check(grid: &Vec<Vec<usize>>) -> bool {
    let mut rows: Vec<_> = (0 .. 9).map(|_| HashSet::<usize>::new()).collect();
    let mut cols: Vec<_> = (0 .. 9).map(|_| HashSet::<usize>::new()).collect();
    let mut boxes: Vec<_> = (0 .. 9).map(|_| HashSet::<usize>::new()).collect();
    let mut squares = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let num = grid[row][col];
            if num == 0 { continue; }
            let boxs: usize = (row / 3) * 3 + col / 3;
            
            if rows[row].contains(&num) || cols[col].contains(&num) || boxes[boxs].contains(&num) {
                return false;
            }
            rows[row].insert(num);
            cols[col].insert(num);
            boxes[boxs].insert(num);
            squares += 1;
        }
    }
    squares >= 17
}

pub fn solve(grid: &mut Vec<Vec<usize>>, count: &mut usize, last_grid: &mut Vec<Vec<usize>>) {
    for x in 0..9 {
        for y in 0..9 {
            if grid[x][y] == 0 {
                for n in 1..10 {
                    if possible(x, y, n, grid) {
                        grid[x][y] = n;
                        solve(grid, count, last_grid);
                        grid[x][y] = 0;
                    }
                } 
                return;
            }
        }
    }
    *last_grid = grid.clone();
    *count += 1;
}

fn possible(x: usize, y: usize, n: usize, grid: &Vec<Vec<usize>>) -> bool {
    for i in 0..9 {
        if grid[i][y] == n {
            return false;
        }
    }
    for i in 0..9 {
        if grid[x][i] == n {
            return false;
        }
    }

    let x0 = (x / 3) * 3;
    let y0 = (y / 3) * 3;

    for i in 0..3 {
        for j in 0..3 {
            if grid[x0 + i][y0 + j] == n {
                return false;
            }
        }
    }
    true
} 




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_sudoku() {
        let grid: Vec<Vec<usize>> = [
            [4, 0, 0, 0, 0, 5, 0, 0, 0].to_vec(),
            [0, 0, 0, 0, 0, 0, 1, 9, 8].to_vec(),
            [3, 0, 0, 0, 8, 2, 4, 0, 0].to_vec(),
            [0, 0, 0, 1, 0, 0, 0, 8, 0].to_vec(),
            [9, 0, 3, 0, 0, 0, 0, 0, 0].to_vec(),
            [0, 0, 0, 0, 3, 0, 6, 7, 0].to_vec(),
            [0, 5, 0, 0, 0, 9, 0, 0, 0].to_vec(),
            [0, 0, 0, 2, 0, 0, 9, 0, 7].to_vec(),
            [6, 4, 0, 3, 0, 0, 0, 0, 0].to_vec(),
            ].to_vec();
        assert_eq!(first_check(&grid), true);
    }
    #[test]
    fn wrong_row() {
        let grid: Vec<Vec<usize>> = [
            [4, 0, 0, 0, 0, 5, 4, 0, 0].to_vec(),
            [0, 0, 0, 0, 0, 0, 1, 9, 8].to_vec(),
            [3, 0, 0, 0, 8, 2, 4, 0, 0].to_vec(),
            [0, 0, 0, 1, 0, 0, 0, 8, 0].to_vec(),
            [9, 0, 3, 0, 0, 0, 0, 0, 0].to_vec(),
            [0, 0, 0, 0, 3, 0, 6, 7, 0].to_vec(),
            [0, 5, 0, 0, 0, 9, 0, 0, 0].to_vec(),
            [0, 0, 0, 2, 0, 0, 9, 0, 7].to_vec(),
            [6, 4, 0, 3, 0, 0, 0, 0, 0].to_vec(),
            ].to_vec();
        assert_eq!(first_check(&grid), false);
    }
    #[test]
    fn wrong_col() {
        let grid: Vec<Vec<usize>> = [
            [4, 0, 0, 0, 0, 5, 0, 0, 0].to_vec(),
            [0, 0, 0, 0, 0, 0, 1, 9, 8].to_vec(),
            [3, 0, 0, 0, 8, 2, 4, 0, 0].to_vec(),
            [0, 0, 0, 1, 0, 0, 0, 8, 0].to_vec(),
            [9, 0, 3, 0, 0, 0, 0, 0, 0].to_vec(),
            [0, 0, 0, 0, 3, 0, 6, 7, 0].to_vec(),
            [0, 5, 0, 1, 0, 9, 0, 0, 0].to_vec(),
            [0, 0, 0, 2, 0, 0, 9, 0, 7].to_vec(),
            [6, 4, 0, 3, 0, 0, 0, 0, 0].to_vec(),
            ].to_vec();
        assert_eq!(first_check(&grid), false);
    }
    #[test]
    fn wrong_box() {
        let grid: Vec<Vec<usize>> = [
            [4, 0, 0, 0, 0, 5, 0, 0, 0].to_vec(),
            [0, 0, 0, 0, 0, 0, 1, 9, 8].to_vec(),
            [3, 0, 0, 0, 8, 2, 4, 0, 0].to_vec(),
            [0, 0, 0, 1, 0, 0, 0, 8, 0].to_vec(),
            [9, 0, 3, 0, 0, 0, 0, 0, 0].to_vec(),
            [0, 0, 0, 0, 3, 0, 6, 7, 0].to_vec(),
            [0, 5, 6, 0, 0, 9, 0, 0, 0].to_vec(),
            [0, 0, 0, 2, 0, 0, 9, 0, 7].to_vec(),
            [6, 4, 0, 3, 0, 0, 0, 0, 0].to_vec(),
            ].to_vec();
        assert_eq!(first_check(&grid), false);
    }
    // "200000009003040200070903060007050100050267030002080600060402080008030400700000003"
    #[test] 
    fn test_str_to_grid() {
        let grid_str: &str = "400005000000000198300082400000100080903000000000030670056009000000200907640300000";
        let grid: Vec<Vec<usize>> = [
        [4, 0, 0, 0, 0, 5, 0, 0, 0].to_vec(),
        [0, 0, 0, 0, 0, 0, 1, 9, 8].to_vec(),
        [3, 0, 0, 0, 8, 2, 4, 0, 0].to_vec(),
        [0, 0, 0, 1, 0, 0, 0, 8, 0].to_vec(),
        [9, 0, 3, 0, 0, 0, 0, 0, 0].to_vec(),
        [0, 0, 0, 0, 3, 0, 6, 7, 0].to_vec(),
        [0, 5, 6, 0, 0, 9, 0, 0, 0].to_vec(),
        [0, 0, 0, 2, 0, 0, 9, 0, 7].to_vec(),
        [6, 4, 0, 3, 0, 0, 0, 0, 0].to_vec(),
        ].to_vec();
        assert_eq!(str_to_grid(grid_str), grid);
    }
}
