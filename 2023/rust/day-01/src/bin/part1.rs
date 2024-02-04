fn main() {
    let input = include_str!("../../input1.txt");

    let result = part1(input);
    println!("{}", result);
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut i = line.chars().filter_map(|c| c.to_digit(10));

            let first = i
                .next()
                .expect("there should be at least one number on each line");
            // if there is only one number, it will be consumed by the first ".next()"
            let last = i.last();

            match last {
                Some(last) => first * 10 + last,
                None => first * 11,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part1(
                "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            ),
            142
        );
    }
}
