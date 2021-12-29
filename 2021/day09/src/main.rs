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
}
