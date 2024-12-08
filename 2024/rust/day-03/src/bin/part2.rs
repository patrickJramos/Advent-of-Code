fn main() {
    let input = include_str!("../../input.txt");

    let result = part1(input);
    println!("{}", result);
}

fn part1(input: &str) -> u32 {
    let mut enabled = true;

    let mut i = 0;

    let mut sum = 0;
    while i < input.len() {
        if !enabled {
            if input[i..].starts_with("do()") {
                enabled = true;
                i += "do()".len();
                continue;
            }
            i += 1;
            continue;
        }
        if input[i..].starts_with("don't()") {
            enabled = false;
            i += "don't()".len();
            continue;
        }

        if !input[i..].starts_with("mul(") {
            i += 1;
            continue;
        }
        i += "mul(".len();

        let mut d1 = 0;
        let mut d2 = 0;

        while let Some(n) = input.chars().nth(i).and_then(|c| c.to_digit(10)) {
            d1 = d1 * 10 + n;
            i += 1;
        }
        if input.chars().nth(i) != Some(',') {
            i += 1;
            continue;
        }
        i += 1;
        while let Some(n) = input.chars().nth(i).and_then(|c| c.to_digit(10)) {
            d2 = d2 * 10 + n;
            i += 1;
        }

        if input.chars().nth(i) != Some(')') {
            i += 1;
            continue;
        }

        sum += d1 * d2;

        i += 1;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part1("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );
    }
}
