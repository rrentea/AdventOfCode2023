use aoc2023::utils;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    multi::{many0, many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

fn game(input: &str) -> IResult<&str, u32> {
    let (res, id) = preceded(tuple((tag("Card"), many0(tag(" ")))), digit1)(input)?;
    Ok((res, id.parse::<u32>().unwrap()))
}

fn numbers(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (res, (winning, owned)) = separated_pair(
        separated_list1(many1(tag(" ")), digit1),
        delimited(many1(tag(" ")), tag("|"), many1(tag(" "))),
        separated_list1(many1(tag(" ")), digit1),
    )(input)?;
    Ok((
        res,
        (
            winning
                .iter()
                .map(|number| number.parse::<u32>().unwrap())
                .collect(),
            owned
                .iter()
                .map(|number| number.parse::<u32>().unwrap())
                .collect(),
        ),
    ))
}

fn parse_card(line: &str) -> IResult<&str, (u32, Vec<u32>, Vec<u32>)> {
    let (res, (id, (winning, owned))) =
        separated_pair(game, preceded(tag(":"), many0(tag(" "))), numbers)(line)?;
    Ok((res, (id, winning, owned)))
}

fn points(winning: Vec<u32>, owned: Vec<u32>) -> i32 {
    let n = owned
        .iter()
        .filter(|number| winning.contains(number))
        .count();
    if n == 0 {
        0
    } else {
        i32::pow(2, n as u32 - 1)
    }
}

fn score(cards: &Vec<(Vec<u32>, Vec<u32>)>, index: usize) -> u32 {
    let (winning, owned) = &cards[index];
    let n = owned
        .iter()
        .filter(|number| winning.contains(number))
        .count();
    if n == 0 {
        1
    } else {
        1 + (1..=n).map(|i| score(cards, index + i)).sum::<u32>()
    }
}

fn main() {
    match utils::read_file("src/data/day4/input") {
        Ok(content) => {
            let sum: i32 = content
                .lines()
                .map(|line| parse_card(line).unwrap())
                .map(|(_res, (id, winning, owned))| (id, winning, owned))
                .map(|(_id, winning, owned)| points(winning, owned))
                .sum();
            println!("{}", sum);

            let cards: Vec<(Vec<u32>, Vec<u32>)> = content
                .lines()
                .map(|line| parse_card(line).unwrap())
                .map(|(_res, (_id, winning, owned))| (winning, owned))
                .collect();

            let sum: u32 = (0..cards.len()).map(|index| score(&cards, index)).sum();
            println!("{}", sum);
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
        }
    }
}
