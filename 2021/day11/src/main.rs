//use std::collections::BTreeSet;
//use std::collections::HashMap;

fn parse(s: &str) -> Vec<Vec<u32>> {
    s.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

struct Context {
    map: Vec<Vec<u32>>,
    xsize: usize,
    ysize: usize,
    total_size: usize,
}

impl Context {
    fn new(map: Vec<Vec<u32>>) -> Self {
        let xsize = map[0].len();
        let ysize = map.len();
        let total_size = xsize * ysize;
        Self {
            map,
            xsize,
            ysize,
            total_size,
        }
    }

    fn adjust(&mut self, x: i32, y: i32) {
        if x >= 0 && y >= 0 && x < self.xsize as i32 && y < self.ysize as i32 {
            self.map[y as usize][x as usize] += 1;
            if self.map[y as usize][x as usize] == 10 {
                // flash
                self.adjust(x - 1, y - 1);
                self.adjust(x - 1, y);
                self.adjust(x - 1, y + 1);
                self.adjust(x, y - 1);
                self.adjust(x, y + 1);
                self.adjust(x + 1, y - 1);
                self.adjust(x + 1, y);
                self.adjust(x + 1, y + 1);
            }
        }
    }
}

fn count_flashes(s: &str, count: u32) -> u32 {
    let mut c = Context::new(parse(s));

    let mut flashes = 0;

    for _ in 0..count {
        // increase first
        for y in 0..c.ysize {
            for x in 0..c.xsize {
                c.adjust(x as i32, y as i32);
            }
        }

        // then count the flashes & reset
        for y in 0..c.ysize {
            for x in 0..c.xsize {
                if c.map[y][x] > 9 {
                    c.map[y][x] = 0;
                    flashes += 1;
                }
            }
        }
    }
    flashes
}

fn find_simultaneous_flashes(s: &str) -> u32 {
    let mut c = Context::new(parse(s));

    let mut count = 0;
    loop {
        count += 1;
        // increase first
        for y in 0..c.ysize {
            for x in 0..c.xsize {
                c.adjust(x as i32, y as i32);
            }
        }

        // then count the flashes & reset
        let mut flashes = 0;
        for y in 0..c.ysize {
            for x in 0..c.xsize {
                if c.map[y][x] > 9 {
                    c.map[y][x] = 0;
                    flashes += 1;
                }
            }
        }
        if flashes == c.total_size {
            return count;
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    const COMMANDS: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_parse() {
        let i = parse(COMMANDS);
        assert_eq!(i[0][0], 5);
    }

    #[test]
    fn test_iterate() {
        let i = count_flashes(COMMANDS, 100);
        assert_eq!(i, 1656);
    }

    #[test]
    fn test_iterate_real() {
        let i = count_flashes(include_str!("input.txt"), 100);
        assert_eq!(i, 1713);
    }

    #[test]
    fn test_find_simultaneous_flashes() {
        let i = find_simultaneous_flashes(COMMANDS);
        assert_eq!(i, 195);
    }

    #[test]
    fn test_find_simultaneous_flashes_real() {
        let i = find_simultaneous_flashes(include_str!("input.txt"));
        assert_eq!(i, 502);
    }
}
