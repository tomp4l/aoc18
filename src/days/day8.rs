use std::str::FromStr;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: &str) -> Result<DayResult, String> {
        let tree = input.parse::<Tree>()?;
        let part1 = tree.metadata_sum().to_string();
        let part2 = Some(tree.root_value().to_string());

        Ok(DayResult { part1, part2 })
    }
}

struct Tree {
    children: Vec<Tree>,
    metadata: Vec<u32>,
}

impl FromStr for Tree {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split_whitespace()
            .map(|part| part.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        let tree = Tree::parse(&mut parts.into_iter())?;
        Ok(tree)
    }
}

impl Tree {
    fn parse(parts: &mut impl Iterator<Item = u32>) -> Result<Self, String> {
        let child_count = parts.next().ok_or("missing child count")?;
        let metadata_count = parts.next().ok_or("missing metadata count")?;

        let children = (0..child_count)
            .map(|_| Tree::parse(parts))
            .collect::<Result<Vec<_>, _>>()?;
        let metadata = parts.take(metadata_count as usize).collect::<Vec<_>>();

        Ok(Tree { children, metadata })
    }

    fn metadata_sum(&self) -> u32 {
        self.metadata.iter().sum::<u32>()
            + self
                .children
                .iter()
                .map(|child| child.metadata_sum())
                .sum::<u32>()
    }

    fn root_value(&self) -> u32 {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata
                .iter()
                .map(|&index| {
                    self.children
                        .get(index as usize - 1)
                        .map_or(0, |child| child.root_value())
                })
                .sum()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let instance = Instance;
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let expected = DayResult {
            part1: "138".to_owned(),
            part2: Some("66".to_owned()),
        };
        assert_eq!(instance.run(input), Ok(expected));
    }
}
