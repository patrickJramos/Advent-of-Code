fn main() {
    let input = include_str!("../../input1.txt");

    let result = part1(input);
    println!("{}", result);
}

fn part1(input: &str) -> usize {
    let symbol_positions: Vec<Point> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| is_symbol(c).then(|| Point { x, y }))
        })
        .collect();

    let number_positions: Vec<NumberPosition> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let mut nums: Vec<NumberPosition> = Vec::new();

            let mut n = 0;
            while n < line.len() {
                let str = line[n..]
                    .chars()
                    .take_while(|c| c.is_digit(10))
                    .collect::<String>();

                if !str.is_empty() {
                    let num = str.parse::<usize>().unwrap();

                    nums.push(NumberPosition {
                        y,
                        start_x: n,
                        end_x: n + str.len() - 1,
                        value: num,
                    });

                    n += str.len();
                } else {
                    n += 1;
                }
            }

            nums
        })
        .collect();

    symbol_positions
        .iter()
        .flat_map(|p| {
            number_positions
                .iter()
                .filter_map(|n_pos| is_number_near_symbol(p, n_pos).then(|| n_pos.value))
        })
        .sum()
}

fn is_symbol(char: char) -> bool {
    !char.is_digit(10) && char != '.'
}

fn is_number_near_symbol(point1: &Point, number_pos: &NumberPosition) -> bool {
    DIFS.iter().any(|dif| {
        let point_near = Point {
            x: point1.x.saturating_add_signed(dif.0),
            y: point1.y.saturating_add_signed(dif.1),
        };

        point_near.y == number_pos.y
            && (number_pos.start_x..=number_pos.end_x).contains(&point_near.x)
    })
}

const DIFS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

struct NumberPosition {
    y: usize,
    start_x: usize,
    end_x: usize,
    value: usize,
}

struct Point {
    x: usize,
    y: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part1(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            4361
        );
    }
}
