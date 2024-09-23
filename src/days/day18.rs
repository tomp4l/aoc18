use std::{collections::HashMap, str::FromStr};

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: &str) -> Result<DayResult, String> {
        let mut lumber_collection = input.parse::<LumberCollection>()?;
        let part1 = lumber_collection.nth(9).unwrap().to_string();
        let part2 = part2(input).to_string();
        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}

fn part2(input: &str) -> usize {
    let lumber_collection = input.parse::<LumberCollection>().unwrap();
    let mut seen = HashMap::new();
    let mut cycle = None;
    for (i, value) in lumber_collection.enumerate() {
        if let Some((j, prev_cyle)) = seen.get(&value) {
            if let Some(size) = prev_cyle {
                if i - j == *size {
                    cycle = Some((*j, i));
                    break;
                }
            }
            seen.insert(value, (i, Some(i - j)));
        } else {
            seen.insert(value, (i, None));
        }
    }

    let (start, end) = cycle.unwrap();
    let cycle_length = end - start;
    let remaining = (1_000_000_000 - start) % cycle_length;
    let mut lumber_collection = input.parse::<LumberCollection>().unwrap();
    for _ in 0..start + remaining {
        lumber_collection.step();
    }
    lumber_collection.resouce_value()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Acre {
    Open,
    Trees,
    Lumberyard,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn neighbours(&self) -> Vec<Coord> {
        vec![
            Coord {
                x: self.x - 1,
                y: self.y - 1,
            },
            Coord {
                x: self.x,
                y: self.y - 1,
            },
            Coord {
                x: self.x + 1,
                y: self.y - 1,
            },
            Coord {
                x: self.x - 1,
                y: self.y,
            },
            Coord {
                x: self.x + 1,
                y: self.y,
            },
            Coord {
                x: self.x - 1,
                y: self.y + 1,
            },
            Coord {
                x: self.x,
                y: self.y + 1,
            },
            Coord {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
    }
}

struct LumberCollection {
    grid: HashMap<Coord, Acre>,
}

impl Iterator for LumberCollection {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.step();
        Some(self.resouce_value())
    }
}

impl LumberCollection {
    fn step(&mut self) {
        let mut new_grid = self.grid.clone();

        for (coord, acre) in &self.grid {
            let trees = coord
                .neighbours()
                .iter()
                .filter_map(|n| self.grid.get(n))
                .filter(|a| **a == Acre::Trees)
                .count();
            let lumberyards = coord
                .neighbours()
                .iter()
                .filter_map(|n| self.grid.get(n))
                .filter(|a| **a == Acre::Lumberyard)
                .count();

            match acre {
                Acre::Open => {
                    if trees >= 3 {
                        new_grid.insert(*coord, Acre::Trees);
                    }
                }
                Acre::Trees => {
                    if lumberyards >= 3 {
                        new_grid.insert(*coord, Acre::Lumberyard);
                    }
                }
                Acre::Lumberyard => {
                    if lumberyards == 0 || trees == 0 {
                        new_grid.insert(*coord, Acre::Open);
                    }
                }
            }
        }

        self.grid = new_grid;
    }

    fn resouce_value(&self) -> usize {
        let trees = self.grid.values().filter(|a| **a == Acre::Trees).count();
        let lumberyards = self
            .grid
            .values()
            .filter(|a| **a == Acre::Lumberyard)
            .count();
        trees * lumberyards
    }
}

impl FromStr for LumberCollection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    let coord = Coord {
                        x: x as i32,
                        y: y as i32,
                    };
                    let acre = match c {
                        '.' => Acre::Open,
                        '|' => Acre::Trees,
                        '#' => Acre::Lumberyard,
                        _ => return Err(format!("invalid acre: {}", c)),
                    };
                    Ok((coord, acre))
                })
            })
            .collect::<Result<_, _>>()?;
        Ok(Self { grid })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let instance = Instance;
        let input = ".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";
        let expected = DayResult {
            part1: "1147".to_owned(),
            part2: Some("0".to_owned()),
        };
        assert_eq!(instance.run(input), Ok(expected));
    }
}
