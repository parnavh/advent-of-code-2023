// Used brute force TODO - better solution

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut paras = input.split("\n\n");

    let seeds = paras
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split(" ")
        .collect::<Vec<&str>>();

    let ranges = (0..seeds.len() / 2)
        .map(|s| {
            let start = seeds[s * 2].parse::<u64>().unwrap();
            let end = start + seeds[s * 2 + 1].parse::<u64>().unwrap();
            start..end
        })
        .collect::<Vec<_>>();

    let transformations = paras.map(|para| {
        let mut lines = para.split("\n");
        lines.next();
        lines.map(|line| {
            line.split(" ")
                .map(|e| e.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
    });

    let result = ranges
        .iter()
        .flat_map(|range| range.clone())
        .map(|seed| {
            let mut result = seed;

            transformations.clone().for_each(|transformation| {
                transformation.clone().any(|rule| {
                    if rule[1] <= result && result < rule[1] + rule[2] {
                        result = rule[0] + result - rule[1];
                        return true;
                    }

                    false
                });
            });

            result
        })
        .min()
        .unwrap();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "seeds: 79 14 55 13

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
56 93 4",
        );
        assert_eq!(result, "46");
    }
}
