struct TargetArea {
    x: (i64, i64),
    y: (i64, i64),
}

fn get_highest_if_it_lands(mut dx: i64, mut dy: i64, t: &TargetArea) -> Option<i64> {
    let mut x = 0;
    let mut y = 0;
    assert!(dx > 0);
    let limit_x = t.x.0.max(t.x.1);
    let limit_y = t.y.0.min(t.y.1);
    let mut max_y = 0;
    while x < limit_x && y > limit_y {
        x += dx;
        y += dy;
        dy -= 1;
        if dx > 0 {
            dx -= 1;
        }
        max_y = max_y.max(y);
        // println!("{} {} {} {}", x, y, dx, dy);
        if x >= t.x.0 && x <= t.x.1 && y >= t.y.0 && y <= t.y.1 {
            return Some(max_y);
        }
    }
    None
}

fn find_highest_value(t: &TargetArea) -> (i64, i64) {
    let mut max_y = 0;
    let mut count = 0;
    for dx in 1..300 {
        for dy in -1000..500 {
            // TODO probably I should have searched for the range in code.
            if let Some(v) = get_highest_if_it_lands(dx, dy, t) {
                // Got a result.
                println!("{} {} {}", dx, dy, v);
                max_y = max_y.max(v);
                count += 1
            }
        }
    }
    (max_y, count)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let t = TargetArea {
            x: (20, 30),
            y: (-10, -5),
        };
        assert_eq!(get_highest_if_it_lands(6, 3, &t), Some(6));
        assert_eq!(get_highest_if_it_lands(9, 0, &t), Some(0));
        assert_eq!(find_highest_value(&t).0, 45);
    }

    #[test]
    fn test_parse_real() {
        let t = TargetArea {
            x: (79, 137),
            y: (-176, -117),
        };
        assert_eq!(get_highest_if_it_lands(15, 0, &t), Some(0));
        let (highest, count) = find_highest_value(&t);
        assert_eq!(highest, 15400);
        assert_eq!(count, 5844);
    }
}
