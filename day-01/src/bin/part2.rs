fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|line| {
            let mut index = 0;
            let modified_line = std::iter::from_fn(move || {
                let new_line = &line[index..];

                let result = if new_line.starts_with("one") {
                    Some('1')
                } else if new_line.starts_with("two") {
                    Some('2')
                } else if new_line.starts_with("three") {
                    Some('3')
                } else if new_line.starts_with("four") {
                    Some('4')
                } else if new_line.starts_with("five") {
                    Some('5')
                } else if new_line.starts_with("six") {
                    Some('6')
                } else if new_line.starts_with("seven") {
                    Some('7')
                } else if new_line.starts_with("eight") {
                    Some('8')
                } else if new_line.starts_with("nine") {
                    Some('9')
                } else {
                    new_line.chars().next()
                };

                index += 1;

                result
            });

            let mut it = modified_line.filter_map(|f| f.to_digit(10));

            let first = it.next().unwrap();

            let last = it.last();

            match last {
                Some(num) => first * 10 + num,
                None => first * 10 + first,
            }
        })
        .sum();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, "281");
    }
}
