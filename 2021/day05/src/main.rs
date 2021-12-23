fn split_str(s: &str) -> (usize, usize) {
    let result: Vec<usize> = s.split(",").map(|x| x.parse().unwrap()).collect();
    (result[0], result[1])
}

fn parse_commands(s: &str) -> Vec<((usize, usize), (usize, usize))> {
    s.lines()
        .map(|s| {
            let mut s = s.split(' ');
            match (s.next(), s.next(), s.next()) {
                (Some(before), Some("->"), Some(after)) => (before, after),
                _ => panic!("wrong input"),
            }
        })
        .map(|(before, after)| (split_str(before), split_str(after)))
        .collect()
}

fn parse1(s: &str) -> Vec<Vec<u32>> {
    let commands = parse_commands(s);
    let max_value: usize = commands
        .iter()
        .map(|&((x1, y1), (x2, y2))| x1.max(x2.max(y1.max(y2))) as usize)
        .max()
        .unwrap();
    let mut table = vec![vec![0; max_value + 1]; max_value + 1];
    commands.iter().for_each(|&((x1, y1), (x2, y2))| {
        if (x1 == x2) || (y1 == y2) {
            // only process horizontal or vertical lines.
            if x1 == x2 {
                (y1.min(y2)..=y1.max(y2)).for_each(|y| {
                    table[x1][y] += 1;
                });
            } else {
                (x1.min(x2)..=x1.max(x2)).for_each(|x| {
                    table[x][y1] += 1;
                });
            }
        }
    });
    table
}

fn parse2(s: &str) -> Vec<Vec<u32>> {
    let commands = parse_commands(s);
    let max_value: usize = commands
        .iter()
        .map(|&((x1, y1), (x2, y2))| x1.max(x2.max(y1.max(y2))) as usize)
        .max()
        .unwrap();
    let mut table = vec![vec![0; max_value + 1]; max_value + 1];

    commands.iter().for_each(|&((x1, y1), (x2, y2))| {
        // only process horizontal, vertical or 45 degrees diagonal lines.
        if x1 == x2 {
            (y1.min(y2)..=y1.max(y2)).for_each(|y| {
                table[x1][y] += 1;
            });
        } else {
            // Swap around to make things go right.
            let (x1, y1, x2, y2) = if x1 > x2 {
                (x2, y2, x1, y1)
            } else {
                (x1, y1, x2, y2)
            };

            println!("  {},{} - {},{}", x1, y1, x2, y2);
            let offset: i32 = x2 as i32 - x1 as i32;
            let ydirection = (y2 as i32 - y1 as i32) / offset;
            // TODO assert direction = -1, 0, or 1.
            (0..=offset).for_each(|i| {
                let x = (x1 as i32 + i) as usize;
                let y = (y1 as i32 + ydirection * i) as usize;
                // println!("  {},{}", x, y);
                table[x][y] += 1;
            });
        }
    });
    table
}

fn count_2plus(map: &Vec<Vec<u32>>) -> usize {
    map.iter()
        .map(|x| {
            x.iter()
                .filter_map(|&x| if x >= 2 { Some(1) } else { None })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    const COMMANDS: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_split_str() {
        assert_eq!(split_str("12,30").0, 12);
        assert_eq!(split_str("12,30").1, 30);
    }

    #[test]
    fn test_part1() {
        let map = parse1(COMMANDS);
        let count = count_2plus(&map);
        // println!(" {:?} {}", map, count);
        // assert_eq!(map, [[1]]);
        assert_eq!(count, 5);
    }

    #[test]
    fn test_part1_real() {
        let map = parse1(include_str!("input.txt"));
        let count = count_2plus(&map);
        // println!(" {:?} {}", map, count);
        // assert_eq!(map, [[1]]);
        assert_eq!(count, 6283);
    }

    #[test]
    fn test_part2() {
        let map = parse2(COMMANDS);
        let count = count_2plus(&map);
        map.iter().for_each(|x| println!("{:?}", x));
        assert_eq!(count, 12);
    }

    #[test]
    fn test_part2_real() {
        let map = parse2(include_str!("input.txt"));
        let count = count_2plus(&map);
        assert_eq!(count, 18864);
    }
}
