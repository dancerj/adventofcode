use std::collections::HashMap;
use std::collections::HashSet;

fn parse(s: &str) -> Vec<(String, String)> {
    s.lines()
        .map(|line| {
            let mut i = line.split("-");
            (i.next().unwrap().to_string(), i.next().unwrap().to_string())
        })
        .collect()
}

fn parse_map(s: &str) -> HashMap<&str, Vec<&str>> {
    let mut result = HashMap::new();
    for line in s.lines() {
        let mut i = line.split("-");
        let key = i.next().unwrap();
        let value = i.next().unwrap();
        let e = result.entry(key).or_insert(vec![]);
        e.push(value);
        let e = result.entry(value).or_insert(vec![]);
        e.push(key);
    }
    result
}

fn routes(s: &str) -> u32 {
    let map = parse_map(s);
    println!(" {:?}", map);

    fn route(s: &str, prev: Vec<&str>, map: &HashMap<&str, Vec<&str>>) -> u32 {
        let mut result = 0;
        let mut prev = prev.clone();
        prev.push(s);
        let seen = prev.iter().copied().collect::<HashSet<&str>>();
        match map.get(s) {
            Some(items) => {
                for item in items {
                    // println!("{:?}", item);
                    if item == &"end" {
                        result += 1;
                        // println!("complete {:?}", prev);
                        continue;
                    }
                    if item.chars().nth(0).unwrap().is_ascii_lowercase() && seen.contains(item) {
                        // println!("skip {:?}", item);
                        continue;
                    }
                    // println!("go {:?}", item);
                    result += route(item, prev.clone(), map)
                }
                result
            }
            _ => 0,
        }
    }

    route("start", vec![], &map)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    const COMMANDS: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn test_parse() {
        let i = parse(COMMANDS);
        assert_eq!(i[0].0, "fs");
    }

    #[test]
    fn test_parse_map() {
        let i = parse_map(COMMANDS);
        assert_eq!(i["fs"][0], "end");
    }

    #[test]
    fn test_iterate() {
        let i = routes(COMMANDS);
        assert_eq!(i, 1656);
    }
}
