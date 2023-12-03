use std::ops::Range;

use regex::Regex;

fn ranges_overlap<T: PartialOrd>(range1: &Range<T>, range2: &Range<T>) -> bool {
    range1.start <= range2.end && range2.start <= range1.end
}

type Matcher = (Range<usize>, String);

trait RangeMatch {
    fn get_range_match(&self, string: &str) -> Vec<Matcher>;
}

impl RangeMatch for Regex {
    fn get_range_match(&self, string: &str) -> Vec<Matcher> {
        return self
            .captures_iter(string)
            .filter_map(|cap| cap.get(0))
            .map(|matcher| (matcher.start()..matcher.end(), matcher.as_str().to_string()))
            .collect();
    }
}

trait GetMatches {
    fn get_matches(&self, match_list: &Vec<Matcher>) -> Vec<&String>;
}

impl GetMatches for Vec<Matcher> {
    fn get_matches(&self, match_list: &Vec<Matcher>) -> Vec<&String> {
        return self
            .iter()
            .filter(|(number_range, _)| {
                let overlap = match_list
                    .iter()
                    .find(|(symbol_range, _)| ranges_overlap(number_range, symbol_range));

                return overlap.is_some();
            })
            .map(|(_, string)| string)
            .collect();
    }
}

fn main() {
    let file = std::fs::read_to_string("input").expect("could not read file");

    let lines = file.lines().collect::<Vec<&str>>();

    let num_regex: Regex = Regex::new(r"\d+").unwrap();
    let symbol_regex: Regex = Regex::new(r"[^\.\d]").unwrap();

    let mut sum: usize = 0;

    for (i, line) in lines.iter().enumerate() {
        let mut prev_symbol: Option<Vec<Matcher>> = None;
        let mut next_symbol: Option<Vec<Matcher>> = None;
        let current_num = num_regex.get_range_match(line);
        let current_symbol = symbol_regex.get_range_match(line);

        if i.gt(&0) {
            if let Some(prev) = lines.get(i - 1) {
                prev_symbol = Some(symbol_regex.get_range_match(prev));
            }
        }

        if let Some(next) = lines.get(i + 1) {
            next_symbol = Some(symbol_regex.get_range_match(next));
        }

        let mut prev_num_strs = current_num.get_matches(&prev_symbol.unwrap_or(vec![]));
        let mut current_num_strs = current_num.get_matches(&current_symbol);
        let mut next_num_strs = current_num.get_matches(&next_symbol.unwrap_or(vec![]));

        prev_num_strs.append(&mut current_num_strs);
        prev_num_strs.append(&mut next_num_strs);

        sum += prev_num_strs
            .into_iter()
            .map(|num| num.parse::<usize>().expect("should be a number"))
            .sum::<usize>();
    }

    println!("part1: {}", sum);

    // let gear_regex: Regex = Regex::new(r"\*").unwrap();
    //
    // for (i, line) in lines.iter().enumerate() {
    //     let mut prev_num: Option<Vec<Matcher>> = None;
    //     let mut next_num: Option<Vec<Matcher>> = None;
    //     let current_gear = gear_regex.get_range_match(line);
    //     let current_num = num_regex.get_range_match(line);
    //
    //     if i.gt(&0) {
    //         if let Some(prev) = lines.get(i - 1) {
    //             prev_num = Some(num_regex.get_range_match(prev));
    //         }
    //     }
    //
    //     if let Some(next) = lines.get(i + 1) {
    //         next_num = Some(num_regex.get_range_match(next));
    //     }
    //
    //     let mut prev_num_strs = current_gear.get_matches(&prev_num.unwrap_or(vec![]));
    //     let mut current_num_strs = current_gear.get_matches(&current_num);
    //     let mut next_num_strs = current_gear.get_matches(&next_num.unwrap_or(vec![]));
    //
    //     prev_num_strs.append(&mut current_num_strs);
    //     prev_num_strs.append(&mut next_num_strs);
    //
    //     sum += prev_num_strs
    //         .into_iter()
    //         .map(|num| num.parse::<usize>().expect("should be a number"))
    //         .sum::<usize>();
    // }
}
