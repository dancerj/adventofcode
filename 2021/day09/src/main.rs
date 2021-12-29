use std::collections::BTreeSet;

fn parse_map(s: &str) -> Vec<Vec<u32>> {
    s.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn find_low_point(map: &Vec<Vec<u32>>) -> Vec<u32> {
    let max_y = map.len() as i32;
    let max_x = map[0].len() as i32;
    let mut result = vec![];
    for x in 0..max_x {
        for y in 0..max_y {
            let current = map[y as usize][x as usize];
            let is_lower = |x, y| {
                if x >= 0 && y >= 0 && x < max_x && y < max_y {
                    if map[y as usize][x as usize] > current {
                        true
                    } else {
                        false
                    }
                } else {
                    /* when on edge */
                    true
                }
            };
            if is_lower(x, y - 1) && is_lower(x - 1, y) && is_lower(x + 1, y) && is_lower(x, y + 1)
            {
                result.push(current + 1);
            }
        }
    }
    result
}

fn get_sum_of_risk(map: &Vec<Vec<u32>>) -> u32 {
    find_low_point(map).iter().sum()
}

fn get_basins(map: &Vec<Vec<u32>>) -> u32{
    let max_y = map.len();
    let max_x = map[0].len();
    let mut unique_id: Vec<Vec<usize>> = map
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, _)| y * 100 + x)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut modified = true;
    while modified {
        modified = false;
        for x in 0..max_x {
            for y in 0..max_y {
                let is_nine_or_edge = |x, y| {
                    if x < max_x && y < max_y {
                        if map[y as usize][x as usize] == 9 {
                            true
                        } else {
                            false
                        }
                    } else {
                        /* when on edge */
                        true
                    }
                };

                let mut maybe_mod = |x1: usize, y1: usize, x2: usize, y2: usize| {
                    if unique_id[y1][x1] != unique_id[y2][x2] {
                        let new = unique_id[y1][x1].min(unique_id[y2][x2]);
                        unique_id[y2][x2] = new;
                        unique_id[y1][x1] = new;
                        modified = true;
                    }
                };

                if !is_nine_or_edge(x, y) {
                    if !is_nine_or_edge(x + 1, y) {
                        maybe_mod(x, y, x + 1, y);
                    }
                    if !is_nine_or_edge(x, y + 1) {
                        maybe_mod(x, y, x, y + 1);
                    }
                    if x != 0 && !is_nine_or_edge(x - 1, y) {
                        maybe_mod(x, y, x - 1, y);
                    }
                    if y != 0 && !is_nine_or_edge(x, y - 1) {
                        maybe_mod(x, y, x, y - 1);
                    }
                } else {
                    unique_id[y][x] = 9999
                }
            }
        }
    }

    unique_id.iter().for_each(|row| {
        println!(
            "{}",
            row.iter()
                .map(|x| format!("{:4} ", x))
                .collect::<Vec<String>>()
                .join("")
        )
    });

    let mut survived_unique_ids = unique_id
        .clone()
        .into_iter()
        .flatten()
        .collect::<BTreeSet<usize>>();
    survived_unique_ids.remove(&9999);
    let mut per_id_region = survived_unique_ids
        .iter()
        .map(|x| {
            unique_id
                .iter()
                .flatten()
                .map(|y| if x == y { 1 } else { 0 })
                .sum()
        })
        .collect::<Vec<u32>>();
    per_id_region.sort();
    println!("{:?}", per_id_region);
    per_id_region.iter().rev().take(3).product::<u32>()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    const COMMANDS: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_parse_str() {
        let map = parse_map(COMMANDS);
        assert_eq!(map[0], [2, 1, 9, 9, 9, 4, 3, 2, 1, 0]);
        assert_eq!(map.len(), 5);
    }

    #[test]
    fn test_low_point() {
        let map = parse_map(COMMANDS);
        let r = find_low_point(&map);
        assert_eq!(r.len(), 4);
        let r = get_sum_of_risk(&map);
        assert_eq!(r, 15);
    }

    #[test]
    fn test_low_point_real() {
        let map = parse_map(include_str!("input.txt"));
        let r = get_sum_of_risk(&map);
        assert_eq!(r, 603);
    }

    #[test]
    fn test_basin() {
        let map = parse_map(COMMANDS);
        let r = get_basins(&map);
        assert_eq!(r, 1134);
    }

    #[test]
    fn test_basin_real() {
        let map = parse_map(include_str!("input.txt"));
        let r = get_basins(&map);
        assert_eq!(r, 786780);
    }
}
