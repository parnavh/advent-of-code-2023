use std::cmp::min;
fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);
    dbg!(output);
}

fn find_next_digit(line: &str, index: &mut usize) {
    while *index < line.len() && !line.chars().nth(*index).unwrap().is_digit(10) {
        *index += 1;
    }
}

fn find_digit_boundary(line: &str, mut index: usize) -> usize {
    while index < line.len() && line.chars().nth(index).unwrap().is_digit(10) {
        index += 1;
    }

    index
}

fn is_symbol(ch: char) -> bool {
    if ch == '.' || ch.is_digit(10) {
        false
    } else {
        true
    }
}

fn process(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let sum: u32 = lines
        .iter()
        .enumerate()
        .map(|(line_index, line)| {
            let mut char_index = 0;
            let mut sum = 0;

            while char_index < line.len() - 1 {
                find_next_digit(line, &mut char_index);
                if char_index == line.len() {
                    break;
                }

                let last_index = find_digit_boundary(line, char_index);

                let digit: u32 = line[char_index..last_index].parse().unwrap();

                let mut symbol = false;

                if char_index > 0 {
                    symbol = is_symbol(line.chars().nth(char_index - 1).unwrap());
                }

                if !symbol && last_index < line.len() - 1 {
                    symbol = is_symbol(line.chars().nth(last_index).unwrap());
                }

                if !symbol && line_index > 0 {
                    let prev = lines[line_index - 1];
                    symbol = ((if char_index == 0 { 0 } else { char_index - 1 })
                        ..min(line.len(), last_index + 1))
                        .any(|i| {
                            let char = prev.chars().nth(i).unwrap();

                            is_symbol(char)
                        });
                }

                if !symbol && line_index < lines.len() - 1 {
                    let next = lines[line_index + 1];
                    symbol = ((if char_index == 0 { 0 } else { char_index - 1 })
                        ..min(line.len(), last_index + 1))
                        .any(|i| {
                            let char = next.chars().nth(i).unwrap();

                            is_symbol(char)
                        });
                }

                sum += symbol as u32 * digit;

                char_index = last_index;
            }

            sum
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
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, "4361");
    }
}
