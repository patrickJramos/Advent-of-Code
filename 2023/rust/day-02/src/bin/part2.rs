fn main() {
    let input = include_str!("../../input1.txt");

    let result = part2(input);
    println!("{}", result);
}
use std::collections::HashMap;

fn part2(input: &str) -> u32 {
    let games = input.lines();

    games
        .map(|game| -> u32 {
            let sets = game
                .splitn(2, ':')
                .skip(1)
                .next()
                .expect("a valid game")
                .split(';');

            sets.fold(HashMap::new(), |mut map, set| {
                set.split(',').for_each(|cube_info| {
                    let mut split = cube_info.trim().split(' ');
                    let num = split
                        .next()
                        .unwrap()
                        .parse::<u32>()
                        .expect("a valid number");
                    let color = split.next().expect("a valid color");

                    map.entry(color)
                        .and_modify(|v| *v = num.max(*v))
                        .or_insert(num);
                });

                map
            })
            .values()
            .product()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part2(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            2286
        );
    }
}
