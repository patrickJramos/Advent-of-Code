fn main() {
    let input = include_str!("../../input1.txt");

    let result = part2(input);
    println!("{}", result);
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut first: Option<usize> = None;
            let mut last: Option<usize> = None;

            let len = line.len();
            let mut n = 0;

            while n < len {
                let num = line[n..n + 1].parse::<usize>().ok().or_else(|| {
                    NUM_STRS.iter().enumerate().find_map(|(i, &str)| {
                        line[n..].starts_with(str).then(|| {
                            // we skip to the last character of the string. we can't skip the whole string
                            // because the end of a number might match with the start of another.
                            // looking at the strings that might match, there are no endings that have
                            // 2 characters in common with the start of another string, se we can go to len - 1.
                            // we do len - 2 because at the end of the loop we += 1
                            n += str.len() - 2;
                            // nums in the array are sorted, starting at 0, so we add 1
                            i + 1
                        })
                    })
                });

                first = first.or(num);
                last = num.or(last);
                n += 1;
            }

            first.expect("should always exist") * 10 + last.expect("should always exist")
        })
        .sum()
}

const NUM_STRS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part2(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            ),
            281
        );
    }
}
