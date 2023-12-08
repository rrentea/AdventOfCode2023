use aoc2023::utils;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

fn parse_input(input: &str) -> IResult<&str, (Vec<u64>, Vec<u64>)> {
    let (remaining, times) = preceded(
        tuple((tag("Time:"), space1)),
        separated_list1(space1, nom::character::complete::u64),
    )(input)?;
    let (remaining, _) = newline(remaining)?;
    let (remaining, distances) = preceded(
        tuple((tag("Distance:"), space1)),
        separated_list1(space1, nom::character::complete::u64),
    )(remaining)?;
    let (remaining, _) = newline(remaining)?;

    Ok((remaining, (times, distances)))
}

fn parse_input2(input: &str) -> IResult<&str, (u64, u64)> {
    let (remaining, times) = preceded(
        tuple((tag("Time:"), space1)),
        separated_list1(space1, digit1),
    )(input)?;
    let (remaining, _) = newline(remaining)?;
    let (remaining, distances) = preceded(
        tuple((tag("Distance:"), space1)),
        separated_list1(space1, digit1),
    )(remaining)?;
    let (remaining, _) = newline(remaining)?;

    Ok((
        remaining,
        (
            times.join("").parse::<u64>().unwrap(),
            distances.join("").parse::<u64>().unwrap(),
        ),
    ))
}

fn wins(race_duration: u64, record_dist: u64) -> f64 {
    let r = race_duration as f64;
    let d = record_dist as f64;
    let sqrt = (r * r - 4.0 * d).sqrt();
    let h1 = (-1.0 * r + sqrt) / -2.0;
    let h2 = (-1.0 * r - sqrt) / -2.0;

    if sqrt.fract() == 0.0 {
        h2.floor() - h1.ceil() + 1.0 - 2.0
    } else {
        h2.floor() - h1.ceil() + 1.0
    }
}

fn part1(input: String) -> u64 {
    let (_, (times, distances)) = parse_input(&input).unwrap();
    let options: Vec<u64> = times
        .iter()
        .zip(distances.iter())
        .map(|(race_time, record_dist)| {
            wins(*race_time, *record_dist) as u64
        })
        .collect();
    let mut res = 1;
    for n in options.iter() {
        res *= n;
    }
    res
}

fn part2(input: String) -> u64 {
    let (_, (time, record_dist)) = parse_input2(&input).unwrap();
    wins(time, record_dist) as u64
}

fn main() {
    match utils::read_file("src/data/day6/input") {
        Ok(input) => {
            println!("{}", part1(input.to_string()));
            println!("{}", part2(input.to_string()));
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
        }
    }
}
