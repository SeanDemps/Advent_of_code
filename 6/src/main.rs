type Race = (usize, usize);

trait RaceCalc {
    fn get_num_winning_scenarios(&self) -> usize;
}

impl RaceCalc for Race {
    fn get_num_winning_scenarios(&self) -> usize {
        let (time, best_distance) = self;

        let mut last_distance = 0;
        let mut game_index = 0;

        while &last_distance < best_distance {
            last_distance = game_index * (time - game_index);
            if &last_distance <= best_distance {
                game_index += 1;
            }
        }

        return time + 1 - (game_index * 2);
    }
}

fn main() {
    let file_data = std::fs::read_to_string("input").expect("could not read file");
    let data: Vec<_> = file_data
        .split_whitespace()
        .filter_map(|string| string.parse::<usize>().ok())
        .collect();

    let mid = data.len() / 2;
    let races: Vec<Race> = data[..mid]
        .iter()
        .zip(data[mid..].iter())
        .map(|(a, b)| (a.to_owned(), b.to_owned()))
        .collect();

    let number_total_winning_scenarios = races
        .iter()
        .map(|race| race.get_num_winning_scenarios())
        .reduce(|acc, num_scenarios| acc * num_scenarios);

    println!("part 1: {}", number_total_winning_scenarios.unwrap());

    let race_data_strings: Vec<_> = data.into_iter().map(|num| num.to_string()).collect();

    let race: Option<Race> = match race_data_strings
        .chunks_exact(4)
        .filter_map(|chunk| Some(chunk.concat()))
        .filter_map(|chunk| chunk.parse::<usize>().ok())
        .collect::<Vec<_>>()
        .as_slice()
    {
        &[a, b] => Some((a, b)),
        _ => None,
    };

    let num_scenarios_long_race = race.unwrap().get_num_winning_scenarios();

    println!("part 2: {}", num_scenarios_long_race);
}
