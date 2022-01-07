//use std::collections::HashMap;
//use std::collections::HashSet;


fn parse(s:&str) -> (Vec<&str>, Vec<&str>) {
    let mut lines = s.lines();
    let positions:Vec<&str> = lines.by_ref().take_while(|&line| line != "").collect();
    let fold_commands:Vec<&str> = lines.by_ref().collect();
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
        assert_eq!(i.0[0], "6,10");
        assert_eq!(i.1[0], "fold along y=7");
    }
}
