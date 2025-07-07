use std::ops;

#[derive(Debug)]
struct Range {
    dest_start: usize,
    source_start: usize,
    length: usize,
}

impl Range {
    fn source_contains(&self, index: usize) -> bool {
        index >= self.source_start && index < self.source_start + self.length
    }
    fn translate(&self, index: usize) -> usize {
        self.dest_start + (index - self.source_start)
    }
    fn dest_contains(&self, index: usize) -> bool {
        index >= self.dest_start && index < self.dest_start + self.length
    }
}

pub fn process_part1(input: &str) -> String {
    let mut sections = input.split("\n\n");

    let seeds: Vec<usize> = sections
        .next()
        .expect("Seeds section")
        .split("seeds: ")
        .last()
        .unwrap()
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut result = seeds.clone();

    sections.for_each(|sec| {
        let ranges = sec
            .split("map:\n")
            .last()
            .expect("Values")
            .split("\n")
            .map(|range| {
                let mut range = range.split(" ");
                let dest_start = range.next().expect("dest_start").parse::<usize>().unwrap();
                let source_start = range
                    .next()
                    .expect("source_start")
                    .parse::<usize>()
                    .unwrap();
                let length = range.next().expect("length").parse::<usize>().unwrap();

                Range {
                    dest_start,
                    source_start,
                    length,
                }
            })
            .collect::<Vec<Range>>();

        for val in result.iter_mut() {
            for range in ranges.iter() {
                if range.source_contains(*val) {
                    *val = range.translate(*val);
                    break;
                }
            }
        }
    });

    result.iter().min().unwrap().to_string()
}

fn inverse_translate(ranges: &Vec<Range>, index: usize) -> usize {
    for range in ranges.iter() {
        if range.dest_contains(index) {
            return range.source_start + (index - range.dest_start);
        }
    }
    index
}

pub fn process_part2(input: &str) -> String {
    let mut sections = input.split("\n\n");

    let seeds: Vec<ops::Range<usize>> = sections
        .next()
        .expect("Seeds section")
        .split("seeds: ")
        .last()
        .expect("Seeds")
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .chunks(2)
        .map(|r| r[0]..r[0] + r[1])
        .collect();

    let mut translators: Vec<Vec<Range>> = sections
        .map(|sec| {
            sec.split("map:\n")
                .last()
                .expect("Values")
                .split("\n")
                .map(|range| {
                    let mut range = range.split(" ");
                    let dest_start = range.next().expect("dest_start").parse::<usize>().unwrap();
                    let source_start = range
                        .next()
                        .expect("source_start")
                        .parse::<usize>()
                        .unwrap();
                    let length = range.next().expect("length").parse::<usize>().unwrap();

                    Range {
                        dest_start,
                        source_start,
                        length,
                    }
                })
                .collect::<Vec<Range>>()
        })
        .collect();

    // Reverse the translators so we start from the location and go back to the seed
    translators.reverse();

    let mut translation = 0;
    let mut res = 0;
    loop {
        for (i, translator) in translators.iter().enumerate() {
            if i == 0 {
                translation = res;
            }
            translation = inverse_translate(translator, translation);
        }

        if seeds.iter().any(|s| s.contains(&translation)) {
            break;
        }
        res += 1;
    }

    res.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "35");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "46");
    }
}
