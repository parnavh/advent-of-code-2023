use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let lines = input.lines();

    let matches = lines
        .clone()
        .map(|line| {
            let mut split = line.split(":").last().unwrap().split("|").map(|s| s.trim());

            let mut win_map: HashMap<&str, bool> = HashMap::new();

            split.next().unwrap().split_whitespace().for_each(|s| {
                win_map.insert(s, true);
            });

            let cards = split
                .next()
                .unwrap()
                .split_whitespace()
                .collect::<Vec<&str>>();

            let count = cards
                .iter()
                .map(|card| if win_map.contains_key(card) { 1 } else { 0 })
                .sum::<u32>();

            count
        })
        .collect::<Vec<_>>();

    let card_count = (0..lines.clone().count())
        .map(|i| (i, 1))
        .collect::<HashMap<usize, u32>>();

    matches
        .iter()
        .enumerate()
        .fold(card_count, |mut acc, (i, count)| {
            let to_add = *acc.get(&i).unwrap();

            for x in i + 1..i + *count as usize + 1 {
                acc.entry(x).and_modify(|e| {
                    *e += to_add;
                });
            }

            acc
        })
        .values()
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, "30");
    }
}
