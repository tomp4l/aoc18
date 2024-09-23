use std::{collections::HashMap, str::FromStr};

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, _input: &str) -> Result<DayResult, String> {
        let mut carts = Minecarts::from_str(_input)?;

        let part1 = run_to_crash(&mut carts);
        let part1 = format!("{},{}", part1.x, part1.y);

        let part2 = run_to_sole_survivor(carts);
        let part2 = format!("{},{}", part2.x, part2.y);
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
enum Track {
    Horizontal,
    Vertical,
    Intersection,
    CurveLeft,
    CurveRight,
}

#[derive(Debug)]
struct Cart {
    coord: Coord,
    direction: Direction,
    turn: Turn,
}

impl Cart {
    fn new(coord: Coord, direction: Direction) -> Self {
        Self {
            coord,
            direction,
            turn: Turn::Left,
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Turn {
    Left,
    Straight,
    Right,
}

struct Minecarts {
    tracks: HashMap<Coord, Track>,
    carts: Vec<Cart>,
}

impl FromStr for Minecarts {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tracks = HashMap::new();
        let mut carts = Vec::new();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let coord = Coord {
                    x: x as i32,
                    y: y as i32,
                };
                match c {
                    '-' => {
                        tracks.insert(coord, Track::Horizontal);
                    }
                    '|' => {
                        tracks.insert(coord, Track::Vertical);
                    }
                    '+' => {
                        tracks.insert(coord, Track::Intersection);
                    }
                    '/' => {
                        tracks.insert(coord, Track::CurveRight);
                    }
                    '\\' => {
                        tracks.insert(coord, Track::CurveLeft);
                    }
                    '^' => {
                        tracks.insert(coord, Track::Vertical);
                        carts.push(Cart::new(coord, Direction::Up));
                    }
                    'v' => {
                        tracks.insert(coord, Track::Vertical);
                        carts.push(Cart::new(coord, Direction::Down));
                    }
                    '<' => {
                        tracks.insert(coord, Track::Horizontal);
                        carts.push(Cart::new(coord, Direction::Left));
                    }
                    '>' => {
                        tracks.insert(coord, Track::Horizontal);
                        carts.push(Cart::new(coord, Direction::Right));
                    }
                    _ => {}
                }
            }
        }

        Ok(Minecarts { tracks, carts })
    }
}

impl Cart {
    fn move_tick(&mut self) {
        match self.direction {
            Direction::Up => {
                self.coord.y -= 1;
            }
            Direction::Down => {
                self.coord.y += 1;
            }
            Direction::Left => {
                self.coord.x -= 1;
            }
            Direction::Right => {
                self.coord.x += 1;
            }
        }
    }

    fn turn(&mut self, track: &Track) {
        match track {
            Track::CurveLeft => {
                self.direction = match self.direction {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };
            }
            Track::CurveRight => {
                self.direction = match self.direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                };
            }
            Track::Intersection => match self.turn {
                Turn::Left => {
                    self.direction = match self.direction {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                    };
                    self.turn = Turn::Straight;
                }
                Turn::Straight => {
                    self.turn = Turn::Right;
                }
                Turn::Right => {
                    self.direction = match self.direction {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    };
                    self.turn = Turn::Left;
                }
            },
            Track::Horizontal | Track::Vertical => {
                // Do nothing
            }
        }
    }
}

impl Minecarts {
    fn tick(&mut self) -> Option<Coord> {
        let mut first_crash = None;

        self.carts.sort_by_key(|cart| (cart.coord.y, cart.coord.x));

        let mut c = 0;
        while c < self.carts.len() {
            let cart = &mut self.carts[c];
            cart.move_tick();

            if let Some(track) = self.tracks.get(&cart.coord) {
                cart.turn(track);
            }

            let coord = cart.coord;
            let c_orig = c;
            c += 1;
            for c2 in 0..self.carts.len() {
                if c2 != c_orig && self.carts[c2].coord == coord {
                    if first_crash.is_none() {
                        first_crash = Some(coord);
                    }
                    self.carts.retain(|c| c.coord != coord);

                    if c2 < c_orig {
                        c -= 2;
                    } else {
                        c -= 1;
                    }

                    break;
                }
            }
        }

        first_crash
    }
}

fn run_to_crash(minecarts: &mut Minecarts) -> Coord {
    loop {
        if let Some(coord) = minecarts.tick() {
            return coord;
        }
    }
}

fn run_to_sole_survivor(mut minecarts: Minecarts) -> Coord {
    while minecarts.carts.len() > 1 {
        minecarts.tick();
    }

    minecarts.carts[0].coord
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let instance = Instance;
        let input = "/>-<\\  
|   |  
| /<+-\\
| | | v
\\>+</ |
  |   ^
  \\<->/";
        assert_eq!(
            instance.run(input).ok().and_then(|t| t.part2),
            Some("6,4".to_owned())
        );
    }
}
