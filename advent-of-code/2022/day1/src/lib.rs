pub fn process_part1(input: &str) -> String {
    let mut loads: Vec<u32> = input
        .split("\n\n")
        .map(|items| items.lines().map(|load| load.parse::<u32>().unwrap()).sum())
        .collect();
    loads.sort();
    loads.last().unwrap().to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut loads: Vec<u32> = input
        .split("\n\n")
        .map(|items| items.lines().map(|load| load.parse::<u32>().unwrap()).sum())
        .collect();
    loads.sort_by(|a, b| b.cmp(a));
    loads.iter().take(3).sum::<u32>().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "24000");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "45000");
    }
}
