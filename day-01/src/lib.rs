use std::collections::HashMap;

pub fn process_part1(input: &str) -> String {
    let mut res = 0;

    input.lines().for_each(|line| {
        let nums: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

        res += nums[0] * 10 + nums.last().expect("At least one digit");
    });

    res.to_string()
}

pub fn process_part2(input: &str) -> String {
    let values = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let mut res = 0;

    input.lines().for_each(|line| {
        let mut line = line.to_string();

        let mut i = 0;

        while i < line.len() {
            let mut j = i + 1;

            while j <= line.len() {
                if let Some(v) = values.get(&line[i..j]) {
                    line.replace_range(i..j, v);
                    i += 1;
                    j = i + 1;
                } else {
                    j += 1;
                }
            }
            i += 1;
        }

        let nums: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

        res += nums[0] * 10 + nums[nums.len() - 1];
    });

    res.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    /*  #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "142");
    } */

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "281");
    }
}
