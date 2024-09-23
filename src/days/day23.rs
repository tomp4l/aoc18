use std::str::FromStr;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: &str) -> Result<DayResult, String> {
        let bots = input
            .lines()
            .map(|line| line.parse::<Nanobot>())
            .collect::<Result<Vec<_>, _>>()?;

        let part1 = part1(bots.as_slice()).to_string();
        let part2 = part2(bots.as_slice()).to_string();
        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Nanobot {
    coord: Coord,
    r: i32,
}

impl FromStr for Nanobot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split([',', '>', '<', '='].as_ref())
            .collect::<Vec<&str>>();
        if parts.len() != 8 {
            return Err(format!("invalid nanobot: {}", s));
        }
        let x = parts[2]
            .parse::<i32>()
            .map_err(|e| format!("invalid x: {}", e))?;
        let y = parts[3]
            .parse::<i32>()
            .map_err(|e| format!("invalid y: {}", e))?;
        let z = parts[4]
            .parse::<i32>()
            .map_err(|e| format!("invalid z: {}", e))?;
        let r = parts[7]
            .parse::<i32>()
            .map_err(|e| format!("invalid r: {}", e))?;
        Ok(Nanobot {
            coord: Coord { x, y, z },
            r,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

impl Coord {
    fn distance(&self, other: &Coord) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

impl Nanobot {
    fn in_range(&self, other: &Coord) -> bool {
        let distance = self.coord.distance(other);

        distance <= self.r
    }

    fn almost_in_range(&self, other: &Coord) -> bool {
        let distance = self.coord.distance(other);

        distance <= self.r + 1
    }
}

fn part1(bots: &[Nanobot]) -> i32 {
    let max_bot = bots.iter().max_by_key(|bot| bot.r).unwrap();

    bots.iter()
        .filter(|bot| max_bot.in_range(&bot.coord))
        .count() as i32
}

const DIVISOR: i32 = 10;
const ADDITIONAL_SEARCH: i32 = 20;

fn part2(bots: &[Nanobot]) -> i32 {
    let mut divisor = DIVISOR;
    let max_r = bots.iter().map(|bot| bot.r).max().unwrap();
    while divisor < max_r / ADDITIONAL_SEARCH {
        divisor *= DIVISOR;
    }

    fn scale_bots(bots: &[Nanobot], divisor: i32) -> Vec<Nanobot> {
        bots.iter()
            .map(|bot| Nanobot {
                coord: Coord {
                    x: bot.coord.x / divisor,
                    y: bot.coord.y / divisor,
                    z: bot.coord.z / divisor,
                },
                r: bot.r / divisor,
            })
            .collect::<Vec<_>>()
    }

    let mut scaled_bots = scale_bots(bots, divisor);

    let mut min_x = scaled_bots
        .iter()
        .map(|bot| bot.coord.x - bot.r)
        .min()
        .unwrap();
    let mut max_x = scaled_bots
        .iter()
        .map(|bot| bot.coord.x + bot.r)
        .max()
        .unwrap();
    let mut min_y = scaled_bots
        .iter()
        .map(|bot| bot.coord.y - bot.r)
        .min()
        .unwrap();
    let mut max_y = scaled_bots
        .iter()
        .map(|bot| bot.coord.y + bot.r)
        .max()
        .unwrap();
    let mut min_z = scaled_bots
        .iter()
        .map(|bot| bot.coord.z - bot.r)
        .min()
        .unwrap();
    let mut max_z = scaled_bots
        .iter()
        .map(|bot| bot.coord.z + bot.r)
        .max()
        .unwrap();
    let mut best_coord = Coord { x: 0, y: 0, z: 0 };

    loop {
        let mut max_count = 0;
        let mut best_distance = 0;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    let coord = Coord { x, y, z };
                    let count = scaled_bots
                        .iter()
                        .filter(|bot| {
                            if divisor == 1 {
                                bot.in_range(&coord)
                            } else {
                                bot.almost_in_range(&coord)
                            }
                        })
                        .count();
                    let distance = coord.distance(&Coord { x: 0, y: 0, z: 0 });

                    if count > max_count || (count == max_count && distance < best_distance) {
                        max_count = count;
                        best_coord = coord;
                        best_distance = distance;
                    }
                }
            }
        }

        divisor /= DIVISOR;
        if divisor == 0 {
            break;
        }

        min_x = best_coord.x * DIVISOR - ADDITIONAL_SEARCH;
        max_x = best_coord.x * DIVISOR + ADDITIONAL_SEARCH;
        min_y = best_coord.y * DIVISOR - ADDITIONAL_SEARCH;
        max_y = best_coord.y * DIVISOR + ADDITIONAL_SEARCH;
        min_z = best_coord.z * DIVISOR - ADDITIONAL_SEARCH;
        max_z = best_coord.z * DIVISOR + ADDITIONAL_SEARCH;

        scaled_bots = scale_bots(bots, divisor);
    }

    best_coord.distance(&Coord { x: 0, y: 0, z: 0 })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let instance = Instance;
        let input = "pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1";

        assert_eq!(instance.run(input).map(|d| d.part1), Ok("7".to_owned()));

        let input = "pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5";

        assert_eq!(
            instance.run(input).ok().and_then(|d| d.part2),
            Some("36".to_owned())
        );
    }
}
