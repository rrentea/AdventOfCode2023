use aoc2023::utils;
use std::cmp::max;
use std::cmp::min;

struct PartNumber {
    number: u32,
    positions: Vec<(usize, usize)>,
}

fn neighbors(lines: &Vec<Vec<char>>, x: i32, y: i32) -> Vec<char> {
    let mut neighbors: Vec<char> = vec![];
    let rows = lines.len() as i32;
    let cols = lines[0].len() as i32;

    (max(0, x - 1)..=min(x + 1, rows - 1)).for_each(|i| {
        (max(0, y - 1)..=min(y + 1, cols - 1)).for_each(|j| {
            if x != i || y != j {
                neighbors.push(lines[i as usize][j as usize]);
            }
        });
    });

    neighbors
}

fn neighbors2(lines: &Vec<Vec<char>>, x: i32, y: i32) -> Vec<(i32, i32)> {
    let mut neighbors: Vec<(i32, i32)> = vec![];
    let rows = lines.len() as i32;
    let cols = lines[0].len() as i32;

    (max(0, x - 1)..=min(x + 1, rows - 1)).for_each(|i| {
        (max(0, y - 1)..=min(y + 1, cols - 1)).for_each(|j| {
            if x != i || y != j {
                neighbors.push((i, j));
            }
        });
    });

    neighbors
}

fn get_neighbor_part_numbers(lines: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<u32> {
    lines
        .iter()
        .enumerate()
        .skip(max(0, x - 1))
        .take(3)
        .flat_map(|(index, line)| extract_numbers(line.clone(), index))
        .filter(
            |PartNumber {
                 number: _,
                 positions,
             }| {
                let neighbors = neighbors2(lines, x as i32, y as i32);
                neighbors.iter().any(|(i, j)| {
                    positions
                        .iter()
                        .any(|(m, n)| *i as usize == *m && *j as usize == *n)
                })
            },
        )
        .map(
            |PartNumber {
                 number,
                 positions: _,
             }| number,
        )
        .collect()
}

fn extract_numbers(line: Vec<char>, line_index: usize) -> Vec<PartNumber> {
    let mut numbers: Vec<PartNumber> = vec![];
    let mut tmp_number: u32 = 0;
    let mut positions: Vec<(usize, usize)> = vec![];

    for (i, c) in line.iter().enumerate() {
        if c.is_ascii_digit() {
            tmp_number = tmp_number * 10 + c.to_digit(10).unwrap();
            positions.push((line_index, i));
        } else if tmp_number != 0 {
            numbers.push(PartNumber {
                number: tmp_number,
                positions: positions.clone(),
            });
            tmp_number = 0;
            positions.clear();
        }
    }
    if tmp_number != 0 {
        numbers.push(PartNumber {
            number: tmp_number,
            positions: positions.clone(),
        });
    }

    numbers
}

fn extract_part_numbers(lines: &Vec<Vec<char>>, numbers: Vec<PartNumber>) -> Vec<u32> {
    numbers
        .iter()
        .filter(|number| {
            let neighbors: String = number
                .positions
                .iter()
                .flat_map(|(i, j)| {
                    neighbors(lines, (*i).try_into().unwrap(), (*j).try_into().unwrap())
                })
                .collect();
            let symbols: String = neighbors
                .chars()
                .filter(|value| !(value.is_ascii_digit() || *value == '.'))
                .collect();
            !symbols.is_empty()
        })
        .map(
            |PartNumber {
                 number,
                 positions: _,
             }| *number,
        )
        .collect()
}

fn main() {
    match utils::read_file("src/data/day3/input") {
        Ok(content) => {
            let lines: Vec<Vec<char>> = content
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect();

            let sum_of_components: u32 = lines
                .iter()
                .enumerate()
                .map(|(line_index, line)| extract_numbers(line.to_vec(), line_index))
                .flat_map(|numbers| extract_part_numbers(&lines, numbers))
                .sum();
            println!("part1: {}", sum_of_components);

            let mut sum: u32 = 0;
            (0..lines.len()).for_each(|i| {
                (0..lines[0].len()).for_each(|j| {
                    if lines[i][j] == '*' {
                        let neighbors = get_neighbor_part_numbers(&lines, i, j);
                        if neighbors.len() == 2 {
                            sum += neighbors[0] * neighbors[1];
                        }
                    }
                })
            });
            println!("part2: {}", sum);
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
        }
    }
}
