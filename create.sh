#!/bin/bash

FOLDER="day-$1"

TEMPLATE='fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    "Hello, world!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process("hi");
        assert_eq!(result, "");
    }
}

'

cargo new "$FOLDER"
cd "$FOLDER"

touch input.txt
rm src/main.rs

mkdir src/bin
echo "$TEMPLATE" > src/bin/part1.rs
touch src/bin/part2.rs

exec cargo watch -x test
