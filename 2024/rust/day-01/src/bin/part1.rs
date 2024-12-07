fn main() {
    let input = include_str!("../../input1.txt");

    let result = part1(input);
    println!("{}", result);
}

fn part1(input: &str) -> usize {
    let arr: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            line.split_once("   ")
                .map(|s| {
                    (
                        s.0.parse().expect("invalid number"),
                        s.1.parse().expect("invalid number"),
                    )
                })
                .expect("invalid input")
        })
        .collect();

    let mut left: Vec<usize> = arr.iter().map(|x| x.0).collect();
    left.sort();

    let mut right: Vec<usize> = arr.iter().map(|x| x.1).collect();
    right.sort();

    let result = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| if l > r { l - r } else { r - l })
        .sum();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part1(
                "3   4
4   3
2   5
1   3
3   9
3   3"
            ),
            11
        );
    }
}
