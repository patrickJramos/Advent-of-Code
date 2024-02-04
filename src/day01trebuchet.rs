pub fn trebuchet_2(str: String) -> usize {
    let split = str.split('\n');
    let mut sum: usize = 0;

    split.for_each(|line| {
        let mut first: Option<usize> = None;
        let mut last: Option<usize> = None;

        let len = line.len();
        let mut n = 0;

        while n < len {
            let num: Option<usize> = if let Some(digit) = line[n..n + 1].parse::<usize>().ok() {
                Some(digit as usize)
            } else {
                NUM_STRS.iter().enumerate().find_map(|(i, &str)| {
                    if line[n..].starts_with(str) {
                        // we skip to the last character of the string. we can't skip the whole string
                        // because the end of a number might match with the start of another.
                        // looking at the string that might match, there are no endings that have
                        // 2 characters in common with the start of another string, se we can go to len - 1

                        n += str.len() - 2;
                        Some(i + 1)
                    } else {
                        None
                    }
                })
            };

            first = first.or(num);
            last = num.or(last);
            n += 1;
        }

        let s = first.unwrap_or(0) * 10 + last.unwrap_or(0);

        sum += s;
    });

    sum
}

const NUM_STRS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
