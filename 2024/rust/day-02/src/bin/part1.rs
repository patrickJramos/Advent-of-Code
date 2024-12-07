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
            let mut windows = levels.windows(2);
            let first = windows.next().expect("invalid input");
            let diff = first[0] - &first[1];

            if diff == 0 || diff.abs() > 3 {
                return false;
            }

            return windows.all(|step| {
                let step_diff = step[0] - &step[1];

                step_diff.signum() == diff.signum() && step_diff.abs() <= 3
            });
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
