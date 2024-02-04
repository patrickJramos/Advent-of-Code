fn main() {
    let input = include_str!("../../input1.txt");

    let result = part1(input);
    println!("{}", result);
}
use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    let max_colors: HashMap<&str, usize> =
        HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let games = input.lines();

    games
        .filter_map(|game| {
            let mut split = game.splitn(2, ':');
            let game_name = split.next()?;
            let sets = split.next()?.split(';');

            for set in sets {
                let mut cubes_info = set.split(',').map(|cube_info| {
                    let mut split = cube_info.trim().split(' ');
                    let num = split.next().unwrap().parse::<usize>().unwrap();
                    let color = split.next().unwrap();

                    (num, color)
                });

                if cubes_info.any(|(num, color)| num > *max_colors.get(color).unwrap()) {
                    return None;
                }
            }

            let game_id = game_name
                .trim()
                .split(' ')
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            Some(game_id)
        })
        .into_iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colored_cubes() {
        assert_eq!(
            part1(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            8
        );
    }
}
