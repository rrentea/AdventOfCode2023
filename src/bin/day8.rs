use std::collections::HashMap;

use aoc2023::utils;
use nom::{
    bytes::complete::tag,
    character::{complete::{newline, line_ending}, streaming::{alpha1, alphanumeric1}},
    multi::{separated_list1, separated_list0},
    sequence::{delimited, separated_pair},
    IResult, combinator::eof, branch::alt,
};

fn parse(input: &str) -> IResult<&str, (Vec<char>, HashMap<&str, (&str, &str)>)> {
    let (remaining, lr_steps) = alpha1(input)?;
    let (remaining, _) = newline(remaining)?;
    let (remaining, _) = newline(remaining)?;

    let (remaining, values) = separated_list1(
        alt((line_ending, eof)),
        separated_pair(
            alphanumeric1,
            tag(" = "),
            delimited(
                tag("("),
                separated_pair(alphanumeric1, tag(", "), alphanumeric1),
                tag(")"),
            ),
        ),
    )(remaining)?;

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    for (node, (left, right)) in values.iter() {
        map.insert(node, (left, right));
    }

    Ok((remaining, (lr_steps.chars().collect(), map)))
}

fn play(node: &str, map: &HashMap<&str, (&str, &str)>, lr_steps: &[char]) -> u64 {
    let mut current_node = node;
    let mut n = 0;
    for step in lr_steps.iter().cycle() {
        if current_node.ends_with('Z') {
            break;
        }
        if *step == 'R' {
            current_node = map.get(current_node).unwrap().1;
        } else if *step == 'L' {
            current_node = map.get(current_node).unwrap().0;
        }
        n += 1
    }
    n
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn process(input: String) -> u64 {
    let (_, (lr_steps, map)) = parse(&input).unwrap();

    let mut current_nodes: Vec<&str> = vec![];
    for node in map.keys(){
        if node.ends_with('A') {
            current_nodes.push(node);
        }
    }

    let nums: Vec<u64> = current_nodes
        .iter()
        .map(|node| play(node, &map, &lr_steps))
        .collect();

    lcm(&nums)

}

fn main() {
    match utils::read_file("src/data/day8/input") {
        Ok(input) => {
            println!("{}", process(input.to_string()));
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
        }
    }
}
