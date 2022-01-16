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

    fn shortest_path(&mut self) -> u32 {
        // Starting position is 0.
        self.tentative_cost[0][0] = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                let cost_at_position = self.map[y][x];

                let cost = vec![
                    (x.checked_sub(1), Some(y)),
                    (Some(x), y.checked_sub(1)),
                    (Some(x + 1), Some(y)),
                    (Some(x), Some(y + 1)),
                ]
                .iter()
                .filter_map(|&(newx, newy)| {
                    if let Some(newx) = newx {
                        if let Some(newy) = newy {
                            if newx < self.width && newy < self.height {
                                return self.tentative_cost[newy][newx]
                                    .checked_add(cost_at_position);
                            }
                        }
                    }
                    None
                })
                .min();
                if let Some(cost) = cost {
                    self.tentative_cost[y][x] = cost;
                    if x == self.width - 1 && y == self.height - 1 {
                        // Goal!
                        return cost;
                    }
                }
            }
        }
        panic!();
        //        println!("{:?}", self.tentative_cost);
        //None
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
        let cost = t.shortest_path();
        assert_eq!(cost, 40);
    }

    #[test]
    fn test_parse_real() {
        let mut t = MapInfo::new(include_str!("input.txt"));
        assert_eq!(t.width, 100);
        assert_eq!(t.height, 100);
        assert_eq!(t.map[0][0], 3);
        let cost = t.shortest_path();
        assert_eq!(cost, 447);
    }
}
