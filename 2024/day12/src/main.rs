use std::{collections::{HashMap, HashSet, VecDeque}, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt not found");

    let s = input.as_str();

    let total = calc_total_price(s);
    println!("Answer part 1: {}", total);
}

fn calc_total_price(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input.split_whitespace().map(|s|s.chars().collect()).collect();
    let mut visited: Vec<Vec<char>> = grid.clone();

    let w = grid[0].len();
    let h = grid.len();
    let mut total = 0;

    for y in 0..h {
        for x in 0..w {
            if visited[y][x] != '.' {
                let ch = grid[y][x];
                let (a, b) = measure_plot(&mut grid, &mut visited, ch, x, y, w, h);
                total += (a * b);
                println!("Plot {} = ({}, {})", ch, a, b);
            }
        }
    }
    total
}

static DIRECTIONS: &'static [(isize, isize)] = &[(0, -1), (1, 0), (0, 1), (-1, 0)];

fn measure_plot(grid: &mut Vec<Vec<char>>, visited: &mut Vec<Vec<char>>, ch: char, x: usize, y: usize, w: usize, h: usize) -> (usize, usize) {
    let mut range: (usize, usize) = (1, 0);
    let sides = 0;
    visited[y][x] = '.';
    for dir in DIRECTIONS {
        if let Some(pt) = move_pos(x, y, w, h, dir) {
            let v = visited[pt.1][pt.0];
            if grid[pt.1][pt.0] == ch {
                if v != '.' {
                    let r = measure_plot(grid, visited, ch, pt.0, pt.1, w, h);
                    range = (range.0 + r.0, range.1 + r.1);
                }
            } else {
                range = (range.0, range.1 + 1);
            }
        } else {
            range = (range.0, range.1 + 1);
        }
    }
    range
}

fn move_pos(x: usize, y: usize, w: usize, h: usize, dir: &(isize, isize)) -> Option<(usize, usize)> {
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

        let total = calc_total_price(input.into());
        assert_eq!(1930, total);
    }
}