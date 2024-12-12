use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt not found");

    let s = input.as_str();

    let total = calc_total_price(s, false);
    println!("Answer part 1: {}", total);

    let total = calc_total_price(s, true);
    println!("Answer part 2: {}", total);
}

fn calc_total_price(input: &str, join_sides: bool) -> usize {
    let mut grid: Vec<Vec<char>> = input.split_whitespace().map(|s|s.trim().chars().collect()).collect();
    let mut visited: Vec<Vec<usize>> = grid.iter().map(|x|x.iter().map(|_|0).collect()).collect();
    let w = grid[0].len();
    let h = grid.len();

    let mut total = 0;

    for y in 0..h {
        for x in 0..w {
            if visited[y][x] & VISITED == 0 {
                let ch = grid[y][x];
                let (a, b) = measure_plot(&mut grid, &mut visited, ch, x, y, w, h, join_sides);
                total += a * b.1;
            }
        }
    }
    total
}

const TOP: usize = 1;
const RIGHT: usize = 2; 
const BOTTOM: usize = 4; 
const LEFT: usize = 8; 

const VISITED: usize = 64;

const LEFT_RIGHT_MASK: usize = LEFT | RIGHT;
const TOP_BOTTOM_MASK: usize = TOP | BOTTOM;

static DIRECTIONS: &'static [(isize, isize, usize, usize)] = 
    &[(0, -1, TOP, LEFT_RIGHT_MASK), (1, 0, RIGHT, TOP_BOTTOM_MASK), (0, 1, BOTTOM, LEFT_RIGHT_MASK), (-1, 0, LEFT, TOP_BOTTOM_MASK)];

fn measure_plot(grid: &mut Vec<Vec<char>>, visited: &mut Vec<Vec<usize>>, ch: char, x: usize, y: usize, w: usize, h: usize, join_sides: bool) -> (usize, (usize, usize)) {
    let mut area: usize = 1;
    let mut my_sides = 0;
    let mut total_sides = 0;

    visited[y][x] = VISITED;

    let mut sides: HashMap<usize, usize> = HashMap::new();

    for d in 0..DIRECTIONS.len() {
        let dir = DIRECTIONS[d];
        let side_mask = dir.2.clone();
        if let Some(pt) = move_pos(x, y, w, h, &dir) {
            let v = visited[pt.1][pt.0];
            if grid[pt.1][pt.0] == ch {
                if v & VISITED == 0 {
                    let r = measure_plot(grid, visited, ch, pt.0, pt.1, w, h, join_sides);
                    sides.insert(d, r.1.0);
                    area += r.0;
                    total_sides += r.1.1;
                } else {
                    sides.insert(d, v - VISITED);
                }
            } else {
                if join_sides {
                    my_sides |= side_mask;
                }
                total_sides += 1;
            }
        } else {
            if join_sides {
                my_sides |= side_mask;
            }
            total_sides += 1;
        }
    }

    if join_sides {
        let mut borrow = 0;

        // Now account for duplicate sides im adjacent neighbours
        for d in 0..DIRECTIONS.len() {
            let dir = DIRECTIONS[d];
            let dir_mask = dir.3;
            if let Some(neighbour_sides) = sides.get(&d) {
                let match_sides = (my_sides & neighbour_sides) & dir_mask;
                borrow += match_sides.count_ones() as usize;
            }
        }

        total_sides -= borrow;
    }

    visited[y][x] = my_sides + VISITED;

    return (area, (my_sides, total_sides));
}

fn move_pos(x: usize, y: usize, w: usize, h: usize, dir: &(isize, isize, usize, usize)) -> Option<(usize, usize)> {
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
            "RRRRIICCFF
             RRRRIICCCF
             VVRRRCCFFF
             VVRCCCJFFF
             VVVVCJJCFE
             VVIVCCJJEE
             VVIIICJJEE
             MIIIIIJJEE
             MIIISIJEEE
             MMMISSJEEE
             ";

        let total = calc_total_price(input.into(), false);
        assert_eq!(1930, total);
    }

    #[test]
    fn test_part2() {
        let input = 
            "RRRRIICCFF
             RRRRIICCCF
             VVRRRCCFFF
             VVRCCCJFFF
             VVVVCJJCFE
             VVIVCCJJEE
             VVIIICJJEE
             MIIIIIJJEE
             MIIISIJEEE
             MMMISSJEEE
             ";
            // "AA
            //  AA";

        let total = calc_total_price(input.into(), true);
        assert_eq!(19120630, total);
    }
}