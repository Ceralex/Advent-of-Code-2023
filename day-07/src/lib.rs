use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug)]
struct Hand {
    cards: String,
    combination: Type,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Hand {
    fn new(cards: String) -> Self {
        let t = Hand::get_type(&cards);
        Self {
            cards,
            combination: t,
        }
    }

    fn get_type(cards: &str) -> Type {
        let cards = cards.chars().collect::<Vec<char>>();
        let mut counts = HashMap::new();

        for c in cards {
            let count = counts.entry(c).or_insert(0);
            *count += 1;
        }

        let mut counts = counts.values().collect::<Vec<&usize>>();
        counts.sort();
        match counts.as_slice() {
            [1, 1, 1, 1, 1] => Type::HighCard,
            [1, 1, 1, 2] => Type::OnePair,
            [1, 2, 2] => Type::TwoPair,
            [1, 1, 3] => Type::ThreeOfAKind,
            [2, 3] => Type::FullHouse,
            [1, 4] => Type::FourOfAKind,
            [5] => Type::FiveOfAKind,
            _ => panic!("Invalid hand"),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.combination == other.combination
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.combination == other.combination {
            compare_cards(&self.cards, &other.cards)
        } else {
            self.combination.cmp(&other.combination)
        }
    }
}

fn compare_cards(a: &str, b: &str) -> Ordering {
    let order = "AKQJT98765432";

    for (char_a, char_b) in a.chars().zip(b.chars()) {
        let index_a = order.find(char_a).expect("Valid chard");
        let index_b = order.find(char_b).expect("Valid char");

        if index_a != index_b {
            return index_a.cmp(&index_b);
        }
    }

    Ordering::Equal
}

pub fn process_part1(input: &str) -> String {
    let mut game: Vec<(Hand, usize)> = input
        .lines()
        .map(|line| {
            let mut values = line.split_ascii_whitespace();

            let hand = Hand::new(values.next().expect("Valid hand").to_string());
            let bid = values
                .next()
                .expect("Valid bid")
                .parse::<usize>()
                .expect("Valid number");

            (hand, bid)
        })
        .collect();

    game.sort_by(|a, b| b.0.cmp(&a.0));

    game.iter()
        .enumerate()
        .map(|(i, (_, b))| b * (i + 1))
        .sum::<usize>()
        .to_string()
}

pub fn process_part2(_input: &str) -> String {
    todo!("Part 2")
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "6440");
    }

    /* #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "");
    } */
}
