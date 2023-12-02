use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let colors = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let sum: u32 = input
        .lines()
        .map(|line| {
            let mut it = line.split(": ");

            let game: u32 = it
                .next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse()
                .unwrap();

            let pass = it.next().unwrap().split("; ").all(|sub| {
                sub.split(", ").all(|ind| {
                    let mut i = ind.split(' ');
                    let count: u32 = i.next().unwrap().parse().unwrap();
                    let color = i.next().unwrap();

                    count <= colors[color]
                })
            });

            if pass {
                game
            } else {
                0
            }
        })
        .sum();

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, "8");
    }
}
