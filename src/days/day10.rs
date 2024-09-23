use std::str::FromStr;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: &str) -> Result<DayResult, String> {
        let mut points = input
            .lines()
            .map(|line| line.parse::<Point>())
            .collect::<Result<Vec<_>, _>>()?;

        let (message, seconds) = find_message(&mut points);

        Ok(DayResult {
            part1: message,
            part2: Some(seconds.to_string()),
        })
    }
}

struct Point {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split(|c| c == '<' || c == '>' || c == ',')
            .filter_map(|part| part.trim().parse::<i32>().ok())
            .collect::<Vec<_>>();
        if parts.len() != 4 {
            return Err(format!("invalid point: {}", s));
        }
        let position = (parts[0], parts[1]);
        let velocity = (parts[2], parts[3]);
        Ok(Point { position, velocity })
    }
}

impl Point {
    fn step(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
    }
}

fn next_step(points: &mut Vec<Point>) {
    for point in points.iter_mut() {
        point.step();
    }
}

fn find_message(points: &mut Vec<Point>) -> (String, usize) {
    let mut seconds = 0;
    loop {
        next_step(points);
        seconds += 1;

        // Check if the points are close enough to form a message
        let min_x = points.iter().map(|point| point.position.0).min().unwrap();
        let max_x = points.iter().map(|point| point.position.0).max().unwrap();
        let min_y = points.iter().map(|point| point.position.1).min().unwrap();
        let max_y = points.iter().map(|point| point.position.1).max().unwrap();
        if max_x - min_x < 100 && max_y - min_y < 10 {
            let row_size = (max_x - min_x + 1) as usize;
            let mut message = vec![vec![' '; row_size]; (max_y - min_y + 1) as usize];
            for point in points.iter() {
                let x = point.position.0 - min_x;
                let y = point.position.1 - min_y;
                message[y as usize][x as usize] = '█';
            }

            // Looks for a space between characters
            let mut empty_col = true;
            for x in 0..row_size {
                empty_col = true;
                for y in 0..message.len() {
                    if message[y][x] != ' ' {
                        empty_col = false;
                        break;
                    }
                }
                if empty_col {
                    break;
                }
            }
            if !empty_col {
                continue;
            }

            let message = message
                .iter()
                .map(|row| row.iter().collect::<String>())
                .collect::<Vec<_>>()
                .join("\n");
            return (format!("\n{}", message), seconds);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let instance = Instance;
        let input = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";
        println!("{}", instance.run(input).unwrap().part1);
        let expected = DayResult {
            part1: "
█   █  ███
█   █   █ 
█   █   █ 
█████   █ 
█   █   █ 
█   █   █ 
█   █   █ 
█   █  ███"
                .to_owned(),
            part2: Some("3".to_owned()),
        };
        assert_eq!(instance.run(input), Ok(expected));
    }
}
