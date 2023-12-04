use std::collections::HashSet;

use aoc2023::utils;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

fn game(input: &str) -> IResult<&str, u32> {
    let (res, id) = preceded(tuple((tag("Card"), space1)), digit1)(input)?;
    Ok((res, id.parse::<u32>().unwrap()))
}

fn numbers(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (res, (winning, owned)) = separated_pair(
        separated_list1(space1, digit1),
        delimited(space1, tag("|"), space1),
        separated_list1(space1, digit1),
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

fn parse_card(line: &str) -> IResult<&str, (u32, HashSet<u32>, HashSet<u32>)> {
    let (res, (id, (winning, owned))) =
        separated_pair(game, preceded(tag(":"), space1), numbers)(line)?;
    Ok((
        res,
        (
            id,
            HashSet::from_iter(winning.iter().cloned()),
            HashSet::from_iter(owned.iter().cloned()),
        ),
    ))
}

fn points(winning: HashSet<u32>, owned: HashSet<u32>) -> i32 {
    let n = winning.intersection(&owned).count();
    if n == 0 {
        0
    } else {
        i32::pow(2, n as u32 - 1)
    }
}

fn score(cards: &Vec<(HashSet<u32>, HashSet<u32>)>) -> u32 {
    let mut cards_counter = vec![1; cards.len()];
    cards
        .iter()
        .enumerate()
        .for_each(|(index, (winning, owned))| {
            let n = winning.intersection(owned).count();
            for i in 1..=n {
                cards_counter[index + i] += cards_counter[index];
            }
        });
    cards_counter.iter().sum()
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

            let cards: Vec<(HashSet<u32>, HashSet<u32>)> = content
                .lines()
                .map(|line| parse_card(line).unwrap())
                .map(|(_res, (_id, winning, owned))| (winning, owned))
                .collect();

            println!("{}", score(&cards));
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
        }
    }
}
