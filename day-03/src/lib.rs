use std::{collections::BTreeMap, ops::Range};

pub fn process_part1(input: &str) -> String {
    let mut values: BTreeMap<(i32, i32), u32> = BTreeMap::new();
    let mut symbols: Vec<(usize, usize)> = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {}
                token if token.is_ascii_digit() => {
                    values.insert(
                        (y as i32, x as i32),
                        token.to_digit(10).expect("is number, for prev match"),
                    );
                    ()
                }
                _ => symbols.push((y, x)),
            };
        }
    }

    let positions = [
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let pos_with_num: Vec<(i32, i32)> = symbols
        .iter()
        .flat_map(|(y, x)| {
            positions.iter().map(|outer_pos| {
                // outer_pos.x + pos.x, .y + .y
                (outer_pos.0 + *y as i32, outer_pos.1 + *x as i32)
            })
        })
        .filter(|num| values.contains_key(&(num.0, num.1)))
        .collect();

    pos_with_num
        .iter()
        .map(|(y, x)| {
            let mut right = x + 1;
            while values.get(&(*y, right)).is_some() {
                right += 1
            }
            right -= 1;
            let mut e: u32 = 0;
            let mut num: u32 = 0;
            while values.get(&(*y, right)).is_some() {
                num += values.get(&(*y, right)).expect("is some") * (10_u32.pow(e));
                values.remove(&(*y, right));
                e += 1;
                right -= 1;
            }
            num
        })
        .sum::<u32>()
        .to_string()
}

#[derive(Debug)]
struct Number {
    pub y: i32,
    pub x: Range<i32>,
    pub value: u32,
}

pub fn process_part2(input: &str) -> String {
    let mut values: Vec<Number> = Vec::new();
    let mut symbols: Vec<(usize, usize)> = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {}
                token if token.is_ascii_digit() => {
                    match values.last_mut() {
                        Some(v) => {
                            if v.y == y as i32 && v.x.end + 1 == x as i32 {
                                v.x.end += 1;
                                v.value = v.value * 10
                                    + token.to_digit(10).expect("is number, for prev match");
                            } else {
                                values.push(Number {
                                    y: (y as i32),
                                    x: (x as i32)..(x as i32),
                                    value: (token.to_digit(10).expect("is number, for prev match")),
                                })
                            }
                        }
                        None => values.push(Number {
                            y: (y as i32),
                            x: (x as i32)..(x as i32),
                            value: (token.to_digit(10).expect("is number, for prev match")),
                        }),
                    }
                    ()
                }
                _ => symbols.push((y, x)),
            };
        }
    }

    let positions = [
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let mut tonz = 0;
    symbols.iter().for_each(|(y, x)| {
        let pos = positions
            .iter()
            .map(|outer_pos| (outer_pos.0 + *y as i32, outer_pos.1 + *x as i32))
            .collect::<Vec<(i32, i32)>>();

        let res = values
            .iter()
            .filter(|v| {
                pos.iter()
                    .filter(|(y, x)| *y == v.y && *x >= v.x.start && *x <= v.x.end)
                    .count()
                    > 0
            })
            .collect::<Vec<&Number>>();

        if res.len() > 1 {
            tonz += res.iter().map(|v| (*v).value).product::<u32>()
        }
    });

    tonz.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "4361");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "467835");
    }
}
