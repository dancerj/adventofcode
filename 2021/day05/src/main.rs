fn split_str(s: &str) -> (usize, usize) {
    let result: Vec<usize> = s.split(",").map(|x| x.parse().unwrap()).collect();
    (result[0], result[1])
}

fn parse1(s: &str) -> Vec<Vec<u32>> {
    const SIZE: usize = 1000;
    let mut table = vec![vec![0; SIZE]; SIZE];
    s.lines()
        .map(|s| {
            let mut s = s.split(' ');
            match (s.next(), s.next(), s.next()) {
                (Some(before), Some("->"), Some(after)) => (before, after),
                _ => panic!("wrong input"),
            }
        })
        .map(|(before, after)| (split_str(before), split_str(after)))
        .for_each(|((x1, y1), (x2, y2))| {
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
}
