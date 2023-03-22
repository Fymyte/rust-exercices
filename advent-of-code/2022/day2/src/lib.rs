use std::{cmp::Ordering, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Outcome {
    Lost,
    Draw,
    Win,
}

impl PartialOrd for HandShape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandShape {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (HandShape::Rock, HandShape::Paper) => Ordering::Less,
            (HandShape::Rock, HandShape::Scissors) => Ordering::Greater,
            (HandShape::Paper, HandShape::Rock) => Ordering::Greater,
            (HandShape::Paper, HandShape::Scissors) => Ordering::Less,
            (HandShape::Scissors, HandShape::Rock) => Ordering::Less,
            (HandShape::Scissors, HandShape::Paper) => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

impl From<char> for HandShape {
    fn from(value: char) -> Self {
        match value {
            'A' | 'X' => HandShape::Rock,
            'B' | 'Y' => HandShape::Paper,
            'C' | 'Z' => HandShape::Scissors,
            _ => panic!("Invalid character for HandShape {value}"),
        }
    }
}

impl From<&str> for HandShape {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => HandShape::Rock,
            "B" | "Y" => HandShape::Paper,
            "C" | "Z" => HandShape::Scissors,
            _ => panic!("Invalid character for HandShape {value}"),
        }
    }
}

impl From<&str> for Outcome {
    fn from(value: &str) -> Self {
        match value {
            "X" => Outcome::Lost,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Invalid character for Outcome {value}")
        }
    }
}

impl From<&HandShape> for u32 {
    fn from(val: &HandShape) -> Self {
        match val {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            HandShape::Scissors => 3,
        }
    }
}

impl From<&Outcome> for u32 {
    fn from(val: &Outcome) -> Self {
        match val {
            Outcome::Lost => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

impl From<Ordering> for Outcome {
    fn from(value: Ordering) -> Self {
        match value {
            Ordering::Less => Outcome::Lost,
            Ordering::Equal => Outcome::Draw,
            Ordering::Greater => Outcome::Win,
        }
    }
}

#[derive(Debug)]
struct Round {
    elf: HandShape,
    me: HandShape,
}

#[derive(Debug)]
struct ExpectedRound {
    elf: HandShape,
    expected: Outcome,
}

impl FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.split(' ');
        Ok(Self {
            elf: chars.next().ok_or("Unable to parse elf HandShape")?.into(),
            me: chars.next().ok_or("Unable to parse mine HandShape")?.into(),
        })
    }
}

impl FromStr for ExpectedRound {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.split(' ');
        Ok(Self {
            elf: chars.next().ok_or("Unable to parse elf HandShape")?.into(),
            expected: chars.next().ok_or("Unable to parse mine HandShape")?.into(),
        })
    }
}

impl Round {
    fn play(&self) -> u32 {
        let outcome: Outcome = self.me.cmp(&self.elf).into();
        u32::from(&outcome) + u32::from(&self.me)
    }
}

impl ExpectedRound {
    fn play(&self) -> u32 {
        let me = match (&self.expected, &self.elf) {
            (Outcome::Lost, HandShape::Rock) => &HandShape::Scissors,
            (Outcome::Lost, HandShape::Paper) => &HandShape::Rock,
            (Outcome::Lost, HandShape::Scissors) => &HandShape::Paper,
            (Outcome::Win, HandShape::Rock) => &HandShape::Paper,
            (Outcome::Win, HandShape::Paper) => &HandShape::Scissors,
            (Outcome::Win, HandShape::Scissors) => &HandShape::Rock,
            (Outcome::Draw, elf) => elf,
        };
        u32::from(&self.expected) + u32::from(me)
    }
}

pub fn process_part1(input: &str) -> String {
    let score: u32 = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let round: Round = l.parse().unwrap();
            round.play()
        })
        .sum();
    score.to_string()
}

pub fn process_part2(input: &str) -> String {
    let score: u32 = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let round: ExpectedRound = l.parse().unwrap();
            round.play()
        })
        .sum();
    score.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "15");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "12");
    }
}
