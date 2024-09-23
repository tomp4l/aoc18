use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: &str) -> Result<DayResult, String> {
        let (players, marbles) = parse(input);
        let part1 = play_game(players, marbles).to_string();
        let part2 = play_game(players, marbles * 100).to_string();

        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}

struct MarbleGame {
    scores: Vec<usize>,
    marbles: Vec<Marble>,
    current_player: usize,
    current_marble: usize,
}

#[derive(Debug, Clone, Copy)]
struct Marble {
    left: usize,
    right: usize,
}

impl MarbleGame {
    fn new(players: usize, max_marble: usize) -> Self {
        Self {
            scores: vec![0; players],
            current_player: 0,
            marbles: vec![Marble { left: 0, right: 0 }; max_marble + 1],
            current_marble: 0,
        }
    }

    fn insert(&mut self, marble: usize) {
        if marble % 23 == 0 {
            self.scores[self.current_player] += marble;
            let mut removed_marble = self.current_marble;
            for _ in 0..7 {
                removed_marble = self.marbles[removed_marble].left;
            }
            let left = self.marbles[removed_marble].left;
            let right = self.marbles[removed_marble].right;
            self.marbles[left].right = right;
            self.marbles[right].left = left;
            self.scores[self.current_player] += removed_marble;
            self.current_marble = right;
        } else {
            let left = self.marbles[self.current_marble].right;
            let right = self.marbles[left].right;

            self.marbles[marble].left = left;
            self.marbles[marble].right = right;
            self.marbles[left].right = marble;
            self.marbles[right].left = marble;
            self.current_marble = marble;
        }

        self.current_player = (self.current_player + 1) % self.scores.len();
    }
}

fn parse(input: &str) -> (usize, usize) {
    let parts: Vec<_> = input
        .split_whitespace()
        .filter_map(|part| part.parse::<usize>().ok())
        .collect();
    (parts[0], parts[1])
}

fn play_game(players: usize, marbles: usize) -> usize {
    let mut game = MarbleGame::new(players, marbles);

    for marble in 1..=marbles {
        game.insert(marble);
    }
    *game.scores.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let examples = vec![
            (9, 25, 32),
            (10, 1618, 8317),
            (13, 7999, 146373),
            (17, 1104, 2764),
            (21, 6111, 54718),
            (30, 5807, 37305),
        ];

        for example in examples {
            let (players, marbles, high_score) = example;
            assert_eq!(play_game(players, marbles), high_score);
        }
    }
}
