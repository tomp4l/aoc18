use std::{collections::HashMap, iter, str::FromStr};

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
        let regex = input.parse::<RoomRegex>()?;
        let rooms = regex.to_rooms();
        if self.verbose {
            rooms.print();
        }

        let (part1, part2) = rooms.furthest_rooms();
        Ok(DayResult {
            part1: part1.to_string(),
            part2: Some(part2.to_string()),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum RoomRegex {
    Literal(Direction),
    Group(Vec<RoomRegex>),
    Or(Vec<Vec<RoomRegex>>),
}

impl FromStr for RoomRegex {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let mut stack = vec![];
        let mut current = RoomRegex::Group(vec![]);
        while let Some(c) = chars.next() {
            // println!("{:?} {:?}", c, current);
            match c {
                '^' => {}
                '$' => {}
                '(' => {
                    stack.push(current.clone());
                    current = RoomRegex::Group(vec![]);
                }
                ')' => {
                    let prev = current;
                    current = stack.pop().unwrap();
                    match current {
                        RoomRegex::Group(ref mut group) => group.push(prev),
                        RoomRegex::Or(ref mut group) => match prev {
                            RoomRegex::Group(g) => group.push(g),
                            RoomRegex::Or(_) => group.last_mut().unwrap().push(prev),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    }
                }
                '|' => match current {
                    RoomRegex::Group(g) => {
                        current = RoomRegex::Or(vec![g, vec![]]);
                    }
                    RoomRegex::Or(ref mut groups) => {
                        groups.push(vec![]);
                    }
                    _ => unreachable!(),
                },
                _ => {
                    let d = match c {
                        'N' => Direction::North,
                        'E' => Direction::East,
                        'S' => Direction::South,
                        'W' => Direction::West,
                        _ => return Err(format!("invalid direction: {}", c)),
                    };
                    match current {
                        RoomRegex::Group(ref mut group) => group.push(RoomRegex::Literal(d)),
                        RoomRegex::Or(ref mut group) => {
                            group.last_mut().unwrap().push(RoomRegex::Literal(d))
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
        Ok(current)
    }
}

struct Doors {
    north: bool,
    east: bool,
    south: bool,
    west: bool,
}

impl Default for Doors {
    fn default() -> Self {
        Doors {
            north: false,
            east: false,
            south: false,
            west: false,
        }
    }
}

struct Rooms {
    rooms: HashMap<(i32, i32), Doors>,
}

impl Rooms {
    fn print(&self) {
        let min_x = self.rooms.keys().map(|(x, _)| x).min().copied().unwrap();
        let max_x = self.rooms.keys().map(|(x, _)| x).max().copied().unwrap();
        let min_y = self.rooms.keys().map(|(_, y)| y).min().copied().unwrap();
        let max_y = self.rooms.keys().map(|(_, y)| y).max().copied().unwrap();
        let empty = Doors::default();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let doors = self.rooms.get(&(x, y)).unwrap_or(&empty);
                print!("#{}", if doors.north { "-" } else { "#" });
            }
            println!("#");
            for x in min_x..=max_x {
                let doors = self.rooms.get(&(x, y)).unwrap_or(&empty);
                print!(
                    "{}{}",
                    if doors.west { "|" } else { "#" },
                    if x == 0 && y == 0 { "X" } else { "." }
                );
            }
            println!("#");
        }

        println!(
            "{}",
            iter::repeat("#")
                .take((2 * (max_x - min_x) + 3) as usize)
                .collect::<String>()
        );
    }

    fn furthest_rooms(&self) -> (usize, usize) {
        let mut distances = HashMap::new();
        distances.insert((0, 0), 0);
        let mut stack = vec![(0, 0)];
        while let Some((x, y)) = stack.pop() {
            let distance = distances[&(x, y)];
            let doors = &self.rooms[&(x, y)];
            if doors.north {
                let y_ = y - 1;
                if !distances.contains_key(&(x, y_)) {
                    distances.insert((x, y_), distance + 1);
                    stack.push((x, y_));
                }
            }
            if doors.east {
                let x_ = x + 1;
                if !distances.contains_key(&(x_, y)) {
                    distances.insert((x_, y), distance + 1);
                    stack.push((x_, y));
                }
            }
            if doors.south {
                let y_ = y + 1;
                if !distances.contains_key(&(x, y_)) {
                    distances.insert((x, y_), distance + 1);
                    stack.push((x, y_));
                }
            }
            if doors.west {
                let x_ = x - 1;
                if !distances.contains_key(&(x_, y)) {
                    distances.insert((x_, y), distance + 1);
                    stack.push((x_, y));
                }
            }
        }
        (
            *distances.values().max().unwrap(),
            distances.values().filter(|&&d| d >= 1000).count(),
        )
    }
}

impl RoomRegex {
    fn to_rooms(&self) -> Rooms {
        let mut rooms = HashMap::new();

        fn walk(
            rooms: &mut HashMap<(i32, i32), Doors>,
            x: i32,
            y: i32,
            regex: &RoomRegex,
        ) -> (i32, i32) {
            match regex {
                RoomRegex::Literal(Direction::North) => {
                    rooms.entry((x, y)).or_default().north = true;
                    rooms.entry((x, y - 1)).or_default().south = true;
                    (x, y - 1)
                }
                RoomRegex::Literal(Direction::East) => {
                    rooms.entry((x, y)).or_default().east = true;
                    rooms.entry((x + 1, y)).or_default().west = true;
                    (x + 1, y)
                }
                RoomRegex::Literal(Direction::South) => {
                    rooms.entry((x, y)).or_default().south = true;
                    rooms.entry((x, y + 1)).or_default().north = true;
                    (x, y + 1)
                }
                RoomRegex::Literal(Direction::West) => {
                    rooms.entry((x, y)).or_default().west = true;
                    rooms.entry((x - 1, y)).or_default().east = true;
                    (x - 1, y)
                }
                RoomRegex::Group(group) => {
                    let mut x = x;
                    let mut y = y;
                    for r in group {
                        let (x_, y_) = walk(rooms, x, y, r);
                        x = x_;
                        y = y_;
                    }
                    (x, y)
                }
                RoomRegex::Or(group) => {
                    let mut c = (x, y);
                    for r in group {
                        c = walk(rooms, x, y, &RoomRegex::Group(r.clone()));
                    }
                    c
                }
            }
        }

        walk(&mut rooms, 0, 0, self);

        Rooms { rooms }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            "^(NE|SE|EE|WE)$".parse::<RoomRegex>(),
            Ok(RoomRegex::Group(vec![RoomRegex::Or(vec![
                vec![
                    RoomRegex::Literal(Direction::North),
                    RoomRegex::Literal(Direction::East)
                ],
                vec![
                    RoomRegex::Literal(Direction::South),
                    RoomRegex::Literal(Direction::East)
                ],
                vec![
                    RoomRegex::Literal(Direction::East),
                    RoomRegex::Literal(Direction::East)
                ],
                vec![
                    RoomRegex::Literal(Direction::West),
                    RoomRegex::Literal(Direction::East)
                ],
            ])]))
        );
        assert_eq!(
            "^(N|N(E|W))$".parse::<RoomRegex>(),
            Ok(RoomRegex::Group(vec![RoomRegex::Or(vec![
                vec![RoomRegex::Literal(Direction::North),],
                vec![
                    RoomRegex::Literal(Direction::North),
                    RoomRegex::Or(vec![
                        vec![RoomRegex::Literal(Direction::East),],
                        vec![RoomRegex::Literal(Direction::West),],
                    ]),
                ],
            ])]))
        );
    }

    #[test]
    fn example() {
        let instance = Instance { verbose: true };
        let examples = vec![
            ("^WNE$", 3),
            ("^ENWWW(NEEE|SSE(EE|N))$", 10),
            ("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$", 18),
            ("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$", 23),
            (
                "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$",
                31,
            ),
        ];

        for (input, expected) in examples {
            let expected = DayResult {
                part1: expected.to_string(),
                part2: Some("0".to_owned()),
            };
            assert_eq!(instance.run(input), Ok(expected));
        }
    }
}
