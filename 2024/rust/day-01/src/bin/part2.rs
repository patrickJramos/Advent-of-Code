use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input1.txt");

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

    let left: Vec<usize> = arr.iter().map(|x| x.0).collect();

    let right: Vec<usize> = arr.iter().map(|x| x.1).collect();

    let mut score_map = HashMap::<usize, usize>::new();

    right.iter().for_each(|n| {
        let score_ref = score_map.entry(*n).or_insert(0);

        *score_ref += n;
    });

    let result = left.iter().map(|n| score_map.get(n).unwrap_or(&0)).sum();

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
