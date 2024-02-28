use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, multispace0, space1, u64},
    multi::separated_list1,
    sequence::{pair, separated_pair, tuple},
    IResult,
};

fn main() {
    let input = include_str!("../../input1.txt");
    let result = part2(input);
    println!("{}", result);
}

fn part2(input: &str) -> u64 {
    let (input, seeds) = seeds(input).expect("a valid input");

    let (input, _) = multispace0::<&str, ()>(input).expect("a valid input");

    let (_, mappers) = all_mappers(input).expect("a valid input");

    let odered_mappers = find_mappers_in_order(&mappers, "seed", "location");

    todo!();
    // seeds
    //     .iter()
    //     .map(|(min_seed, distance)| {
    //         odered_mappers
    //             .iter()
    //             .fold(*seed, |acc, mapper| mapper.map_to_destination(acc))
    //     })
    //     .min()
    //     .unwrap()
}

fn find_mappers_in_order<'a>(
    mappers: &'a Vec<Mapper>,
    source: &'a str,
    destination: &'a str,
) -> Vec<&'a Mapper<'a>> {
    let mut found_mappers = vec![];

    let mut mapper = mappers
        .iter()
        .find(|x| x.source == source)
        .expect(format!("mapper not found for {source}").as_str());

    while mapper.destination != destination {
        found_mappers.push(mapper);

        mapper = mappers
            .iter()
            .find(|x| x.source == mapper.destination)
            .expect(format!("mapper not found for {}", mapper.destination).as_str());
    }

    found_mappers
}

fn seeds(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    let (input, _) = tag("seeds: ")(input)?;

    let (input, seed_nums) = separated_list1(tag(" "), u64)(input)?;

    let seed_nums = seed_nums.chunks(2).map(|x| (x[0], x[1])).collect();

    Ok((input, seed_nums))
}

fn all_mappers(input: &str) -> IResult<&str, Vec<Mapper>> {
    separated_list1(pair(line_ending, line_ending), mappers)(input)
}

fn flatten_mappers<'a>(mapper1: &mut Mapper<'a>, mapper2: &mut Mapper<'a>) -> Mapper<'a> {
    if mapper1.destination != mapper2.source {
        panic!("mappers not connected");
    }

    let mut rgs1 = mapper1.ranges.clone();
    let mut rgs2 = mapper2.ranges.clone();

    rgs1.sort_by(|x, y| x.source_start.cmp(&y.source_start));
    rgs2.sort_by(|x, y| x.source_start.cmp(&y.source_start));

    let mut new_ranges = vec![];

    for r1 in &rgs1 {
        let overlaps: Vec<&MapperRange> = mapper2
            .ranges
            .iter()
            .filter(|r2| {
                (r2.source_start <= r1.source_end() && r2.source_start >= r1.source_start)
                    || (r2.source_end() <= r1.source_end() && r2.source_end() >= r1.source_start)
            })
            .collect();

        if overlaps.len() == 0 {
            new_ranges.push(r1.clone());
            continue;
        }

        for r2 in overlaps {
            if (r2.source_start < r1.destination_start) {
                new_ranges.push(MapperRange {
                    source_start: r2.source_start,
                    destination_start: r2.destination_start,
                    size: r1.destination_start - r2.source_end(),
                });
            }

            if (r2.source_end() > r1.destination_end()) {
                new_ranges.push(MapperRange {
                    source_start: r2.source_start,
                    destination_start: r1.destination_end() + 1,
                    size: r2.source_end() - r1.destination_end(),
                });
            }

            if (r2.source_start != r1.source_start) {
                new_ranges.push(MapperRange {
                    source_start: r1.source_start,
                    destination_start: r1.destination_start,
                    size: r2.source_start - r1.source_start,
                });
            }

            if (r2.source_end() != r1.source_end()) {
                new_ranges.push(MapperRange {
                    source_start: r2.source_end() + 1,
                    destination_start: r1.destination_start,
                    size: r1.source_end() - r2.source_end(),
                });
            }

            new_ranges.push(MapperRange {
                source_start: r1.source_end() + 1,
                destination_start: r1.destination_start,
                size: r2.source_end() - r1.source_end(),
            });
        }
    }

    Mapper {
        source: mapper1.source,
        destination: mapper2.destination,
        ranges: new_ranges,
    }
}

/*
seed-to-soil map:
50 98 2
52 50 48 */
fn mappers(input: &str) -> IResult<&str, Mapper> {
    let (input, (source, destination)) = separated_pair(alpha1, tag("-to-"), alpha1)(input)?;

    let (input, _) = tag(" map:")(input)?;
    let (input, _) = multispace0(input)?;

    let (input, ranges) = mapper_ranges(input)?;

    Ok((
        input,
        Mapper {
            source,
            destination,
            ranges,
        },
    ))
}

fn mapper_ranges(input: &str) -> IResult<&str, Vec<MapperRange>> {
    separated_list1(line_ending, mapper_range)(input)
}

fn mapper_range(input: &str) -> IResult<&str, MapperRange> {
    let (input, (destination_start, _, source_start, _, size)) =
        tuple((u64, space1, u64, space1, u64))(input)?;

    Ok((
        input,
        MapperRange {
            source_start,
            destination_start,
            size,
        },
    ))
}

#[derive(Debug, Clone)]
struct MapperRange {
    source_start: u64,
    destination_start: u64,
    size: u64,
}

impl MapperRange {
    fn diff(&self) -> u64 {
        self.destination_start - self.source_start
    }
    fn source_end(&self) -> u64 {
        self.source_start + self.size - 1
    }

    fn destination_end(&self) -> u64 {
        self.destination_start + self.size - 1
    }
}

#[derive(Debug, Clone)]
struct Mapper<'a> {
    source: &'a str,
    destination: &'a str,
    ranges: Vec<MapperRange>,
}

impl Mapper<'_> {
    fn map_to_destination(&self, source_num: u64) -> u64 {
        self.ranges
            .iter()
            .find_map(|range| {
                (range.source_start <= source_num && source_num < range.source_start + range.size)
                    .then(|| source_num + range.diff())
            })
            .unwrap_or(source_num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(
            part2(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            ),
            35
        );
    }
}
