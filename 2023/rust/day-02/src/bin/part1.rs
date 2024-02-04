fn main() {
    let input = include_str!("../../input1.txt");

    let result = part1(input);
    println!("{}", result);
}
use std::collections::HashMap;

fn part1(input: &str) -> usize {
    let max_colors: HashMap<&str, usize> =
        HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let games = input.lines();

    games
        .filter_map(|game| {
            let mut split = game.splitn(2, ':');
            let game_name = split.next().expect("a valid game");
            let mut sets = split.next().expect("valid sets").split(';');

            sets.all(|set| {
                let mut cubes = set.split(',').map(|cube| {
                    let mut split = cube.trim().split(' ');
                    let num = split.next().unwrap().parse::<usize>().unwrap();
                    let color = split.next().unwrap();

                    (num, color)
                });

                cubes.all(|(num, color)| num <= *max_colors.get(color).unwrap())
            })
            .then(|| {
                // get game id
                game_name
                    .trim()
                    .split(' ')
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
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
