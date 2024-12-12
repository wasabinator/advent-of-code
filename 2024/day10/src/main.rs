use std::{collections::{HashSet, VecDeque}, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt not found");

    let s = input.as_str();

    // Calculate part 1 answer
    let checksum = calc_trailhead_scores(s);
    println!("Answer part 1: {}", checksum);

}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    level: usize,
    x: usize,
    y: usize,
}

fn calc_trailhead_scores(input: &str) -> usize {
    let rows: Vec<&str> = input.lines().map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
    let grid: Vec<Vec<usize>> = rows.into_iter().map(|s|s.chars().map(|n| n.to_digit(10).unwrap() as usize).collect()).collect();
    let width = grid[0].len();
    let height = grid.len();
    let mut total_score = 0;

    for y in 0..height {
        for x in 0..width {
            print!("{}", grid[y][x]);
        }
        println!();
    }

    let dirs: Vec<(isize, isize)> = vec![
        (0, -1), (-1, 0), (1, 0), (0, 1)
    ];

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 0 {
                // Found a trailhead, so calc it's score
                //let trail_ends: HashSet<(usize, usize)> = HashSet::new();
                let mut trail_points: VecDeque<Point> = VecDeque::new();
                let mut v: HashSet<Point> = HashSet::new(); 
                let mut score = 0;

                trail_points.push_back(Point {level: 0, x: x, y: y});
                while let Some(pt) = trail_points.pop_front() {
                    for d in &dirs {
                        //println!("dir: {},{}", d.0, d.1);
                        if let Some(pt2) = move_pos(pt.x, pt.y, width, height, d) {
                            println!("Checking ({},{}) for {}", pt2.0, pt2.1, pt.level + 1);
                            let pt2 = Point { level: grid[pt2.1][pt2.0], x: pt2.0, y: pt2.1 };
                            if pt2.level == pt.level+1 {//} && v.insert(pt.clone()) {
                                if pt2.level == 9 {
                                    score += 1;
                                    v.insert(pt2.clone());
                                } else {
                                    trail_points.push_back(pt2);
                                }
                            }
                        } 
                    }
                }
                total_score += score;//v.len();
            }
        }
    }
    
    total_score
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
        // "0123
        // 1234
        // 8765
        // 9876";

            "89010123
             78121874
             87430965
             96549874
             45678903
             32019012
             01329801
             10456732
             ";

// "1110111
// 1111111
// 1112111
// 6543456
// 7111117
// 8111118
// 9111119";

        let total = calc_trailhead_scores(input.into());
        assert_eq!(36, total);
    }
}