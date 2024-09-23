use std::{collections::HashSet, hash::Hash, str::FromStr};

use super::day::*;

pub struct Instance {
    verbose: bool,
}

impl Default for Instance {
    fn default() -> Self {
        Self { verbose: false }
    }
}

impl Day for Instance {
    fn run(&self, input: &str) -> Result<DayResult, String> {
        let mut ground = input.parse::<Ground>()?;
        ground.simulate();
        let part1 = ground.count_water().to_string();
        let part2 = ground.count_standing_water().to_string();

        if self.verbose {
            ground.print();
        }
        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Ground {
    clay: HashSet<Coord>,
    min_y: i32,
    max_y: i32,
    flowing_water: HashSet<Coord>,
    standing_water: HashSet<Coord>,
}
impl Ground {
    fn new(clay: HashSet<Coord>) -> Self {
        let min_y = clay.iter().map(|c| c.y).min().unwrap();
        let max_y = clay.iter().map(|c| c.y).max().unwrap();

        Self {
            clay,
            min_y,
            max_y,
            flowing_water: HashSet::new(),
            standing_water: HashSet::new(),
        }
    }

    fn count_water(&self) -> usize {
        self.flowing_water
            .union(&self.standing_water)
            .filter(|c| c.y >= self.min_y && c.y <= self.max_y)
            .count()
    }

    fn count_standing_water(&self) -> usize {
        self.standing_water
            .iter()
            .filter(|c| c.y >= self.min_y && c.y <= self.max_y)
            .count()
    }

    fn simulate(&mut self) {
        let mut to_fill = vec![Coord { x: 500, y: 0 }];
        while !to_fill.is_empty() {
            let mut next_to_fill = vec![];
            for coord in to_fill {
                if coord.y > self.max_y {
                    continue;
                }
                if self.clay.contains(&coord) {
                    continue;
                }
                if self.standing_water.contains(&coord) {
                    continue;
                }
                self.flowing_water.insert(coord);
                let below = Coord {
                    x: coord.x,
                    y: coord.y + 1,
                };

                if !self.clay.contains(&below) && !self.standing_water.contains(&below) {
                    next_to_fill.push(below);
                    continue;
                }

                let mut left_wall = None;
                let mut right_wall = None;

                fn scan(
                    clay: &HashSet<Coord>,
                    standing_water: &HashSet<Coord>,
                    flowing_water: &mut HashSet<Coord>,
                    next_to_fill: &mut Vec<Coord>,
                    wall: &mut Option<Coord>,
                    mut coord: Coord,
                    is_left: bool,
                ) {
                    let scan_direction = if is_left { -1 } else { 1 };
                    loop {
                        coord = Coord {
                            x: coord.x + scan_direction,
                            y: coord.y,
                        };
                        if clay.contains(&coord) {
                            *wall = Some(coord);
                            return;
                        }
                        flowing_water.insert(coord);
                        let below = Coord {
                            x: coord.x,
                            y: coord.y + 1,
                        };
                        if !clay.contains(&below) && !standing_water.contains(&below) {
                            next_to_fill.push(below);
                            break;
                        }
                    }
                }

                scan(
                    &self.clay,
                    &self.standing_water,
                    &mut self.flowing_water,
                    &mut next_to_fill,
                    &mut left_wall,
                    coord,
                    true,
                );

                scan(
                    &self.clay,
                    &self.standing_water,
                    &mut self.flowing_water,
                    &mut next_to_fill,
                    &mut right_wall,
                    coord,
                    false,
                );

                if let (Some(left_wall), Some(right_wall)) = (left_wall, right_wall) {
                    for x in (left_wall.x + 1)..right_wall.x {
                        self.standing_water.insert(Coord { x, y: coord.y });
                    }
                    let above = Coord {
                        x: coord.x,
                        y: coord.y - 1,
                    };
                    next_to_fill.push(above);
                }
            }
            to_fill = next_to_fill;
        }
    }

    fn print(&self) {
        let min_x = self.clay.iter().map(|c| c.x).min().unwrap();
        let max_x = self.clay.iter().map(|c| c.x).max().unwrap();
        let min_y = self.clay.iter().map(|c| c.y).min().unwrap();
        let max_y = self.clay.iter().map(|c| c.y).max().unwrap();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let coord = Coord { x, y };
                if self.standing_water.contains(&coord) {
                    print!("~");
                } else if self.flowing_water.contains(&coord) {
                    print!("|");
                } else if self.clay.contains(&coord) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

impl FromStr for Ground {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let clay = s
            .lines()
            .map(|line| {
                let mut parts = line.split(", ");
                let first = parts.next().ok_or("missing first part")?;
                let second = parts.next().ok_or("missing second part")?;
                let first = first.split('=').collect::<Vec<&str>>();
                let second = second.split('=').collect::<Vec<&str>>();
                if first.len() != 2 || second.len() != 2 {
                    return Err("invalid parts".to_owned());
                }
                let is_y_range = first[0] == "x";
                let first = first[1]
                    .parse::<i32>()
                    .map_err(|e| format!("invalid {}: {}", first[0], e))?;
                let second = second[1].split("..").collect::<Vec<&str>>();
                if second.len() != 2 {
                    return Err("invalid second part".to_owned());
                }
                let second_start = second[0]
                    .parse::<i32>()
                    .map_err(|e| format!("invalid {}: {}", second[0], e))?;
                let second_end = second[1]
                    .parse::<i32>()
                    .map_err(|e| format!("invalid {}: {}", second[0], e))?;

                Ok((is_y_range, first, second_start..=second_end))
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flat_map(|(is_y_range, c, range)| {
                range.map(move |v| {
                    if is_y_range {
                        Coord { x: c, y: v }
                    } else {
                        Coord { x: v, y: c }
                    }
                })
            })
            .collect();
        Ok(Ground::new(clay))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "x=495, y=2..7\ny=7, x=495..501";
        let expected = vec![
            Coord { x: 495, y: 2 },
            Coord { x: 495, y: 3 },
            Coord { x: 495, y: 4 },
            Coord { x: 495, y: 5 },
            Coord { x: 495, y: 6 },
            Coord { x: 495, y: 7 },
            Coord { x: 496, y: 7 },
            Coord { x: 497, y: 7 },
            Coord { x: 498, y: 7 },
            Coord { x: 499, y: 7 },
            Coord { x: 500, y: 7 },
            Coord { x: 501, y: 7 },
        ]
        .into_iter()
        .collect();
        assert_eq!(input.parse::<Ground>().map(|g| g.clay), Ok(expected));
        assert_eq!(input.parse::<Ground>().map(|g| g.min_y), Ok(2));
        assert_eq!(input.parse::<Ground>().map(|g| g.max_y), Ok(7));
    }

    #[test]
    fn example() {
        let instance = Instance { verbose: true };
        let input = "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504
";
        let expected = DayResult {
            part1: "57".to_owned(),
            part2: Some("29".to_owned()),
        };
        assert_eq!(instance.run(input), Ok(expected));
    }
}
