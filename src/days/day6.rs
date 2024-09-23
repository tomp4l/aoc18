use std::{str::FromStr, vec};

use super::day::*;

pub struct Instance {
    target_distance: usize,
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            target_distance: 10_000,
        }
    }
}

impl Day for Instance {
    fn run(&self, input: &str) -> Result<DayResult, String> {
        let coords = input
            .lines()
            .map(|line| line.parse::<Coord>())
            .collect::<Result<Vec<_>, _>>()?;
        let part1 = part1(&coords).to_string();
        Ok(DayResult {
            part1,
            part2: Some(part2(&coords, self.target_distance).to_string()),
        })
    }
}

struct Coord {
    x: i32,
    y: i32,
}

impl FromStr for Coord {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',').collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(format!("invalid coord: {}", s));
        }
        let x = parts[0]
            .parse::<i32>()
            .map_err(|e| format!("invalid x: {}", e))?;
        let y = parts[1]
            .trim()
            .parse::<i32>()
            .map_err(|e| format!("invalid y: {}", e))?;
        Ok(Coord { x, y })
    }
}

impl Coord {
    fn manhattan_distance(&self, other: &Coord) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn part1(coords: &[Coord]) -> usize {
    let mut counts = vec![0; coords.len()];
    let min_x = coords.iter().map(|c| c.x).min().unwrap();
    let max_x = coords.iter().map(|c| c.x).max().unwrap();
    for x in min_x..=max_x {
        let min_y = coords.iter().map(|c| c.y).min().unwrap();
        let max_y = coords.iter().map(|c| c.y).max().unwrap();
        for y in min_y..=max_y {
            let coord = Coord { x, y };
            let mut distances = coords
                .iter()
                .map(|c| c.manhattan_distance(&coord))
                .enumerate()
                .collect::<Vec<_>>();
            distances.sort_by_key(|t| t.1);
            if distances[0].1 != distances[1].1 {
                let min_index = distances[0].0;
                if counts[min_index] == usize::MAX
                    || x == min_x
                    || x == max_x
                    || y == min_y
                    || y == max_y
                {
                    counts[min_index] = usize::MAX;
                } else {
                    counts[min_index] += 1;
                }
            }
        }
    }
    counts
        .into_iter()
        .filter(|c| *c != usize::MAX)
        .max()
        .unwrap()
}

fn part2(coords: &[Coord], target_distance: usize) -> usize {
    let t32 = (target_distance / coords.len()) as i32;
    let min_x = coords.iter().map(|c| c.x).min().unwrap() - t32;
    let max_x = coords.iter().map(|c| c.x).max().unwrap() + t32;
    let min_y = coords.iter().map(|c| c.x).min().unwrap() - t32;
    let max_y = coords.iter().map(|c| c.x).max().unwrap() + t32;

    let mut count = 0;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let coord = Coord { x, y };
            let total_distance: i32 = coords.iter().map(|c| c.manhattan_distance(&coord)).sum();
            if total_distance < target_distance as i32 {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let instance = Instance {
            target_distance: 32,
        };
        let input = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

        assert_eq!(
            instance.run(input),
            Ok(DayResult {
                part1: "17".to_owned(),
                part2: Some("16".to_owned())
            })
        );
    }
}
