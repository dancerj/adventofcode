use std::collections::HashSet;

#[derive(PartialEq, Debug)]
enum Fold {
    X,
    Y,
}

fn printable(positions: &HashSet<(usize, usize)>) -> Vec<String> {
    let size = positions.iter().map(|(x, y)| x.max(y)).max().unwrap() + 1;
    (0..size)
        .map(|y| {
            (0..size)
                .map(|x| {
                    if positions.contains(&(x, y)) {
                        '#'
                    } else {
                        ' '
                    }
                })
                .collect()
        })
        .collect()
}

fn print_lines(s: &Vec<String>) {
    s.iter().for_each(|x| println!("{}", x));
}

fn parse(s: &str) -> (HashSet<(usize, usize)>, Vec<(Fold, usize)>) {
    let mut lines = s.lines();
    let positions: HashSet<(usize, usize)> = lines
        .by_ref()
        .map_while(|line| {
            if line != "" {
                let mut words = line.split(",");
                Some((
                    words.next().unwrap().parse().unwrap(),
                    words.next().unwrap().parse().unwrap(),
                ))
            } else {
                None
            }
        })
        .collect();

    let fold_commands: Vec<_> = lines
        .map(|line| {
            let mut words = line.split(" ");
            match (
                words.next().unwrap(),
                words.next().unwrap(),
                words.next().unwrap(),
            ) {
                ("fold", "along", command) => {
                    let mut words = command.split("=");
                    match (words.next().unwrap(), words.next().unwrap()) {
                        ("x", value) => {
                            let value = value.parse().unwrap();
                            (Fold::X, value)
                        }
                        ("y", value) => {
                            let value = value.parse().unwrap();
                            (Fold::Y, value)
                        }
                        _ => {
                            panic!();
                        }
                    }
                }
                _ => {
                    panic!();
                }
            }
        })
        .collect();
    (positions, fold_commands)
}

fn fold_once(positions: &HashSet<(usize, usize)>, fold: &(Fold, usize)) -> HashSet<(usize, usize)> {
    let positions = positions
        .iter()
        .map(|&(x, y)| match *fold {
            (Fold::X, value) => {
                if x > value {
                    (value - (x - value), y)
                } else {
                    (x, y)
                }
            }
            (Fold::Y, value) => {
                if y > value {
                    (x, value - (y - value))
                } else {
                    (x, y)
                }
            }
        })
        .collect();
    positions
}

fn fold_multiple(
    positions: HashSet<(usize, usize)>,
    fold: &Vec<(Fold, usize)>,
) -> HashSet<(usize, usize)> {
    let mut positions = positions;
    fold.iter().for_each(|fold| {
        positions = fold_once(&positions, fold);
    });
    positions
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    const COMMANDS: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_parse() {
        let (positions, fold) = parse(COMMANDS);
        assert!(positions.contains(&(6, 10)));
        assert_eq!(fold[0], (Fold::Y, 7));
        let positions2 = fold_once(&positions, &fold[0]);
        assert_eq!(positions2.len(), 17);
    }

    #[test]
    fn test_parse_real() {
        let (positions, fold) = parse(include_str!("input.txt"));
        let positions2 = fold_once(&positions, &fold[0]);
        assert_eq!(positions2.len(), 602);
    }

    #[test]
    fn test_parse2() {
        let (positions, fold) = parse(COMMANDS);
        let positions2 = fold_multiple(positions, &fold);
        print_lines(&printable(&positions2));
        assert_eq!(
            printable(&positions2),
            vec!["#####", "#   #", "#   #", "#   #", "#####"],
        );
    }

    #[test]
    fn test_parse2_real() {
        let (positions, fold) = parse(include_str!("input.txt"));
        let positions2 = fold_multiple(positions, &fold);
        let p = printable(&positions2);
        print_lines(&p);
        assert_eq!(p[0], " ##   ##  ####   ## #  # ####  ##  #  #");
        // panic!();
        // CAFJHZCK
    }
}
