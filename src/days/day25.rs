use std::str::FromStr;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: &str) -> Result<DayResult, String> {
        let points = input
            .lines()
            .map(|line| line.parse::<Point>())
            .collect::<Result<Vec<_>, _>>()?;
        let part1 = make_constellations(&points).len().to_string();
        Ok(DayResult { part1, part2: None })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    w: i32,
    x: i32,
    y: i32,
    z: i32,
}

impl FromStr for Point {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',').collect::<Vec<&str>>();
        if parts.len() != 4 {
            return Err(format!("invalid point: {}", s));
        }

        let w = parts[0]
            .parse::<i32>()
            .map_err(|e| format!("invalid w: {}", e))?;
        let x = parts[1]
            .parse::<i32>()
            .map_err(|e| format!("invalid x: {}", e))?;
        let y = parts[2]
            .parse::<i32>()
            .map_err(|e| format!("invalid y: {}", e))?;
        let z = parts[3]
            .parse::<i32>()
            .map_err(|e| format!("invalid z: {}", e))?;
        Ok(Point { w, x, y, z })
    }
}

impl Point {
    fn manhattan_distance(&self, other: &Point) -> usize {
        ((self.w - other.w).abs()
            + (self.x - other.x).abs()
            + (self.y - other.y).abs()
            + (self.z - other.z).abs()) as usize
    }
}

#[derive(Debug, Clone)]
struct Constellation {
    points: Vec<Point>,
}

impl Constellation {
    fn new() -> Self {
        Constellation { points: Vec::new() }
    }

    fn add(&mut self, point: Point) {
        self.points.push(point);
    }

    fn merge(&mut self, other: &mut Constellation) {
        self.points.append(&mut other.points);
    }

    fn contains(&self, point: &Point) -> bool {
        self.points.iter().any(|p| p.manhattan_distance(point) <= 3)
    }
}

fn make_constellations(points: &[Point]) -> Vec<Constellation> {
    let mut constellations: Vec<Constellation> = Vec::new();
    for point in points {
        let mut new_constellations = Vec::new();
        let mut new_constellation = Constellation::new();
        new_constellation.add(*point);

        for constellation in constellations.iter_mut() {
            if constellation.contains(point) {
                new_constellation.merge(constellation);
            } else {
                new_constellations.push(constellation.clone());
            }
        }
        new_constellations.push(new_constellation);
        constellations = new_constellations;
    }
    constellations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let instance = Instance;
        let input = "0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0";
        let expected = DayResult {
            part1: "2".to_owned(),
            part2: None,
        };
        assert_eq!(instance.run(input), Ok(expected));
    }
}
