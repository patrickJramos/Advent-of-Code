use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");

    let result = part2(input);
    println!("{}", result);
}

fn part2(input: &str) -> usize {
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

    let mut score_map = HashMap::<usize, usize>::new();

    arr.iter().for_each(|(_, n)| {
        let score_ref = score_map.entry(*n).or_insert(0);

        *score_ref += n;
    });

    let result = arr.iter().filter_map(|(n, _)| score_map.get(n)).sum();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part2(
                "3   4
4   3
2   5
1   3
3   9
3   3"
            ),
            31
        );
    }
}
