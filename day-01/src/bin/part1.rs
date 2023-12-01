fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|f| f.to_digit(10));

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
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, "142");
    }
}
