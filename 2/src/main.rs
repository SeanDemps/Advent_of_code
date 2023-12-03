use std::str::FromStr;

#[derive(Debug)]
struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

impl Set {
    fn could_contain(&self, other: &Set) -> bool {
        let check_red = self.red >= other.red;
        let check_green = self.green >= other.green;
        let check_blue = self.blue >= other.blue;

        return check_red && check_green && check_blue;
    }

    fn get_power(&self) -> usize {
        let sum = self.red * self.green * self.blue;
        println!("{}", sum);
        return sum;
    }
}

impl FromStr for Set {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let set = s.split(",");

        let mut ret = Set {
            red: 0,
            green: 0,
            blue: 0,
        };

        set.into_iter().for_each(|color| {
            let (number, color) = color.trim().split_once(" ").unwrap_or(("0", ""));
            match color {
                "red" => ret.red = number.parse::<usize>().unwrap_or(0),
                "green" => ret.green = number.parse::<usize>().unwrap_or(0),
                "blue" => ret.blue = number.parse::<usize>().unwrap_or(0),
                _ => {}
            }
        });

        return Ok(ret);
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>,
}

impl Game {
    fn get_min_set(&self) -> Set {
        let mut min_set = Set {
            red: 0,
            green: 0,
            blue: 0,
        };

        self.sets.iter().for_each(|set| {
            if set.red > min_set.red {
                min_set.red = set.red;
            }

            if set.green > min_set.green {
                min_set.green = set.green;
            }

            if set.blue > min_set.blue {
                min_set.blue = set.blue;
            }
        });

        return min_set;
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game, sets) = s.split_once(":").unwrap_or(("", ""));
        let id = game
            .strip_prefix("Game ")
            .unwrap_or("")
            .parse::<usize>()
            .expect("could not parse id");

        let sets = sets.split(";");
        let sets: Vec<Set> = sets
            .into_iter()
            .map(|set| set.parse::<Set>().expect("error on set parse"))
            .collect();
        return Ok(Game { id, sets });
    }
}

fn main() -> Result<(), anyhow::Error> {
    let compare_set = Set {
        red: 12,
        green: 13,
        blue: 14,
    };

    let file = std::fs::read_to_string("input").expect("could not read file");

    let sum: usize = file
        .lines()
        .map(|line| {
            let game = line.parse::<Game>().expect("couldn't parse game");
            return (
                game.sets.iter().all(|set| compare_set.could_contain(set)),
                game,
            );
        })
        .filter(|(possible, _game)| *possible)
        .map(|(_possible, game)| game.id)
        .sum();
    println!("part1: {}", sum);

    let sum2: usize = file
        .lines()
        .map(|line| {
            return line.parse::<Game>().expect("couldn't parse game");
        })
        .map(|game| game.get_min_set().get_power())
        .sum();
    println!("part2: {}", sum2);

    return Ok(());
}
