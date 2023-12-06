use std::cmp::max;
use std::cmp::min;
fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);
    dbg!(output);
}

fn is_gear(ch: char) -> bool {
    if ch == '*' {
        true
    } else {
        false
    }
}

fn get_gears(line: &str) -> Vec<usize> {
    let mut indices = Vec::new();
    for (i, ch) in line.chars().enumerate() {
        if is_gear(ch) {
            indices.push(i);
        }
    }

    indices
}

fn get_number(lines: Vec<&str>, (y, x): (usize, usize)) -> (usize, usize) {
    let line = lines[y];

    let chars = line.chars().collect::<Vec<char>>();

    let end = chars[x..]
        .iter()
        .position(|c| !c.is_digit(10))
        .unwrap_or(line.len())
        + chars[..x].len();

    let rev = &mut chars.clone()[0..x];
    rev.reverse();

    let start = rev.len()
        - rev
            .iter()
            .position(|c| !c.is_digit(10))
            .unwrap_or(rev.len());

    (start, end)
}

fn process(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let sum: u32 = lines
        .iter()
        .enumerate()
        .map(|(line_index, line)| {
            let gears = get_gears(line);

            if gears.len() == 0 {
                return 0;
            }

            gears
                .iter()
                .map(|g| {
                    let mut matches = Vec::new();

                    let x_range = g - 1..g + 2;
                    let y_range =
                        max(0, line_index as i32 - 1) as usize..min(lines.len(), line_index + 2);

                    y_range.clone().for_each(|y| {
                        x_range.clone().for_each(|x| {
                            if lines[y].chars().nth(x).unwrap_or('*').is_digit(10) {
                                matches.push((y, x));
                            }
                        });
                    });

                    let mut numbers = matches
                        .iter()
                        .map(|m| (m.0, get_number(lines.clone(), *m)))
                        .collect::<Vec<(usize, (usize, usize))>>();

                    numbers.dedup();

                    if numbers.len() == 2 {
                        numbers
                            .clone()
                            .iter()
                            .map(|(line, (start, end))| {
                                let num = lines[*line]
                                    .chars()
                                    .skip(*start)
                                    .take(end - start)
                                    .collect::<String>();

                                num.parse::<u32>().unwrap()
                            })
                            .product::<u32>()
                    } else {
                        0
                    }
                })
                .sum::<u32>()
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
        assert_eq!(result, "467835");
    }
}
