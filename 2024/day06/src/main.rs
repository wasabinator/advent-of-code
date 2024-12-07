use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt not found");

    let s = input.as_str();

    // Calculate part 1 answer
    let moves = calc_guard_route(s);
    println!("Answer part 1: {}", moves.unwrap());
    let obstacles = calc_guard_obstructions(s);
    println!("Answer part 2: {}", obstacles);
}

static DIRECTIONS: &'static [(isize, isize)] = &[(0, -1), (1, 0), (0, 1), (-1, 0)];

fn calc_guard_route(input: &str) -> Option<usize> {
    let rows: Vec<&str> = input.lines().map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
    let mut grid: Vec<Vec<char>> = rows.into_iter().map(|s|s.chars().collect()).collect();
    let width = grid[0].len();
    let height = grid.len();

    for x in 0..width {
        for y in 0..height {
            let ch = grid[y][x];
            if ch == '^' {
                return calc_positions(&mut grid, x, y, 0);
            }
        }
    }

    None
}

fn calc_guard_obstructions(input: &str) -> usize {
    let rows: Vec<&str> = input.lines().map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
    let mut grid: Vec<Vec<char>> = rows.into_iter().map(|s|s.chars().collect()).collect();
    let width = grid[0].len();
    let height = grid.len();

    for x in 0..width {
        for y in 0..height {
            let ch = grid[y][x];
            if ch == '^' {
                return calc_obstructions(&mut grid, x, y, 0);
            }
        }
    }

    0
}

fn calc_positions(grid: &mut Vec<Vec<char>>, start_x: usize, start_y: usize, start_dir: usize) -> Option<usize> {
    let width = grid[0].len();
    let height = grid.len();
    let mut direction = start_dir;

    let mut x = start_x;
    let mut y = start_y;

    let mut prev_pos: HashSet<(usize, usize)> = HashSet::new();
    let mut prev_pos_direction: HashSet<(usize, usize, usize)> = HashSet::new();

    prev_pos.insert((start_x, start_y));
    prev_pos_direction.insert((start_x, start_y, start_dir));

    loop {
        let dir = DIRECTIONS[direction];
        let next_x = x.checked_add_signed(dir.0);
        let next_y = y.checked_add_signed(dir.1);

        let valid = next_x.is_some() && next_y.is_some();
        let next_x = next_x.unwrap_or(0);
        let next_y = next_y.unwrap_or(0);

        if valid && next_x < width && next_y < height {
            let ch = grid[next_y][next_x];
            if ch == '#' || ch == 'O' {
                direction = (direction + 1) & 3;
            } else {
                x = next_x;
                y = next_y;
                prev_pos.insert((x, y));
                if !prev_pos_direction.insert((x, y, direction)) {
                    return None;
                }
            }
        } else {
            break;
        }
    }

    Some(prev_pos.len())
}

fn calc_obstructions(grid: &mut Vec<Vec<char>>, start_x: usize, start_y: usize, start_dir: usize) -> usize {
    let mut count = 0;
    let width = grid[0].len();
    let height = grid.len();
    let mut direction = start_dir;

    let mut x = start_x;
    let mut y = start_y;

    loop {
        if let Some(p) = move_pos(x, y, width, height, direction) {
            let ch = grid[p.1][p.0];
            if ch == '#' {
                direction = (direction + 1) & 3;
            } else {
                // Test with an obstruction here
                if p.0 != start_x && p.1 != start_y {
                    grid[p.1][p.0] = 'O';
                    let rc = calc_positions(grid, start_x, start_y, start_dir);
                    if let None = rc {
                        // println!("Found obstacle");
                        // for i in 0..width {
                        //     for j in 0..height {
                        //         print!("{}", grid[i][j]);
                        //     }
                        //     println!();
                        // }
                        count += 1;
                    }
                    grid[p.1][p.0] = '.';
                }
                x = p.0;
                y = p.1;
            }
        } else {
            println!("Finished");
            break;
        }
    }
    count
}

fn move_pos(x: usize, y: usize, w: usize, h: usize, dir: usize) -> Option<(usize, usize)> {
    let dir = DIRECTIONS[dir];
    let x2 = x.checked_add_signed(dir.0);
    let y2 = y.checked_add_signed(dir.1);
    if x2.is_none() || y2.is_none() {
        return None;
    }
    let x2 = x2.unwrap();
    let y2 = y2.unwrap();
    if x2 < w && y2 < h {
        return Some((x2, y2));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = 
            "....#.....
             .........#
             ..........
             ..#.......
             .......#..
             ..........
             .#..^.....
             ........#.
             #.........
             ......#...";

        let pos_count= calc_guard_route(input.into());
        assert_eq!(Some(41), pos_count);
    }

    #[test]
    fn test_part2() {
        let input =
            "....#.....
             .........#
             ..........
             ..#.......
             .......#..
             ..........
             .#..^.....
             ........#.
             #.........
             ......#...";

        let obstacle_count = calc_guard_obstructions(input.into());
        assert_eq!(6, obstacle_count);
    }
}