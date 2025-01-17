use std::collections::BTreeSet;

fn main() {
    println!("Hello, world!");
}

fn parse(s: &str) -> Vec<u32> {
    // Return total calories per elf.
    let mut a = vec![0];
    let mut n = 0;
    s.lines().for_each(|line| {
        if line == "" {
            n = n + 1;
            a.push(0);
        } else {
            a[n] = a[n] + line.parse::<u32>().unwrap();
        }
    });
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    const ELVES: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_parse_str() {
        let simple = parse(ELVES);
        assert_eq!(simple[0], 6000);
        assert_eq!(simple, vec![6000, 4000, 11000, 24000, 10000]);
        assert_eq!(simple.len(), 5);
    }

    #[test]
    fn test_parse_str_part1() {
        let elves = parse(include_str!("input.txt"));
        let m = *elves.iter().max().unwrap();
        assert_eq!(70296, m);
    }

    #[test]
    fn test_parse_str_part2() {
        let elves = parse(include_str!("input.txt"));
        let mut m = elves.into_iter().collect::<BTreeSet<u32>>();
        let top_three = m.pop_last().unwrap() + m.pop_last().unwrap() + m.pop_last().unwrap();
        assert_eq!(top_three, 205381);
    }
}
