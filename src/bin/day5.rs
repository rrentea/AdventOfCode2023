use aoc2023::utils;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1},
    combinator::fail,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};
use rayon::iter::{ParallelIterator, IntoParallelIterator};
use indicatif::ParallelProgressIterator;

#[derive(Debug, Copy, Clone)]
struct Range {
    dst: u64,
    src: u64,
    len: u64,
}

#[derive(Debug, Copy, Clone)]
struct Range2 {
    src: u64,
    len: u64,
}

#[derive(Debug, Clone)]
struct RangeMap {
    ranges: Vec<Range>,
}

impl RangeMap {
    fn get(&self, n: u64) -> u64 {
        for range in self.ranges.iter() {
            if (range.src..range.src + range.len).contains(&n) {
                return range.dst + (n - range.src);
            }
        }
        n
    }

    fn from(ranges: &[Range]) -> Self {
        let mut r = vec![];
        for range in ranges.iter() {
            r.push(*range);
        }
        r.sort_by_key(|range| range.src);

        let mut negative_ranges: Vec<Range> = vec![];
        for i in 0..r.len() - 1 {
            let start = r[i].src + r[i].len;
            let len = r[i + 1].src - start;
            negative_ranges.push(Range {
                dst: start,
                src: start,
                len,
            })
        }
        r.append(&mut negative_ranges);
        r.sort_by_key(|range| range.src);
        Self { ranges: r }
    }
}

fn range(input: &str) -> IResult<&str, Range> {
    let (res, values) = separated_list1(space1, nom::character::complete::u64)(input)?;
    if values.len() == 3 {
        Ok((
            res,
            Range {
                dst: values[0],
                src: values[1],
                len: values[2],
            },
        ))
    } else {
        fail("Too many values")
    }
}

fn parse(input: &str) -> IResult<&str, (Vec<u64>, Vec<Vec<Range>>)> {
    let (res, seeds) = preceded(
        tag("seeds: "),
        separated_list1(space1, nom::character::complete::u64),
    )(input)?;
    let (res, _) = newline(res)?;
    let (res, _) = newline(res)?;
    let (res, seed2soil) =
        preceded(tag("seed-to-soil map:\n"), separated_list1(newline, range))(res)?;
    let (res, _) = newline(res)?;
    let (res, _) = newline(res)?;
    let (res, soil2fertilizer) = preceded(
        tag("soil-to-fertilizer map:\n"),
        separated_list1(newline, range),
    )(res)?;
    let (res, _) = newline(res)?;
    let (res, _) = newline(res)?;
    let (res, fertilizer2water) = preceded(
        tag("fertilizer-to-water map:\n"),
        separated_list1(newline, range),
    )(res)?;
    let (res, _) = newline(res)?;
    let (res, _) = newline(res)?;
    let (res, water2light) = preceded(
        tag("water-to-light map:\n"),
        separated_list1(newline, range),
    )(res)?;
    let (res, _) = newline(res)?;
    let (res, _) = newline(res)?;
    let (res, light2temperature) = preceded(
        tag("light-to-temperature map:\n"),
        separated_list1(newline, range),
    )(res)?;
    let (res, _) = newline(res)?;
    let (res, _) = newline(res)?;
    let (res, temperature2humidity) = preceded(
        tag("temperature-to-humidity map:\n"),
        separated_list1(newline, range),
    )(res)?;
    let (res, _) = newline(res)?;
    let (res, _) = newline(res)?;
    let (res, humidity2location) = preceded(
        tag("humidity-to-location map:\n"),
        separated_list1(newline, range),
    )(res)?;
    let (res, _) = newline(res)?;

    Ok((
        res,
        (
            seeds,
            vec![
                seed2soil,
                soil2fertilizer,
                fertilizer2water,
                water2light,
                light2temperature,
                temperature2humidity,
                humidity2location,
            ],
        ),
    ))
}

fn part1(input: String) -> u64 {
    let (_, (seeds, values)) = parse(&input).unwrap();
    println!("Done parsing");
    let seed2soil = RangeMap::from(&values[0]);
    let soil2fertilizer = RangeMap::from(&values[1]);
    let fertilizer2water = RangeMap::from(&values[2]);
    let water2light = RangeMap::from(&values[3]);
    let light2temperature = RangeMap::from(&values[4]);
    let temperature2humidity = RangeMap::from(&values[5]);
    let humidity2location = RangeMap::from(&values[6]);

    println!("Done making maps");

    seeds
        .iter()
        .map(|seed| {
            let soil = seed2soil.get(*seed);
            let fertilizer = soil2fertilizer.get(soil);
            let water = fertilizer2water.get(fertilizer);
            let light = water2light.get(water);
            let temperature = light2temperature.get(light);
            let humidity = temperature2humidity.get(temperature);
            humidity2location.get(humidity)
        })
        .min()
        .unwrap()
}

fn part2(input: String) -> u64 {
    let (_, (seeds, values)) = parse(&input).unwrap();

    let seed_ranges: Vec<_> = seeds
        .chunks(2)
        .map(|seed_range| Range2 {
            src: seed_range[0],
            len: seed_range[1],
        })
        .collect();
    let seed2soil = RangeMap::from(&values[0]);
    let soil2fertilizer = RangeMap::from(&values[1]);
    let fertilizer2water = RangeMap::from(&values[2]);
    let water2light = RangeMap::from(&values[3]);
    let light2temperature = RangeMap::from(&values[4]);
    let temperature2humidity = RangeMap::from(&values[5]);
    let humidity2location = RangeMap::from(&values[6]);

    seed_ranges
        .into_par_iter()
        .progress()
        .map(|range| {
            let mut min = u64::MAX;
            for seed in range.src..range.src + range.len {
                let soil = seed2soil.get(seed);
                let fertilizer = soil2fertilizer.get(soil);
                let water = fertilizer2water.get(fertilizer);
                let light = water2light.get(water);
                let temperature = light2temperature.get(light);
                let humidity = temperature2humidity.get(temperature);
                let loc = humidity2location.get(humidity);

                if loc < min {
                    min = loc
                }
            }
            min
        })
        .min()
        .unwrap()
}

fn main() {
    match utils::read_file("src/data/day5/input") {
        Ok(input) => {
            println!("{}", part1(input.to_string()));
            println!("{}", part2(input.to_string()));
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
        }
    }
}
