use std::error::Error;

fn get_past(history: &Vec<i64>) -> i64 {
    if history.iter().all(|&data| data == 0) {
        return 0;
    }

    let next_history = history
        .iter()
        .skip(1)
        .zip(history.iter())
        .map(|(&a, &b)| a - b)
        .collect::<Vec<_>>();
    let first_val = history.first().unwrap();

    return first_val - get_past(&next_history);
}

// previous first value - current first value

fn get_prediction(history: &Vec<i64>, sum: i64) -> i64 {
    if history.iter().all(|&data| data == 0) {
        return sum;
    }

    let next_history = history
        .iter()
        .skip(1)
        .zip(history.iter())
        .map(|(&a, &b)| a - b)
        .collect::<Vec<_>>();

    let summed_val = sum + history.last().unwrap_or(&0);

    return get_prediction(&next_history, summed_val);
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_data = std::fs::read_to_string("input").expect("could not read file");

    let histories = file_data
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|chunk| chunk.parse::<i64>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    let summm: i64 = histories
        .iter()
        .map(|history| get_prediction(history, 0))
        .sum();

    println!("part 1: {}", summm);

    let summm: i64 = histories.iter().map(|history| get_past(history)).sum();

    println!("part 2: {}", summm);

    Ok(())
}
