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
        if let Some(next) = move_pos(x, y, width, height, direction) {
            let ch = grid[next.1][next.0];
            if ch == '#' || ch == 'O' {
                direction = (direction + 1) & 3;
            } else {
                x = next.0;
                y = next.1;
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
    let width = grid[0].len();
    let height = grid.len();
    let mut direction = start_dir;

    let mut x = start_x;
    let mut y = start_y;

    let mut obstacles: HashSet<(usize, usize)> = HashSet::new();

    loop {
        if let Some(p) = move_pos(x, y, width, height, direction) {
            let ch = grid[p.1][p.0];
            if ch == '#' {
                direction = (direction + 1) & 3;
            } else {
                // Test with an obstruction here
                if ch != '^' {
                    grid[p.1][p.0] = 'O';
                    let rc = calc_positions(grid, start_x, start_y, start_dir);
                    if let None = rc {
                        obstacles.insert((p.0, p.1));
                    }
                    grid[p.1][p.0] = '.';
                }
                x = p.0;
                y = p.1;
            }
        } else {
            break;
        }
    }
    obstacles.len()
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