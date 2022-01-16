struct MapInfo {
    map: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl MapInfo {
    fn new(s: &str) -> Self {
        let map: Vec<Vec<u32>> = s
            .lines()
            .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect())
            .collect();
        let width = map[0].len();
        let height = map.len();
        MapInfo { map, width, height }
    }

    fn cost(&self, at: &(usize, usize)) -> u32 {
        self.map[at.1][at.0]
    }
}

fn shortest_path(
    map: &MapInfo,
    route: &Vec<(usize, usize)>,
    position: (Option<usize>, Option<usize>),
    cost: u32,
) -> Option<u32> {
    if position.0 == None
        || position.1 == None
        || position.0.unwrap() >= map.width
        || position.1.unwrap() >= map.height
    {
        return None;
    }
    let position = (position.0.unwrap(), position.1.unwrap());

    // Do not come back.
    if route.iter().any(|&x| x == position) {
        return None;
    }
    let mut route = route.clone();
    route.push(position);

    // Do not try too much
    if cost > 41 || route.len() > 20 {
        return None;
    }
    let cost = cost + map.cost(&position);
    if position == (map.width - 1, map.height - 1) {
        // Goal!
        return Some(cost);
    }

    // Iterate all others and find minimal route.
    let cost = vec![
        shortest_path(
            map,
            &route,
            (position.0.checked_sub(1), Some(position.1)),
            cost,
        ),
        shortest_path(
            map,
            &route,
            (Some(position.0), position.1.checked_sub(1)),
            cost,
        ),
        shortest_path(map, &route, (Some(position.0 + 1), Some(position.1)), cost),
        shortest_path(map, &route, (Some(position.0), Some(position.1 + 1)), cost),
    ]
    .iter()
    .filter_map(|&x| x)
    .min();
    cost
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    const COMMANDS: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn test_parse() {
        let t = MapInfo::new(COMMANDS);
        assert_eq!(t.map[0][0], 1);
        let cost = shortest_path(&t, &vec![], (Some(0), Some(0)), 0).unwrap() - 1;
        assert_eq!(cost, 40);
    }
}
