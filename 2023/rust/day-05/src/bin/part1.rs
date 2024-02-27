use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, multispace0, space1, u64},
    multi::separated_list1,
    sequence::{pair, separated_pair, tuple},
    IResult,
};

fn main() {
    let input = include_str!("../../input1.txt");
    let result = part1(input);
    println!("{}", result);
}

fn part1(input: &str) -> u64 {
    let (input, seeds) = seeds(input).expect("a valid input");

    let (input, _) = multispace0::<&str, ()>(input).expect("a valid input");

    let (_, mappers) = all_mappers(input).expect("a valid input");

    let odered_mappers = find_mappers_in_order(&mappers, "seed", "location");

    seeds
        .iter()
        .map(|seed| {
            odered_mappers
                .iter()
                .fold(*seed, |acc, mapper| mapper.map_to_destination(acc))
        })
        .min()
        .unwrap()
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

fn seeds(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = tag("seeds: ")(input)?;

    let (input, seed_nums) = separated_list1(tag(" "), u64)(input)?;

    Ok((input, seed_nums))
}

fn all_mappers(input: &str) -> IResult<&str, Vec<Mapper>> {
    separated_list1(pair(line_ending, line_ending), mappers)(input)
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

#[derive(Debug)]
struct MapperRange {
    source_start: u64,
    destination_start: u64,
    size: u64,
}

#[derive(Debug)]
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
                    .then(|| {
                        let source_offset = source_num - range.source_start;

                        range.destination_start + source_offset
                    })
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
            part1(
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
