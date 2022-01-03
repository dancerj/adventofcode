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
}

impl Context {
    fn new(map: Vec<Vec<u32>>) -> Self {
        let mut ret = Self {
            map,
            xsize: 0,
            ysize: 0,
        };
        ret.xsize = ret.map[0].len();
        ret.ysize = ret.map.len();
        ret
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
                self.adjust(x, y + 2);
                self.adjust(x + 1, y - 1);
                self.adjust(x + 1, y);
                self.adjust(x + 1, y + 1);
            }
        }
    }
}

fn count_flashes(s: &str) -> u32 {
    let mut c = Context::new(parse(s));

    for y in 0..c.ysize {
        for x in 0..c.xsize {
            c.adjust(x as i32, y as i32);
        }
    }
    0
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
        let i = count_flashes(COMMANDS);
        assert_eq!(i, 5);
    }
}
