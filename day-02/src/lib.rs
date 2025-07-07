use std::collections::HashMap;

pub fn process_part1(input: &str) -> String {
    let max: HashMap<&str, usize> = HashMap::from([("blue", 14), ("green", 13), ("red", 12)]);

    let res: usize = input
        .lines()
        .map(|line| {
            let line = line.to_string();

            let split = line.split(": ").collect::<Vec<&str>>();
            let mut id = split
                .first()
                .expect("We have a game id")
                .replace("Game ", "")
                .parse::<usize>()
                .expect("We have a number");

            let sets = split
                .last()
                .expect("We have sets of game")
                .split("; ")
                .collect::<Vec<&str>>();

            for set in &sets {
                let mut valid = true;
                for pair in set.split(", ") {
                    let mut arr = pair.split_ascii_whitespace();

                    let n = arr
                        .next()
                        .expect("We have a number")
                        .parse::<usize>()
                        .expect("It's a valid number");
                    let clr = arr.next().expect("We have a color");

                    if n > *max.get(clr).expect("Colors are valid") {
                        id = 0;
                        valid = false;
                        break;
                    }
                }
                if !valid {
                    break;
                };
            }

            id
        })
        .sum();

    res.to_string()
}

pub fn process_part2(input: &str) -> String {
    let res: usize = input
        .lines()
        .map(|line| {
            let line = line.to_string();

            let split = line.split(": ").collect::<Vec<&str>>();

            let sets = split
                .last()
                .expect("We have sets of game")
                .split("; ")
                .collect::<Vec<&str>>();

            let mut colors: HashMap<&str, usize> =
                HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

            sets.iter().for_each(|set| {
                set.split(", ").for_each(|pair| {
                    let mut arr = pair.split_ascii_whitespace();

                    let n = arr
                        .next()
                        .expect("We have a number")
                        .parse::<usize>()
                        .expect("It's a valid number");
                    let clr = arr.next().expect("We have a color");

                    if n > *colors.get(clr).expect("Colors are rgb") {
                        colors.insert(clr, n);
                    }
                })
            });

            colors.values().into_iter().product::<usize>()
        })
        .sum();

    res.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "8");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "2286");
    }
}
