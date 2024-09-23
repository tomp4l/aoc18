use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: &str) -> Result<DayResult, String> {
        let part1 = part1(input).to_string();
        let part2 = part2(input).to_string();
        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Coord {
    y: i32,
    x: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum UnitType {
    Elf,
    Goblin,
}

#[derive(Debug, PartialEq, Eq)]
struct Unit {
    hp: i32,
    unit_type: UnitType,
}

#[derive(Debug, PartialEq, Eq)]
struct Cave {
    walls: HashSet<Coord>,
    units: HashMap<Coord, Unit>,
    elf_damage: i32,
}

impl FromStr for Cave {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut walls = HashSet::new();
        let mut units = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let coord = Coord {
                    x: x as i32,
                    y: y as i32,
                };
                match c {
                    '#' => {
                        walls.insert(coord);
                    }
                    'E' => {
                        units.insert(
                            coord,
                            Unit {
                                hp: 200,
                                unit_type: UnitType::Elf,
                            },
                        );
                    }
                    'G' => {
                        units.insert(
                            coord,
                            Unit {
                                hp: 200,
                                unit_type: UnitType::Goblin,
                            },
                        );
                    }
                    _ => {}
                }
            }
        }
        Ok(Cave {
            walls,
            units,
            elf_damage: 3,
        })
    }
}

impl Cave {
    fn round(&mut self) -> bool {
        let mut units: Vec<_> = self.units.keys().copied().collect();
        units.sort();
        let mut killed = HashSet::new();
        for coord in units {
            let mut current_coord = coord;
            if killed.contains(&coord) {
                continue;
            }
            let unit = &self.units[&coord];

            let targets = self.targets(unit.unit_type);
            if targets.is_empty() {
                return false;
            }

            let maybe_next = self.next_move(&current_coord, &targets);
            if let Some(next) = maybe_next {
                let unit = self.units.remove(&coord).unwrap();
                self.units.insert(next, unit);
                current_coord = next;
            }

            let maybe_target = self
                .in_range(&current_coord, &targets)
                .min_by_key(|c| (self.units[c].hp, *c));
            if let Some(target) = maybe_target {
                let unit = self.units.get_mut(target).unwrap();
                let damage = if unit.unit_type == UnitType::Elf {
                    3
                } else {
                    self.elf_damage
                };
                unit.hp -= damage;
                if unit.hp <= 0 {
                    self.units.remove(target);
                    killed.insert(*target);
                }
            }
        }
        true
    }

    fn targets(&self, unit_type: UnitType) -> Vec<Coord> {
        self.units
            .iter()
            .filter(|(_, u)| u.unit_type != unit_type)
            .map(|(c, _)| *c)
            .collect()
    }

    fn in_range<'a>(
        &self,
        current: &'a Coord,
        targets: &'a [Coord],
    ) -> impl Iterator<Item = &'a Coord> {
        targets
            .iter()
            .filter(|t| (t.x - current.x).abs() + (t.y - current.y).abs() == 1)
    }

    fn next_move(&self, start: &Coord, targets: &[Coord]) -> Option<Coord> {
        let mut paths = Vec::new();

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((vec![], *start));
        visited.insert(*start);
        while let Some((path, coord)) = queue.pop_front() {
            if self.in_range(&coord, targets).next().is_some() {
                paths.push(path);
                continue;
            }
            for (dx, dy) in &[(0, -1), (-1, 0), (1, 0), (0, 1)] {
                let next = Coord {
                    x: coord.x + dx,
                    y: coord.y + dy,
                };
                if !self.walls.contains(&next)
                    && !self.units.contains_key(&next)
                    && !visited.contains(&next)
                {
                    let mut path = path.clone();
                    path.push(next);
                    queue.push_back((path, next));
                    visited.insert(next);
                }
            }
        }

        if paths.is_empty() {
            return None;
        }
        let min_length = paths.iter().map(|p| p.len()).min().unwrap();

        let mut min_paths = paths
            .into_iter()
            .filter(|p| p.len() == min_length)
            .collect::<Vec<_>>();
        min_paths.sort_by_key(|p| (p.last().copied(), p.first().copied()));
        min_paths.first().map(|v| v.first()).flatten().copied()
    }

    fn to_string(&self) -> String {
        let mut max_x = 0;
        let mut max_y = 0;
        for coord in self.walls.iter().chain(self.units.keys()) {
            max_x = max_x.max(coord.x);
            max_y = max_y.max(coord.y);
        }
        let mut s = String::new();
        for y in 0..=max_y {
            for x in 0..=max_x {
                let coord = Coord { x, y };
                if self.walls.contains(&coord) {
                    s.push('#');
                } else if let Some(unit) = self.units.get(&coord) {
                    match unit.unit_type {
                        UnitType::Elf => s.push('E'),
                        UnitType::Goblin => s.push('G'),
                    }
                } else {
                    s.push('.');
                }
            }
            s.push('\n');
        }
        s
    }

    fn print(&self) {
        println!("{}", self.to_string());
    }
}

fn part1(input: &str) -> usize {
    run_to_end(input, 3, false).0
}

fn part2(input: &str) -> usize {
    let mut damage = 4;
    let mut kill_turns = 200 / 4;
    loop {
        let (rounds, is_elf_win) = run_to_end(input, damage, false);

        if is_elf_win {
            return rounds;
        }
        let mut new_kill_turns = kill_turns;
        while new_kill_turns == kill_turns {
            damage += 1;
            new_kill_turns = 200 / damage;
            if 200 % damage != 0 {
                new_kill_turns += 1;
            }
        }
        kill_turns = new_kill_turns;
    }
}

fn run_to_end(input: &str, damage: i32, verbose: bool) -> (usize, bool) {
    let mut cave: Cave = input.parse().unwrap();
    let elf_count = cave
        .units
        .values()
        .filter(|u| u.unit_type == UnitType::Elf)
        .count();
    cave.elf_damage = damage;
    let mut rounds = 0;
    if verbose {
        cave.print();
    }
    loop {
        let finished = cave.round();
        if finished {
            rounds += 1;
        }
        if verbose {
            cave.print();
        }

        if cave.units.iter().all(|(_, u)| u.unit_type == UnitType::Elf)
            || cave
                .units
                .iter()
                .all(|(_, u)| u.unit_type == UnitType::Goblin)
        {
            break;
        }
    }

    let new_elf_count = cave
        .units
        .values()
        .filter(|u| u.unit_type == UnitType::Elf)
        .count();
    (
        rounds * cave.units.values().map(|u| u.hp).sum::<i32>() as usize,
        new_elf_count == elf_count,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let examples = vec![
            (
                "#######   
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######",
                27730,
            ),
            (
                "#######   
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######
",
                39514,
            ),
            (
                "#######   
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######
",
                27755,
            ),
            (
                "#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######
",
                28944,
            ),
            (
                "#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########
",
                18740,
            ),
        ];
        for (input, expected) in examples {
            assert_eq!(part1(input), expected);
        }
    }

    #[test]
    fn shortest_path() {
        let cave: Cave = "#######
#.E...#
#.....#
#...G.#
#######"
            .parse()
            .unwrap();

        assert_eq!(
            cave.next_move(&Coord { x: 2, y: 2 }, &vec![Coord { x: 4, y: 3 }]),
            Some(Coord { x: 3, y: 2 })
        );
    }

    #[test]
    fn test_movement() {
        let mut cave: Cave = "#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########"
            .parse()
            .unwrap();

        cave.round();
        let expected = "#########
#.G...G.#
#...G...#
#...E..G#
#.G.....#
#.......#
#G..G..G#
#.......#
#########
";

        assert_eq!(cave.to_string().as_str(), expected);
        cave.round();
        let expected = "#########
#..G.G..#
#...G...#
#.G.E.G.#
#.......#
#G..G..G#
#.......#
#.......#
#########
";
        assert_eq!(cave.to_string().as_str(), expected);
        cave.round();
        let expected = "#########
#.......#
#..GGG..#
#..GEG..#
#G..G...#
#......G#
#.......#
#.......#
#########
";
        assert_eq!(cave.to_string().as_str(), expected);
    }

    #[test]
    fn test_sample() {
        let mut cave: Cave = "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"
            .parse()
            .unwrap();

        cave.round();
        let expected = "#######
#..G..#
#...EG#
#.#G#G#
#...#E#
#.....#
#######
";
        assert_eq!(cave.to_string().as_str(), expected);
        assert_eq!(cave.units[&Coord { x: 3, y: 1 }].hp, 200);
        assert_eq!(cave.units[&Coord { x: 4, y: 2 }].hp, 197);
        assert_eq!(cave.units[&Coord { x: 5, y: 2 }].hp, 197);
        assert_eq!(cave.units[&Coord { x: 3, y: 3 }].hp, 200);
        assert_eq!(cave.units[&Coord { x: 5, y: 3 }].hp, 197);
        assert_eq!(cave.units[&Coord { x: 5, y: 4 }].hp, 197);

        cave.round();

        let expected = "#######
#...G.#
#..GEG#
#.#.#G#
#...#E#
#.....#
#######
";
        assert_eq!(cave.to_string().as_str(), expected);
        assert_eq!(cave.units[&Coord { x: 4, y: 1 }].hp, 200);
        assert_eq!(cave.units[&Coord { x: 3, y: 2 }].hp, 200);
        assert_eq!(cave.units[&Coord { x: 4, y: 2 }].hp, 188);
        assert_eq!(cave.units[&Coord { x: 5, y: 2 }].hp, 194);
        assert_eq!(cave.units[&Coord { x: 5, y: 3 }].hp, 194);
        assert_eq!(cave.units[&Coord { x: 5, y: 4 }].hp, 194);

        for _ in 2..23 {
            cave.round();
        }

        let expected = "#######
#...G.#
#..G.G#
#.#.#G#
#...#E#
#.....#
#######
";
        assert_eq!(cave.to_string().as_str(), expected);
        assert_eq!(cave.units[&Coord { x: 4, y: 1 }].hp, 200);
        assert_eq!(cave.units[&Coord { x: 3, y: 2 }].hp, 200);
        assert_eq!(cave.units[&Coord { x: 5, y: 2 }].hp, 131);
        assert_eq!(cave.units[&Coord { x: 5, y: 3 }].hp, 131);
        assert_eq!(cave.units[&Coord { x: 5, y: 4 }].hp, 131);

        cave.round();
        let expected = "#######
#..G..#
#...G.#
#.#G#G#
#...#E#
#.....#
#######
";
        assert_eq!(cave.to_string().as_str(), expected);
        assert_eq!(cave.units[&Coord { x: 3, y: 1 }].hp, 200);
        assert_eq!(cave.units[&Coord { x: 4, y: 2 }].hp, 131);
        assert_eq!(cave.units[&Coord { x: 3, y: 3 }].hp, 200);
        assert_eq!(cave.units[&Coord { x: 5, y: 3 }].hp, 128);
        assert_eq!(cave.units[&Coord { x: 5, y: 4 }].hp, 128);

        cave.round();
        let expected = "#######
#.G...#
#..G..#
#.#.#G#
#..G#E#
#.....#
#######
";
        assert_eq!(cave.to_string().as_str(), expected);
        assert_eq!(cave.units[&Coord { x: 2, y: 1 }].hp, 200);
        assert_eq!(cave.units[&Coord { x: 3, y: 2 }].hp, 131);
        assert_eq!(cave.units[&Coord { x: 5, y: 3 }].hp, 125);
        assert_eq!(cave.units[&Coord { x: 3, y: 4 }].hp, 200);
        assert_eq!(cave.units[&Coord { x: 5, y: 4 }].hp, 125);

        cave.round();
        let expected = "#######
#G....#
#.G...#
#.#.#G#
#...#E#
#..G..#
#######
";
        assert_eq!(cave.to_string().as_str(), expected);
        assert_eq!(cave.units[&Coord { x: 1, y: 1 }].hp, 200);
        assert_eq!(cave.units[&Coord { x: 2, y: 2 }].hp, 131);
        assert_eq!(cave.units[&Coord { x: 5, y: 3 }].hp, 122);
        assert_eq!(cave.units[&Coord { x: 5, y: 4 }].hp, 122);
        assert_eq!(cave.units[&Coord { x: 3, y: 5 }].hp, 200);

        cave.round();
        let expected = "#######
#G....#
#.G...#
#.#.#G#
#...#E#
#...G.#
#######
";
        assert_eq!(cave.to_string().as_str(), expected);
        assert_eq!(cave.units[&Coord { x: 1, y: 1 }].hp, 200);
        assert_eq!(cave.units[&Coord { x: 2, y: 2 }].hp, 131);
        assert_eq!(cave.units[&Coord { x: 5, y: 3 }].hp, 119);
        assert_eq!(cave.units[&Coord { x: 5, y: 4 }].hp, 119);
        assert_eq!(cave.units[&Coord { x: 4, y: 5 }].hp, 200);

        cave.round();
        let expected = "#######
#G....#
#.G...#
#.#.#G#
#...#E#
#....G#
#######
";
        assert_eq!(cave.to_string().as_str(), expected);
        assert_eq!(cave.units[&Coord { x: 1, y: 1 }].hp, 200);
        assert_eq!(cave.units[&Coord { x: 2, y: 2 }].hp, 131);
        assert_eq!(cave.units[&Coord { x: 5, y: 3 }].hp, 116);
        assert_eq!(cave.units[&Coord { x: 5, y: 4 }].hp, 113);
        assert_eq!(cave.units[&Coord { x: 5, y: 5 }].hp, 200);

        for _ in 29..=47 {
            cave.round();
        }

        let expected = "#######
#G....#
#.G...#
#.#.#G#
#...#.#
#....G#
#######
";
        assert_eq!(cave.to_string().as_str(), expected);
        assert_eq!(cave.units[&Coord { x: 1, y: 1 }].hp, 200);
        assert_eq!(cave.units[&Coord { x: 2, y: 2 }].hp, 131);
        assert_eq!(cave.units[&Coord { x: 5, y: 3 }].hp, 59);
        assert_eq!(cave.units[&Coord { x: 5, y: 5 }].hp, 200);
    }

    #[test]
    fn test_long_movement() {
        let mut cave: Cave = "###########
#G..#....G#
###..E#####
###########
"
        .parse()
        .unwrap();

        cave.round();

        let expected = "###########
#.G.#...G.#
###.E.#####
###########
";
        assert_eq!(cave.to_string().as_str(), expected);
    }
}
