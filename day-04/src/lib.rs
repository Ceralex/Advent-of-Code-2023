use std::collections::BTreeSet;

pub fn process_part1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let content = line.split(": ").last().expect("Get content");
            let mut parts = content.split(" | ");
            let winnings: BTreeSet<usize> = parts
                .next()
                .expect("Get winnings")
                .split_ascii_whitespace()
                .map(|n| n.parse().expect("Parse number"))
                .collect();

            let numbers: BTreeSet<usize> = parts
                .next()
                .expect("Get numbers")
                .split_ascii_whitespace()
                .map(|n| n.parse().expect("Parse number"))
                .collect();

            let intersection_count = winnings.intersection(&numbers).count();

            if intersection_count > 0 {
                2usize.pow((intersection_count - 1) as u32)
            } else {
                0
            }
        })
        .sum::<usize>()
        .to_string()
}

#[derive(Debug)]
struct Card {
    id: usize,
    winnings: usize,
}

pub fn process_part2(input: &str) -> String {
    let mut cards: Vec<Card> = vec![];

    input.lines().enumerate().for_each(|(i, line)| {
        let content = line.split(": ").last().expect("Get content");
        let mut parts = content.split(" | ");
        let winnings: BTreeSet<usize> = parts
            .next()
            .expect("Get winnings")
            .split_ascii_whitespace()
            .map(|n| n.parse().expect("Parse number"))
            .collect();

        let numbers: BTreeSet<usize> = parts
            .next()
            .expect("Get numbers")
            .split_ascii_whitespace()
            .map(|n| n.parse().expect("Parse number"))
            .collect();

        let intersection_count = winnings.intersection(&numbers).count();

        cards.push(Card {
            id: i,
            winnings: intersection_count,
        });
    });

    let mut times: Vec<usize> = vec![1; cards.len()];
    cards.iter().enumerate().for_each(|(i, c)| {
        for _ in 0..times[i] {
            for j in cards[i + 1..=i + c.winnings].iter() {
                times[j.id] += 1;
            }
        }
    });

    times.iter().sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "13");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "30");
    }
}
