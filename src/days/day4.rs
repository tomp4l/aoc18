use std::{collections::HashMap, str::FromStr};

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: &str) -> Result<DayResult, String> {
        let logs = parse_input(input)?;
        Ok(DayResult {
            part1: part1(&logs),
            part2: Some(part2(&logs)),
        })
    }
}

type Id = u16;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Timestamp {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

fn days_in_month(month: u8) -> u8 {
    if month == 2 {
        28
    } else if month == 4 || month == 6 || month == 9 || month == 11 {
        30
    } else {
        31
    }
}

impl Timestamp {
    fn minutes_till(&self, other: &Timestamp) -> Vec<u8> {
        let mut minutes = Vec::new();
        let mut current = self.clone();
        while current < *other {
            minutes.push(current.minute);
            current.minute += 1;
            if current.minute == 60 {
                current.minute = 0;
                current.hour += 1;
                if current.hour == 24 {
                    current.hour = 0;
                    current.day += 1;
                    if current.day > days_in_month(current.month) {
                        current.day = 1;
                        current.month += 1;
                        if current.month == 13 {
                            current.month = 1;
                            current.year += 1;
                        }
                    }
                }
            }
        }
        minutes
    }
}

#[derive(Debug, PartialEq)]
enum Message {
    BeginShift(Id),
    FallAsleep,
    WakeUp,
}

#[derive(Debug, PartialEq)]
struct LogEntry {
    timestamp: Timestamp,
    message: Message,
}

impl FromStr for Message {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        match parts[0] {
            "Guard" => {
                let id = parts[1]
                    .trim_start_matches('#')
                    .parse::<Id>()
                    .map_err(|e| format!("Invalid id {}: {}", parts[1], e))?;
                Ok(Message::BeginShift(id))
            }
            "falls" => Ok(Message::FallAsleep),
            "wakes" => Ok(Message::WakeUp),
            _ => Err(format!("Invalid message: {}", s)),
        }
    }
}

impl FromStr for Timestamp {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(|c| c == '-' || c == ':' || c == ' ').collect();
        if parts.len() != 5 {
            return Err(format!("Invalid timestamp: {}", s));
        }
        let year = parts[0]
            .parse::<u16>()
            .map_err(|e| format!("Invalid year {}: {}", parts[0], e))?;
        let month = parts[1]
            .parse::<u8>()
            .map_err(|e| format!("Invalid month {}: {}", parts[1], e))?;
        let day = parts[2]
            .parse::<u8>()
            .map_err(|e| format!("Invalid day {}: {}", parts[2], e))?;
        let hour = parts[3]
            .parse::<u8>()
            .map_err(|e| format!("Invalid hour {}: {}", parts[3], e))?;
        let minute = parts[4]
            .parse::<u8>()
            .map_err(|e| format!("Invalid minute {}: {}", parts[4], e))?;
        Ok(Timestamp {
            year,
            month,
            day,
            hour,
            minute,
        })
    }
}

impl FromStr for LogEntry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(']').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid log entry: {}", s));
        }
        let timestamp = parts[0]
            .trim_start_matches('[')
            .parse::<Timestamp>()
            .map_err(|e| format!("Invalid timestamp {}: {}", parts[0], e))?;
        let message = parts[1].trim().parse::<Message>()?;
        Ok(LogEntry { timestamp, message })
    }
}

struct Logs {
    logs: Vec<LogEntry>,
}

impl Logs {
    fn new(mut logs: Vec<LogEntry>) -> Self {
        logs.sort_by(|x, y| x.timestamp.cmp(&y.timestamp));
        Logs { logs }
    }

    fn guard_sleep_times(&self) -> HashMap<Id, HashMap<u8, usize>> {
        let mut sleep_times = HashMap::new();

        let mut current_guard = None;
        let mut sleep_start = None;
        for log in &self.logs {
            match log.message {
                Message::BeginShift(id) => {
                    current_guard = Some(id);
                }
                Message::FallAsleep => {
                    sleep_start = Some(&log.timestamp);
                }
                Message::WakeUp => {
                    let guard = current_guard.unwrap();
                    let sleep_start = sleep_start.unwrap();
                    let guard_sleep_times = sleep_times.entry(guard).or_insert_with(HashMap::new);
                    for minute in sleep_start.minutes_till(&log.timestamp) {
                        *guard_sleep_times.entry(minute).or_insert(0) += 1;
                    }
                }
            }
        }

        sleep_times
    }
}

fn parse_input(input: &str) -> Result<Logs, String> {
    input
        .lines()
        .map(|line| line.parse::<LogEntry>())
        .collect::<Result<Vec<LogEntry>, String>>()
        .map(Logs::new)
}

fn part1(logs: &Logs) -> String {
    let sleep_times = logs.guard_sleep_times();
    let (guard, sleep_times) = sleep_times
        .iter()
        .max_by_key(|(_, times)| times.values().sum::<usize>())
        .unwrap();
    let (minute, _) = sleep_times.iter().max_by_key(|(_, times)| *times).unwrap();
    (guard * (*minute as u16)).to_string()
}

fn part2(logs: &Logs) -> String {
    let sleep_times = logs.guard_sleep_times();

    let (guard, minute, _) = sleep_times
        .iter()
        .map(|(guard, times)| {
            times
                .iter()
                .map(move |(minute, times)| (guard, minute, times))
        })
        .flatten()
        .max_by_key(|(_, _, times)| *times)
        .unwrap();

    (guard * (*minute as u16)).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "[1518-11-01 00:00] Guard #10 begins shift";
        let expected = LogEntry {
            timestamp: Timestamp {
                year: 1518,
                month: 11,
                day: 1,
                hour: 0,
                minute: 0,
            },
            message: Message::BeginShift(10),
        };
        assert_eq!(input.parse::<LogEntry>(), Ok(expected));
    }

    #[test]
    fn test_minutes_between() {
        let start = Timestamp {
            year: 1518,
            month: 11,
            day: 1,
            hour: 0,
            minute: 5,
        };
        let end = Timestamp {
            year: 1518,
            month: 11,
            day: 1,
            hour: 0,
            minute: 10,
        };
        assert_eq!(start.minutes_till(&end), vec![5, 6, 7, 8, 9]);

        let feb = Timestamp {
            year: 1518,
            month: 2,
            day: 28,
            hour: 23,
            minute: 58,
        };
        let mar = Timestamp {
            year: 1518,
            month: 3,
            day: 1,
            hour: 0,
            minute: 2,
        };

        assert_eq!(feb.minutes_till(&mar), vec![58, 59, 0, 1]);
    }

    #[test]
    fn example() {
        let instance = Instance;
        let input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";
        let expected = DayResult {
            part1: "240".to_owned(),
            part2: Some("4455".to_owned()),
        };
        assert_eq!(instance.run(input), Ok(expected));
    }
}
