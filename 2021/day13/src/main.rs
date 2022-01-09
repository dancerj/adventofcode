//use std::collections::HashMap;
//use std::collections::HashSet;

fn parse(s: &str) -> (Vec<(u32, u32)>, Vec<(&str, u32)>) {
    let mut lines = s.lines();
    let positions: Vec<(u32, u32)> = lines
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
    let size = positions.iter().map(|(x, y)| x.max(y)).max();
    // let mut matrix = vec![vec![]];

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
                            ("x", value)
                        }
                        ("y", value) => {
                            let value = value.parse().unwrap();
                            ("y", value)
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
        assert_eq!(i.0[0], (6, 10));
        assert_eq!(i.1[0], ("y", 7));
    }
}
