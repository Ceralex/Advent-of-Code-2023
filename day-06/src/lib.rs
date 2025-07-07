pub fn process_part1(input: &str) -> String {
    let mut lines = input.lines();

    let races: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let record_distances: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let my_races: Vec<Vec<usize>> = races
        .iter()
        .map(|t| (0..=*t).map(|i| (t - i) * i).collect())
        .collect();

    record_distances
        .iter()
        .zip(my_races)
        .map(|(r, m)| m.iter().filter(|x| *x > r).count())
        .product::<usize>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut lines = input.lines();

    let race = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("")
        .parse::<usize>()
        .expect("Valid number");

    let record_distance = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("")
        .parse::<usize>()
        .expect("Valid number");

    let my_races = (0..=race).map(|i| (race - i) * i).collect::<Vec<usize>>();

    my_races
        .iter()
        .filter(|x| *x > &record_distance)
        .count()
        .to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "288");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "71503");
    }
}
