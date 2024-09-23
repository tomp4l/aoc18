use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: &str) -> Result<DayResult, String> {
        let serial = input.parse::<i32>().map_err(|e| e.to_string())?;
        let (x, y, _) = max_power_level(serial);

        let part1 = format!("{},{}", x, y);
        let (x, y, size, _) = max_power_level_any_size(serial);

        let part2 = format!("{},{},{}", x, y, size);

        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}

fn power_level(x: i32, y: i32, serial: i32) -> i32 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += serial;
    power_level *= rack_id;
    power_level = (power_level / 100) % 10;
    power_level - 5
}

fn max_power_level(serial: i32) -> (i32, i32, i32) {
    let mut grid = vec![vec![0; 300]; 300];
    for x in 0..300 {
        for y in 0..300 {
            grid[x][y] = power_level(x as i32, y as i32, serial);
        }
    }

    let mut max_power = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for x in 0..300 - 2 {
        for y in 0..300 - 2 {
            let mut power = 0;
            for dx in 0..3 {
                for dy in 0..3 {
                    power += grid[x + dx][y + dy];
                }
            }
            if power > max_power {
                max_power = power;
                max_x = x;
                max_y = y;
            }
        }
    }

    (max_x as i32, max_y as i32, max_power)
}

fn max_power_level_any_size(serial: i32) -> (i32, i32, i32, i32) {
    // https://en.wikipedia.org/wiki/Summed-area_table
    let mut partial_sums = vec![vec![0; 301]; 301];

    for y in 1..=300 {
        for x in 1..=300 {
            let p = power_level(x as i32, y as i32, serial);
            partial_sums[y][x] =
                p + partial_sums[y - 1][x] + partial_sums[y][x - 1] - partial_sums[y - 1][x - 1];
        }
    }

    let mut max_power: i32 = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_size = 0;
    for s in 1..=300 {
        for y in s..=300 {
            for x in s..=300 {
                let total = partial_sums[y][x] - partial_sums[y - s][x] - partial_sums[y][x - s]
                    + partial_sums[y - s][x - s];
                if total > max_power {
                    max_power = total;
                    max_x = x - s + 1;
                    max_y = y - s + 1;
                    max_size = s;
                }
            }
        }
    }

    (max_x as i32, max_y as i32, max_size as i32, max_power)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let instance = Instance;
        let input = "18";
        let expected = DayResult {
            part1: "33,45".to_owned(),
            part2: Some("90,269,16".to_owned()),
        };
        assert_eq!(instance.run(input), Ok(expected));
    }
}
