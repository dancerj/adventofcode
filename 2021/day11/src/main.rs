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
}
