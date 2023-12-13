use std::{cmp::Ordering, collections::HashMap, str::FromStr};

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn count_cards(s: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();

    for ch in s.chars() {
        *counts.entry(ch).or_insert(0) += 1;
    }
    return counts;
}

fn count_cards_w_wildcard(s: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();

    for ch in s.chars() {
        *counts.entry(ch).or_insert(0) += 1;
    }

    if let Some(wildcards) = counts.get(&'J') {
        if *wildcards == 5 {
            return counts;
        }
        let key_to_update = counts
            .iter()
            .filter(|&(key, _val)| *key != 'J')
            .max_by(|a, b| {
                return a.1.cmp(&b.1);
            })
            .map(|(k, _v)| k);

        let wildcard_num = *counts.get(&'J').unwrap();

        match key_to_update {
            Some(key) => {
                *counts.entry(*key).or_insert(0) += wildcard_num;
            }
            None => {
                let first_key = counts.keys().nth(0).unwrap();
                *counts.entry(*first_key).or_insert(0) += wildcard_num;
            }
        }
        counts.remove(&'J');
    }

    return counts;
}

impl FromStr for HandType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //how to optionally change the count method???
        let counts = count_cards_w_wildcard(s);

        let hand_type = match counts.len() {
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            3 => {
                if counts.values().any(|&num| num == 3) {
                    HandType::ThreeOfKind
                } else {
                    HandType::TwoPair
                }
            }
            2 => {
                if counts.values().any(|&num| num == 2) {
                    HandType::FullHouse
                } else {
                    HandType::FourOfKind
                }
            }
            1 => HandType::FiveOfKind,
            _ => panic!("couldn't get hand type"),
        };

        return Ok(hand_type);
    }
}

struct Hand<'a> {
    string_val: &'a str,
    card_strengths: Vec<usize>,
    hand_type: HandType,
    bid: usize,
}

impl<'a> Eq for Hand<'a> {}
impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.string_val == other.string_val
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type.cmp(&other.hand_type).then_with(|| {
            return self
                .card_strengths
                .iter()
                .zip(other.card_strengths.iter())
                .map(|(a, b)| a.cmp(b))
                .find(|&order| order != Ordering::Equal)
                .unwrap();
        })
    }
}

fn get_hands<'a>(file_data: &'a str, card_strengths: &Vec<char>) -> Vec<Hand<'a>> {
    let mut hands = file_data
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .filter_map(|slice| match slice.as_slice() {
            [hand, bid] => return Some((*hand, *bid)),
            _ => None,
        })
        .map(|(hand, bid)| {
            return Hand {
                string_val: hand,
                card_strengths: hand
                    .chars()
                    .into_iter()
                    .filter_map(|c| card_strengths.iter().position(|&rank| rank == c))
                    .collect(),
                hand_type: hand.parse::<HandType>().expect("couldn't parse hand"),
                bid: bid.parse::<usize>().unwrap(),
            };
        })
        .collect::<Vec<_>>();

    hands.sort();

    return hands;
}

fn main() {
    let card_strength = vec![
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];

    let card_strength_wildcard = vec![
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];

    let file_data = std::fs::read_to_string("input").expect("could not read file");

    let hands = get_hands(&file_data, &card_strength);
    let total_winnings: usize = hands
        .iter()
        .rev()
        .enumerate()
        .map(|(index, hand)| hand.bid * (index + 1))
        .sum();
    println!("part 1: {}", total_winnings);

    let w_hands = get_hands(&file_data, &card_strength_wildcard);

    let total_winnings_w: usize = w_hands
        .iter()
        .rev()
        .enumerate()
        .map(|(index, hand)| hand.bid * (index + 1))
        .sum();
    println!("part 2: {}", total_winnings_w);
}

#[test]
fn wildcards() {}
