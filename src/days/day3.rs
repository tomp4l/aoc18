use std::{collections::HashMap, str::FromStr};

use super::day::*;

pub struct Instance;

type Id = u16;

impl Day for Instance {
    fn run(&self, input: &str) -> Result<DayResult, String> {
        let claims = input
            .lines()
            .map(|line| line.parse::<Claim>())
            .collect::<Result<Vec<_>, _>>()?;

        let mut fabric = Fabric {
            fabric: HashMap::new(),
        };

        for claim in &claims {
            fabric.add_claim(claim);
        }

        let part1 = fabric.count_overlaps().to_string();
        let part2 = fabric
            .find_non_overlapping_claim(&claims)
            .map(|id| id.to_string());

        Ok(DayResult { part1, part2 })
    }
}

struct Claim {
    id: Id,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl FromStr for Claim {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() != 4 {
            return Err(format!("Invalid claim: {}", s));
        }
        let id = parts[0]
            .trim_start_matches('#')
            .parse::<Id>()
            .map_err(|e| format!("Invalid id {}: {}", parts[0], e))?;
        let coords: Vec<i32> = parts[2]
            .trim_end_matches(':')
            .split(',')
            .map(|s| {
                s.parse::<i32>()
                    .map_err(|e| format!("Invalid coordinate {}: {}", s, e))
            })
            .collect::<Result<Vec<i32>, String>>()?;
        let size: Vec<i32> = parts[3]
            .split('x')
            .map(|s| s.parse::<i32>().map_err(|e| e.to_string()))
            .collect::<Result<Vec<i32>, String>>()?;
        if coords.len() != 2 || size.len() != 2 {
            return Err(format!("Invalid claim: {}", s));
        }
        Ok(Claim {
            id,
            x: coords[0],
            y: coords[1],
            width: size[0],
            height: size[1],
        })
    }
}

struct Fabric {
    fabric: HashMap<(i32, i32), Vec<Id>>,
}

impl Fabric {
    fn add_claim(&mut self, claim: &Claim) {
        for x in claim.x..claim.x + claim.width {
            for y in claim.y..claim.y + claim.height {
                self.fabric
                    .entry((x, y))
                    .or_insert_with(Vec::new)
                    .push(claim.id);
            }
        }
    }

    fn count_overlaps(&self) -> usize {
        self.fabric.values().filter(|v| v.len() > 1).count()
    }

    fn find_non_overlapping_claim(&self, claims: &[Claim]) -> Option<Id> {
        for claim in claims {
            let overlaps = self
                .fabric
                .iter()
                .filter(|(_, v)| v.len() > 1)
                .any(|(_, v)| v.contains(&claim.id));
            if !overlaps {
                return Some(claim.id);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let instance = Instance;
        let input = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";
        let expected = DayResult {
            part1: "4".to_owned(),
            part2: Some("3".to_owned()),
        };
        assert_eq!(instance.run(input), Ok(expected));
    }
}
