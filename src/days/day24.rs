use std::{cmp::Reverse, collections::HashSet, str::FromStr};

use itertools::Itertools;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: &str) -> Result<DayResult, String> {
        let mut armies = input.parse::<Armies>()?;
        let part1 = armies.to_death().to_string();
        Ok(DayResult {
            part1,
            part2: Some(part2(&input.parse::<Armies>()?).to_string()),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Group {
    unit_count: usize,
    hit_points: usize,
    damage: usize,
    damage_type: String,
    initiative: usize,
    weaknesses: Vec<String>,
    immunities: Vec<String>,
}

impl FromStr for Group {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<&str>>();

        if parts.len() < 18 {
            return Err(format!("invalid group: {}", s));
        }

        let unit_count = parts[0]
            .parse::<usize>()
            .map_err(|e| format!("invalid unit count: {}", e))?;
        let hit_points = parts[4]
            .parse::<usize>()
            .map_err(|e| format!("invalid hit points: {}", e))?;

        let mut offset = 0;

        let mut weaknesses = Vec::new();
        let mut immunities = Vec::new();

        if parts[7].starts_with("(") {
            while !parts[7 + offset].ends_with(")") {
                offset += 1;
            }
            offset += 1;

            let immunities_weaknesses = parts[7..7 + offset]
                .join(" ")
                .trim_matches(|c| c == '(' || c == ')')
                .split("; ")
                .map(|s| s.to_owned())
                .collect::<Vec<_>>();

            for iw in immunities_weaknesses {
                let parts = iw.split_whitespace().collect::<Vec<&str>>();
                let kind = parts[0];
                let types = parts[2..]
                    .iter()
                    .map(|s| s.trim_end_matches(",").to_owned())
                    .collect::<Vec<String>>();

                match kind {
                    "weak" => weaknesses = types,
                    "immune" => immunities = types,
                    _ => return Err(format!("invalid kind: {}", kind)),
                }
            }
        }

        let damage = parts[12 + offset]
            .parse::<usize>()
            .map_err(|e| format!("invalid damage: {}", e))?;
        let damage_type = parts[13 + offset].to_owned();
        let initiative = parts[17 + offset]
            .parse::<usize>()
            .map_err(|e| format!("invalid initiative: {}", e))?;

        Ok(Group {
            unit_count,
            hit_points,
            damage,
            damage_type,
            initiative,
            weaknesses,
            immunities,
        })
    }
}

impl Group {
    fn effective_power(&self) -> usize {
        self.unit_count * self.damage
    }

    fn damage_to(&self, other: &Group) -> usize {
        if other.immunities.contains(&self.damage_type) {
            0
        } else if other.weaknesses.contains(&self.damage_type) {
            self.effective_power() * 2
        } else {
            self.effective_power()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Army {
    groups: Vec<Group>,
}

impl Army {
    fn select_targets(&self, other: &Army) -> Vec<(usize, usize)> {
        let mut targets = Vec::new();

        let sorted = self
            .groups
            .iter()
            .enumerate()
            .sorted_by_key(|(_, g)| (-(g.effective_power() as isize), -(g.initiative as isize)))
            .collect::<Vec<_>>();
        let mut selected = HashSet::new();

        for (i, group) in sorted {
            let target = other
                .groups
                .iter()
                .enumerate()
                .filter(|(j, g)| {
                    !g.immunities.contains(&group.damage_type) && !selected.contains(j)
                })
                .sorted_by_key(|(_, g)| (group.damage_to(g), g.effective_power(), g.initiative))
                .last();

            if let Some((j, _)) = target {
                targets.push((i, j));
                selected.insert(j);
            }
        }

        targets
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Armies {
    immune_system: Army,
    infection: Army,
}

impl Armies {
    fn attack(&mut self) {
        let mut immune_system_targets = self.immune_system.select_targets(&self.infection);
        let mut infection_targets = self.infection.select_targets(&self.immune_system);

        immune_system_targets
            .sort_by_key(|(i, _)| Reverse(self.immune_system.groups[*i].initiative));
        infection_targets.sort_by_key(|(i, _)| Reverse(self.infection.groups[*i].initiative));

        let mut immune_index = 0;
        let mut infection_index = 0;

        while immune_index < immune_system_targets.len()
            || infection_index < infection_targets.len()
        {
            let is_infection = if immune_index == immune_system_targets.len() {
                true
            } else if infection_index == infection_targets.len() {
                false
            } else {
                self.immune_system.groups[immune_system_targets[immune_index].0].initiative
                    < self.infection.groups[infection_targets[infection_index].0].initiative
            };

            let (attacker, target) = if is_infection {
                let r = (
                    &mut self.infection.groups[infection_targets[infection_index].0],
                    &mut self.immune_system.groups[infection_targets[infection_index].1],
                );
                infection_index += 1;
                r
            } else {
                let r = (
                    &mut self.immune_system.groups[immune_system_targets[immune_index].0],
                    &mut self.infection.groups[immune_system_targets[immune_index].1],
                );
                immune_index += 1;
                r
            };

            let damage = attacker.damage_to(target);
            let units_killed = damage / target.hit_points;
            target.unit_count = target.unit_count.saturating_sub(units_killed);
        }

        self.immune_system.groups.retain(|g| g.unit_count > 0);
        self.infection.groups.retain(|g| g.unit_count > 0);
    }

    fn to_death(&mut self) -> usize {
        let mut prev = self.clone();
        while !self.immune_system.groups.is_empty() && !self.infection.groups.is_empty() {
            self.attack();
            if self == &prev {
                break;
            }
            prev = self.clone();
        }
        if self.immune_system.groups.is_empty() {
            self.infection.groups.iter().map(|g| g.unit_count).sum()
        } else {
            self.immune_system.groups.iter().map(|g| g.unit_count).sum()
        }
    }

    fn boost(&mut self, boost: usize) {
        self.immune_system
            .groups
            .iter_mut()
            .for_each(|g| g.damage += boost);
    }
}

impl FromStr for Armies {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut armies = s.split("\n\n");

        let immune_system = armies
            .next()
            .ok_or("missing immune system")?
            .lines()
            .skip(1)
            .map(|line| line.parse::<Group>())
            .collect::<Result<Vec<_>, _>>()?;

        let infection = armies
            .next()
            .ok_or("missing infection")?
            .lines()
            .skip(1)
            .map(|line| line.parse::<Group>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Armies {
            immune_system: Army {
                groups: immune_system,
            },
            infection: Army { groups: infection },
        })
    }
}

fn binary_search<F>(mut low: usize, mut high: usize, mut f: F) -> usize
where
    F: FnMut(usize) -> bool,
{
    while low < high {
        let mid = low + (high - low) / 2;
        if f(mid) {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    low
}

fn part2(armies: &Armies) -> usize {
    let boost = binary_search(0, 10_000, |boost| {
        let mut armies = armies.clone();
        armies.boost(boost);
        armies.to_death();
        armies.infection.groups.is_empty()
    });
    let mut armies = armies.clone();
    armies.boost(boost);
    armies.to_death()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";

    #[test]
    fn parse() {
        assert_eq!(
            "76 units each with 3032 hit points with an attack that does 334 radiation damage at initiative 7"
                .parse::<Group>(),
            Ok(Group {
                unit_count: 76,
                hit_points: 3032,
                damage: 334,
                damage_type: "radiation".to_owned(),
                initiative: 7,
                weaknesses: Vec::new(),
                immunities: Vec::new(),
            })
        );

        assert_eq!(
            "18 units each with 729 hit points (weak to fire; immune to cold, slashing) with an attack that does 8 fire damage at initiative 10"
                .parse::<Group>(),
            Ok(Group {
                unit_count: 18,
                hit_points: 729,
                damage: 8,
                damage_type: "fire".to_owned(),
                initiative: 10,
                weaknesses: vec!["fire".to_owned()],
                immunities: vec!["cold".to_owned(), "slashing".to_owned()],
            })
        );

        assert_eq!(
            EXAMPLE.parse::<Armies>(),
            Ok(Armies {
                immune_system: Army {
                    groups: vec![
                        Group {
                            unit_count: 17,
                            hit_points: 5390,
                            damage: 4507,
                            damage_type: "fire".to_owned(),
                            initiative: 2,
                            weaknesses: vec!["radiation".to_owned(), "bludgeoning".to_owned()],
                            immunities: Vec::new(),
                        },
                        Group {
                            unit_count: 989,
                            hit_points: 1274,
                            damage: 25,
                            damage_type: "slashing".to_owned(),
                            initiative: 3,
                            weaknesses: vec!["bludgeoning".to_owned(), "slashing".to_owned()],
                            immunities: vec!["fire".to_owned()],
                        },
                    ],
                },
                infection: Army {
                    groups: vec![
                        Group {
                            unit_count: 801,
                            hit_points: 4706,
                            damage: 116,
                            damage_type: "bludgeoning".to_owned(),
                            initiative: 1,
                            weaknesses: vec!["radiation".to_owned()],
                            immunities: Vec::new(),
                        },
                        Group {
                            unit_count: 4485,
                            hit_points: 2961,
                            damage: 12,
                            damage_type: "slashing".to_owned(),
                            initiative: 4,
                            weaknesses: vec!["fire".to_owned(), "cold".to_owned()],
                            immunities: vec!["radiation".to_owned()],
                        },
                    ],
                },
            })
        );
    }

    #[test]
    fn attack() {
        let mut armies = EXAMPLE.parse::<Armies>().unwrap();
        armies.attack();

        assert_eq!(armies.immune_system.groups[0].unit_count, 905);
        assert_eq!(armies.infection.groups[0].unit_count, 797);
        assert_eq!(armies.infection.groups[1].unit_count, 4434);

        armies.attack();

        assert_eq!(armies.immune_system.groups[0].unit_count, 761);
        assert_eq!(armies.infection.groups[0].unit_count, 793);
        assert_eq!(armies.infection.groups[1].unit_count, 4434);
    }

    #[test]
    fn example() {
        let instance = Instance;

        let expected = DayResult {
            part1: "5216".to_owned(),
            part2: Some("51".to_owned()),
        };
        assert_eq!(instance.run(EXAMPLE), Ok(expected));
    }
}
