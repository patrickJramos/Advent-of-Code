fn main() {
    let input = include_str!("../../input.txt");

    let result = part1(input);
    println!("{}", result);
}

fn part1(input: &str) -> u32 {
    let muls = input.match_indices("mul(");

    muls.filter_map(|(idx, _s)| {
        let mut d1 = 0;
        let mut d2 = 0;

        let mut i = idx + 4;
        while let Some(n) = input.chars().nth(i).and_then(|c| c.to_digit(10)) {
            d1 = d1 * 10 + n;
            i += 1;
        }
        if input.chars().nth(i) != Some(',') {
            return None;
        }
        i += 1;
        while let Some(n) = input.chars().nth(i).and_then(|c| c.to_digit(10)) {
            d2 = d2 * 10 + n;
            i += 1;
        }

        if input.chars().nth(i) != Some(')') {
            return None;
        }

        Some(d1 * d2)
    })
    .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }
}
