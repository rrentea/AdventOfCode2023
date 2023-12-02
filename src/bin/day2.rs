use aoc2023::utils;
use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
struct Cube<'a> {
    color: &'a str,
    amount: u32,
}

#[derive(Debug)]
struct Game<'a> {
    id: u32,
    rounds: Vec<Vec<Cube<'a>>>,
}

fn cube(input: &str) -> IResult<&str, Cube> {
    let (input, (amount, color)) = separated_pair(complete::u32, tag(" "), alpha1)(input)?;
    Ok((input, Cube { color, amount }))
}

fn round(input: &str) -> IResult<&str, Vec<Cube>> {
    let (input, cubes) = separated_list1(tag(", "), cube)(input)?;
    Ok((input, cubes))
}

fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) = preceded(tag("Game "), complete::u32)(input)?;
    let (input, rounds) = preceded(tag(": "), separated_list1(tag("; "), round))(input)?;
    Ok((input, Game { id, rounds }))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(line_ending, game)(input)?;
    Ok((input, games))
}

fn check_game(game: &Game, bag_size: &[u32]) -> bool {
    for round in &game.rounds {
        for cube in round {
            match cube.color {
                "red" => {
                    if cube.amount > bag_size[0] {
                        return false;
                    }
                }
                "green" => {
                    if cube.amount > bag_size[1] {
                        return false;
                    }
                }
                "blue" => {
                    if cube.amount > bag_size[2] {
                        return false;
                    }
                }
                _ => println!("Impossible color"),
            }
        }
    }
    true
}

fn get_game_power(game: &Game) -> u32 {
    let mut min_cubes = vec![0, 0, 0];
    for round in &game.rounds {
        for cube in round {
            match cube.color {
                "red" => {
                    if cube.amount > min_cubes[0] {
                        min_cubes[0] = cube.amount;
                    }
                }
                "green" => {
                    if cube.amount > min_cubes[1] {
                        min_cubes[1] = cube.amount;
                    }
                }
                "blue" => {
                    if cube.amount > min_cubes[2] {
                        min_cubes[2] = cube.amount;
                    }
                }
                _ => println!("Impossible color"),
            }
        }
    }
    min_cubes[0] * min_cubes[1] * min_cubes[2]
}

fn part1(input: &str) -> u32 {
    let (_, games) = parse_games(input).unwrap();
    let bag_size = vec![12, 13, 14];
    games
        .iter()
        .filter(|game| check_game(game, &bag_size))
        .map(|game| game.id)
        .sum()
}

fn part2(input: &str) -> u32 {
    let (_, games) = parse_games(input).unwrap();
    games.iter().map(get_game_power).sum()
}

fn main() {
    match utils::read_file("src/data/day2/input") {
        Ok(content) => {
            println!("part1: {}; part2: {}", part1(&content), part2(&content));
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
        }
    }
}
