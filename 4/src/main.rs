use std::str::FromStr;

#[derive(Debug)]
struct Card {
    id: usize,
    winning_nums: Vec<usize>,
    game_nums: Vec<usize>,
}

trait AssessCard {
    fn get_winning_numbers(&self) -> Vec<usize>;
    fn get_points(&self) -> usize;
}

impl AssessCard for Card {
    fn get_winning_numbers(&self) -> Vec<usize> {
        let Card {
            winning_nums,
            game_nums,
            ..
        } = self;

        return winning_nums
            .into_iter()
            .filter_map(|winning_num| {
                if game_nums.contains(winning_num) {
                    return Some(*winning_num);
                }
                return None;
            })
            .collect();
    }

    fn get_points(&self) -> usize {
        let winning_nums = self.get_winning_numbers();
        let base: usize = 2;
        return match winning_nums.len() {
            0 => 0,
            length => base.pow((length - 1) as u32),
        };
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (card, nums) = s
            .split_once(":")
            .ok_or(anyhow::anyhow!("failed to read line"))?;

        let id = card
            .strip_prefix("Card")
            .map(|id| id.trim().parse::<usize>())
            .ok_or(anyhow::anyhow!("failed to read id"))??;

        let (winning_nums, game_nums) = nums.split_once("|").unwrap_or(("", ""));

        let winning_nums: Vec<usize> = winning_nums
            .trim()
            .split(" ")
            .into_iter()
            .filter_map(|num_str| num_str.trim().parse::<usize>().ok())
            .collect();

        let game_nums: Vec<usize> = game_nums
            .trim()
            .split(" ")
            .into_iter()
            .filter_map(|num_str| num_str.trim().parse::<usize>().ok())
            .collect();

        return Ok(Card {
            id,
            winning_nums,
            game_nums,
        });
    }
}

// fn insert_elements<T>(mut target_vec: Vec<T>, insert_index: usize, insert_vec: Vec<T>) -> Vec<T> {
//     let after_insert_index = target_vec.split_off(insert_index);
//
//     target_vec.extend(insert_vec);
//
//     target_vec.extend(after_insert_index);
//     return target_vec;
// }

fn main() -> Result<(), anyhow::Error> {
    let file = std::fs::read_to_string("input").expect("could not read file");

    let res: usize = file
        .lines()
        .filter_map(|line| {
            return line.parse::<Card>().ok();
        })
        .map(|card| {
            let points = card.get_points();
            return points;
        })
        .sum();

    println!("part 1: {}", res);

    let cards: Vec<Card> = file
        .lines()
        .filter_map(|line| {
            return line.parse::<Card>().ok();
        })
        .collect();

    let mut card_refs: Vec<&Card> = cards.iter().collect();

    let mut i = 0;
    let mut num_cards = card_refs.len();

    while i < num_cards {
        let winning_nums = card_refs[i].get_winning_numbers();

        let new_cards: Vec<&Card> = (card_refs[i].id..card_refs[i].id + winning_nums.len())
            .filter_map(|i| cards.get(i))
            .collect();

        card_refs.extend(new_cards);
        num_cards = card_refs.len();

        i += 1;
    }

    println!("part 2: {}", card_refs.len());

    return Ok(());
}
