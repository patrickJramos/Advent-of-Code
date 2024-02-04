fn main() {
    let input = include_str!("../../input1.txt");

    let result = part2(input);
    println!("{}", result);
}

fn part2(input: &str) -> usize {
    let possible_gear_positions: Vec<Point> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| (c == '*').then(|| Point { x, y }))
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
                        start: Point { x: n, y },
                        end: Point {
                            x: n + str.len() - 1,
                            y,
                        },
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

    possible_gear_positions
        .iter()
        .map(|p| {
            let ns: Vec<usize> = number_positions
                .iter()
                .filter_map(|n_pos| is_number_near_symbol(p, n_pos).then(|| n_pos.value))
                .collect();

            (ns.len() == 2).then(|| ns[0] * ns[1]).unwrap_or(0)
        })
        .sum()
}

fn is_number_near_symbol(point1: &Point, number_pos: &NumberPosition) -> bool {
    DIFS.iter().any(|dif| {
        let point_near = Point {
            x: point1.x.saturating_add_signed(dif.0),
            y: point1.y.saturating_add_signed(dif.1),
        };

        point_near.y == number_pos.start.y
            && (number_pos.start.x..=number_pos.end.x).contains(&point_near.x)
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

#[derive(Debug)]
struct NumberPosition {
    start: Point,
    end: Point,
    value: usize,
}

#[derive(Debug)]
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
            part2(
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
            467835
        );
    }
}
