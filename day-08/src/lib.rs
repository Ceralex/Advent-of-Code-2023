use std::collections::BTreeMap;

type Network = BTreeMap<String, (String, String)>;
pub fn process_part1(input: &str) -> String {
    let mut lines = input.lines();

    let instructions = lines
        .next()
        .expect("We have instructions")
        .chars()
        .collect::<Vec<char>>();

    let mut network: Network = BTreeMap::new();
    lines.skip(1).for_each(|l| {
        let mut parts = l.split(" = ");
        let key = parts.next().expect("We have a key").trim().to_string();
        let mut values = parts.next().expect("We have a value").trim().split(", ");

        let left = values
            .next()
            .expect("We have a left value")
            .replace("(", "");
        let right = values
            .next()
            .expect("We have a right value")
            .replace(")", "");

        network.insert(key, (left, right));
    });

    let mut current = "AAA".to_string();

    let mut res = 0;
    let mut index = 0;
    while current != "ZZZ" {
        if index == instructions.len() {
            index = 0;
        }

        let (left, right) = network.get(&current).expect("We have a current");

        current = match instructions[index] {
            'R' => right.to_string(),
            'L' => left.to_string(),
            _ => panic!("Invalid instruction"),
        };

        index += 1;
        res += 1;
    }

    res.to_string()
}

pub fn process_part2(_input: &str) -> String {
    todo!("Part 2")
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "2");
    }

    /* #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "");
    } */
}
