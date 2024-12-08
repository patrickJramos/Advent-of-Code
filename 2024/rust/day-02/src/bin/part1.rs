fn main() {
    let input = include_str!("../../input.txt");

    let result = part1(input);
    println!("{}", result);
}

fn part1(input: &str) -> usize {
    let reports: Vec<Vec<i8>> = input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|s| s.parse().expect("invalid number"))
                .collect()
        })
        .collect();

    reports
        .iter()
        .filter(|levels| {
            let diffs: Vec<_> = levels.windows(2).map(|v| v[0] - v[1]).collect();

            let result = diffs
                .windows(2)
                .all(|v| v[0].signum() == v[1].signum() && v[0].abs() <= 3);

            return result && *diffs.last().unwrap() <= 3;
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part1(
                "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            ),
            2
        );
    }
}