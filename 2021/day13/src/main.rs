//use std::collections::HashMap;
use std::collections::HashSet;

#[derive(PartialEq, Debug)]
enum Fold {
    X,
    Y,
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
    let size = positions.iter().map(|(x, y)| x.max(y)).max().unwrap() + 1;
    let mut matrix = vec![vec![0; size]; size];
    positions.iter().for_each(|&(x, y)| {
        matrix[y][x] = 1;
    });

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
        let i = parse(COMMANDS);
        assert!(i.0.contains(&(6, 10)));
        assert_eq!(i.1[0], (Fold::Y, 7));
    }
}
