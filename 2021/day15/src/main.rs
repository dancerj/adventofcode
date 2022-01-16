struct MapInfo {
    map: Vec<Vec<u32>>,
    tentative_cost: Vec<Vec<u32>>,
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
        let tentative_cost = vec![vec![u32::MAX; width]; height];
        MapInfo {
            map,
            tentative_cost,
            width,
            height,
        }
    }

    fn cost(&self, at: &(usize, usize)) -> u32 {
        self.map[at.1][at.0]
    }

    fn shortest_path(
        &mut self,
        position: (Option<usize>, Option<usize>),
        cost: u32,
    ) -> Option<u32> {
        if position.0 == None
            || position.1 == None
            || position.0.unwrap() >= self.width
            || position.1.unwrap() >= self.height
        {
            return None;
        }
        let position = (position.0.unwrap(), position.1.unwrap());

        let cost = cost + self.cost(&position);

        if cost < self.tentative_cost[position.1][position.0] {
            self.tentative_cost[position.1][position.0] = cost;
        } else {
            // There was already someone who visited this node with lower cost.
            return None;
        }

        if position == (self.width - 1, self.height - 1) {
            // Goal!
            return Some(cost);
        }

        // Iterate all others and find minimal route.
        let cost = vec![
            self.shortest_path((position.0.checked_sub(1), Some(position.1)), cost),
            self.shortest_path((Some(position.0), position.1.checked_sub(1)), cost),
            self.shortest_path((Some(position.0 + 1), Some(position.1)), cost),
            self.shortest_path((Some(position.0), Some(position.1 + 1)), cost),
        ]
        .iter()
        .filter_map(|&x| x)
        .min();
        cost
    }
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
        let mut t = MapInfo::new(COMMANDS);
        assert_eq!(t.map[0][0], 1);
        let cost = t.shortest_path((Some(0), Some(0)), 0).unwrap() - 1;
        assert_eq!(cost, 40);
    }

    #[test]
    fn test_parse_real() {
        let mut t = MapInfo::new(include_str!("input.txt"));
        assert_eq!(t.width, 100);
        assert_eq!(t.height, 100);
        assert_eq!(t.map[0][0], 3);
        // let cost = t.shortest_path((Some(0), Some(0)), 0).unwrap() - 1;
        // assert_eq!(cost, 40);
    }
}
