pub mod day_4 {
    use std::collections::HashMap;

    pub fn part2(input: &str) -> usize {
        let game_wins = input.lines().enumerate().map(|(game_num, line)| {
            let mut split = line
                .split(':')
                .skip(1)
                .next()
                .expect("a valid game")
                .split("|");

            let good_nums: Vec<&str> = split
                .next()
                .expect("a valid game")
                .split_whitespace()
                .collect();
            let my_nums = split.next().expect("a valid game").split_whitespace();

            let wins = my_nums.filter(|num| good_nums.contains(num)).count();

            (game_num, wins)
        });

        game_wins
            .fold(HashMap::new(), |mut card_counts, (game_num, wins)| {
                let card_entry = card_counts.entry(game_num);
                let count = *card_entry.and_modify(|x| *x += 1).or_insert(1);

                (1..=wins).for_each(|win_num| {
                    card_counts
                        .entry(game_num + win_num)
                        .and_modify(|x| {
                            *x += count;
                        })
                        .or_insert(count);
                });

                card_counts
            })
            .values()
            .sum()
    }
}
