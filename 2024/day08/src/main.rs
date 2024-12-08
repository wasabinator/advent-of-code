use std::{collections::{HashMap, HashSet}, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt not found");

    let s = input.as_str();

    // Calculate part 1 answer
    let total = calc_antinodes(s, false);
    println!("Answer part 1: {}", total);

    // Calculate part 1 answer
    let total = calc_antinodes(s, true);
    println!("Answer part 1: {}", total);
}

fn calc_antinodes(input: &str, all_inline: bool) -> usize {
    let rows: Vec<&str> = input.lines().map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
    let grid: Vec<Vec<char>> = rows.into_iter().map(|s|s.chars().collect()).collect();
    let width = grid[0].len();
    let height = grid.len();
    
    let mut antennas: HashMap<char, HashSet<(isize, isize)>> = HashMap::new();
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new(); 
    
    for x in 0..width {
        for y in 0..height {
            let ch = grid[y][x];
            if ch != '.' {
                let pt = (x as isize, y as isize);
                let antenna = antennas.entry(ch).or_insert(HashSet::new());
                for a in antenna.iter() {
                    let diff_x = a.0 - pt.0;
                    let diff_y = a.1 - pt.1;

                    if all_inline {
                        let mut antinode = (a.0, a.1);
                        while antinode.0 >= 0 && antinode.0 < width as isize && antinode.1 >= 0 && antinode.1 < height as isize {
                            antinodes.insert(antinode.clone());
                            antinode = (antinode.0 + diff_x, antinode.1 + diff_y);
                        }
                        let mut antinode = (pt.0, pt.1);
                        while antinode.0 >= 0 && antinode.0 < width as isize && antinode.1 >= 0 && antinode.1 < height as isize {
                            antinodes.insert(antinode.clone());
                            antinode = (antinode.0 - diff_x, antinode.1 - diff_y);
                        }
                    } else {
                        for antinode in vec![(a.0 + diff_x, a.1 + diff_y), (pt.0 - diff_x, pt.1 - diff_y)] {
                            if antinode.0 >= 0 && antinode.0 < width as isize && antinode.1 >= 0 && antinode.1 < height as isize {
                                antinodes.insert(antinode);
                            }
                        }
                    }
                }
                antenna.insert(pt);
            }
        }
    }
    
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = 
            "............
             ........0...
             .....0......
             .......0....
             ....0.......
             ......A.....
             ............
             ............
             ........A...
             .........A..
             ............
             ............";

        let total= calc_antinodes(input.into(), false);
        assert_eq!(14, total);
    }

    #[test]
    fn test_part2() {
        let input = 
            "............
             ........0...
             .....0......
             .......0....
             ....0.......
             ......A.....
             ............
             ............
             ........A...
             .........A..
             ............
             ............";

        let total= calc_antinodes(input.into(), true);
        assert_eq!(34, total);
    }
}