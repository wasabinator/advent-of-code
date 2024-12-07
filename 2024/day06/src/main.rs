use std::{borrow::{Borrow, BorrowMut}, collections::{HashMap, HashSet}, fs::{self}, process::exit};

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt not found");

    // Calculate part 1 answer
    let (answer1, answer2) = process(input.as_str());
    println!("Answer part 1: {}", answer1);
    println!("Answer part 2: {}", answer2);
}

fn process(input: &str) -> (usize, usize) {
    calc_guard_route(input)
}

static DIRECTIONS: &'static [(isize, isize)] = &[(0, -1), (1, 0), (0, 1), (-1, 0)];

fn calc_guard_route(input: &str) -> (usize, usize) {
    let rows: Vec<&str> = input.lines().map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
    let mut grid: Vec<Vec<char>> = rows.into_iter().map(|s|s.chars().collect()).collect();
    let width = grid[0].len();
    let height = grid.len();

    let mut pos_count = 0;
    let mut obstruction_count = 0;

    for x in 0..width {
        for y in 0..height {
            let ch = grid[y][x];
            if ch == '^' {
                grid[y][x] = 'X';
                (pos_count, obstruction_count) = calc_route(&mut grid, x, y, 0)
            }
        }
    }

    (pos_count, obstruction_count)
}

fn calc_route(grid: &mut Vec<Vec<char>>, start_x: usize, start_y: usize, start_dir: usize) -> (usize, usize) {
    let mut pos_count = 1;
    let mut obstruction_count = 0;
    let width = grid[0].len();
    let height = grid.len();
    let mut direction = start_dir;

    // List of positions where we needed to turn, plus the direction that got us into the obstacle
    let mut turn_positions: &mut Vec<(usize, usize, usize)> = &mut Vec::new();

    let mut x = start_x;
    let mut y = start_y;

    loop {
        let dir = DIRECTIONS[direction];
        let next_x = x.checked_add_signed(dir.0);
        let next_y = y.checked_add_signed(dir.1);

        let valid = next_x.is_some() && next_y.is_some();
        let next_x = next_x.unwrap_or(0);
        let next_y = next_y.unwrap_or(0);

        if valid && next_x < width && next_y < height {
            let ch = grid[next_y][next_x];
            if ch == '#' {
                turn_positions.push((x,y,direction));
                direction = (direction + 1) & 3;
            } else {
                x = next_x;
                y = next_y;
                if ch != 'X' {
                    pos_count += 1;
                }
                grid[y][x] = 'X';
            }
        } else {
            // Add the last location on map for purposes of re-walking later
            turn_positions.push((x,y,direction));
            break;
        }
    }

    // Now retrace our steps and fine the obstacles which would lead to an endless walk
    let mut x = start_x;
    let mut y = start_y;
    let mut dir = start_dir;

    println!("*** RETRACING ***** start_dir: {}", dir);
    let len = turn_positions.len();
    let mut in_front: bool = true;
    for i in 0..len {
        let pos = turn_positions[i];
        let right_dir = (dir + 1) & 3;
        while x != pos.0 || y != pos.1 {
            grid[y][x] = '!';
            if let Some(p) = move_pos(x, y, width, height, dir) {
                x = p.0;
                y = p.1;
            } else {
                println!("Finished");
                break;
                //exit(0);
            }
    
            for j in 0..i {
                let turn = turn_positions[j];

                if (x == turn.0 || y == turn.1) && turn.2 == right_dir {
                    if can_reach(grid, x, y, &turn, right_dir) {
                        println!("Found obstacle position from: ({},{}), dir: ({},{})", x, y, DIRECTIONS[dir].0, DIRECTIONS[dir].1);
                        grid[y][x] = 'O';
                        obstruction_count += 1;
                    }
                }
            }
        }
        dir = right_dir;
    }

    for i in 0..width {
        for j in 0..height {
            print!("{}", grid[i][j]);
        }
        println!();
    }

    println!("Turn positions");
    for pos in turn_positions {
        print!("({},{},{})", pos.0, pos.1, pos.2);
    }
    println!();

    (pos_count, obstruction_count)
}

fn can_reach(grid: &mut Vec<Vec<char>>, curr_x: usize, curr_y: usize, point: &(usize, usize, usize), direction: usize) -> bool {
    let mut x = curr_x;
    let mut y = curr_y;
    let width = grid[0].len();
    let height = grid.len();

    //let right = (direction + 1) & 3;
    let dir = DIRECTIONS[direction];
    if (x == point.0 && ((y > point.1 && dir.1 < 0) || (y < point.1 && dir.1 > 0))) ||
       (y == point.1 && ((x > point.0 && dir.0 < 0) || (x < point.0 && dir.0 > 0))) {
        // Scan for now obstacles
        while x != point.0 || y != point.1 {
            if grid[y][x] == '#' {
                return false;
            }
            if let Some(pt) = move_pos(x, y, width, height, direction) {
                x = pt.0;
                y = pt.1;
            } else {
                return false;
            }
        }
        true
    } else {
        false
    }
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

        let (pos_count, obstacle_count) = calc_guard_route(input.into());
        assert_eq!(41, pos_count);
        assert_eq!(6, obstacle_count);
    }
}