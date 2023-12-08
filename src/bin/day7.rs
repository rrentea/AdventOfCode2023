use aoc2023::utils;
use counter::Counter;
use nom::{
    character::{complete::space1, streaming::alphanumeric1},
    sequence::separated_pair,
    IResult,
};

#[derive(PartialEq, PartialOrd)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Eq, Debug)]
struct Hand {
    counts: Counter<char, usize>,
    hand: Vec<char>,
}

impl Hand {
    pub fn hand_type(&self) -> Result<HandType, String> {
        let mut counts: Vec<(char, usize)> = self
            .counts
            .iter()
            .map(|(card, count)| (*card, *count))
            .collect();

        let sorted_cards = vec![
            'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
        ];
        counts.sort_by(|a, b| {
            if a.1 != b.1 {
                return b.1.cmp(&a.1);
            }
            let pos1 = sorted_cards.iter().position(|c| *c == a.0).unwrap();
            let pos2 = sorted_cards.iter().position(|c| *c == b.0).unwrap();
            pos2.cmp(&pos1)
        });

        if counts.len() != 1 {
            let index_j = counts.iter().position(|(hand, _)| *hand == 'J');
            if let Some(index_j) = index_j {
                for i in 0..counts.len() {
                    if counts[i].0 != 'J' {
                        counts[i].1 += counts[index_j].1;
                        break;
                    }
                }
                counts.remove(index_j);
            }
        }

        let counts: Vec<usize> = counts.iter().map(|(_, count)| *count).collect();

        match counts[..] {
            [5] => Ok(HandType::FiveOfAKind),
            [4, 1] => Ok(HandType::FourOfAKind),
            [3, 2] => Ok(HandType::FullHouse),
            [3, 1, 1] => Ok(HandType::ThreeOfAKind),
            [2, 2, 1] => Ok(HandType::TwoPair),
            [2, 1, 1, 1] => Ok(HandType::OnePair),
            [1, 1, 1, 1, 1] => Ok(HandType::HighCard),
            _ => Err(format!("{:?}", self.hand)),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.counts == other.counts
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let type1 = self.hand_type().unwrap();
        let type2 = other.hand_type().unwrap();

        if type1 < type2 {
            return Some(std::cmp::Ordering::Greater);
        } else if type1 > type2 {
            return Some(std::cmp::Ordering::Less);
        } else {
            for (card1, card2) in self.hand.iter().zip(other.hand.iter()) {
                if card1 == card2 {
                    continue;
                }
                let sorted_cards = vec![
                    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
                ];
                let pos1 = sorted_cards.iter().position(|c| c == card1);
                let pos2 = sorted_cards.iter().position(|c| c == card2);
                if pos1.unwrap() > pos2.unwrap() {
                    return Some(std::cmp::Ordering::Greater);
                } else {
                    return Some(std::cmp::Ordering::Less);
                }
            }
        }
        Some(std::cmp::Ordering::Equal)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_hand(input: &str) -> IResult<&str, (Hand, u32)> {
    let (remaining, (hand, bid)) =
        separated_pair(alphanumeric1, space1, nom::character::complete::u32)(input)?;

    Ok((
        remaining,
        (
            Hand {
                counts: hand.chars().collect::<Counter<_>>(),
                hand: hand.chars().collect(),
            },
            bid,
        ),
    ))
}

fn process(input: String) -> u32 {
    let mut hands: Vec<(Hand, u32)> = input
        .lines()
        .map(|line| {
            let (_, (hand, bid)) = parse_hand(line).unwrap();
            (hand, bid)
        })
        .collect();
    hands.sort_by(|a, b| a.0.cmp(&b.0));
    hands
        .iter()
        .enumerate()
        // .inspect(|(rank, (hand, bid))| {
        //     println!("{} {:?} {}", rank, hand, bid);
        // })
        .map(|(rank, (_, bid))| (rank as u32 + 1) * bid)
        .sum()
}

fn main() {
    match utils::read_file("src/data/day7/input") {
        Ok(input) => {
            println!("{}", process(input.to_string()));
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
        }
    }
}
