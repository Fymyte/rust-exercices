#![feature(iter_array_chunks)]
fn priority(char: char) -> u32 {
    match char {
        'a'..='z' => (char as u32) - ('a' as u32) + 1,
        'A'..='Z' => (char as u32) - ('A' as u32) + 27,
        _ => 0,
    }
}

pub fn process_part1(input: &str) -> String {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| (&l[..(l.len() / 2)], &l[(l.len() / 2)..]))
        .map(|(comp1, comp2)| {
            for c in comp1.chars() {
                if comp2.contains(c) {
                    return priority(c);
                }
            }

            panic!("no char in both compartment")
        })
        .sum::<u32>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    input
        .lines()
        .array_chunks::<3>()
        .map(|[e1, e2, e3]| {
            for c in e1.chars() {
                if e2.contains(c) && e3.contains(c) {
                    return priority(c);
                }
            }

            panic!("no char in both compartment")
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "157");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "70");
    }
}
