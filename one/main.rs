use regex::Regex;

fn get_num(num_str: &str) -> usize {
    let nums = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    if let Ok(num) = num_str.parse::<usize>() {
        return num;
    } else {
        if let Some(num) = nums.iter().position(|&num| {
            return num_str.eq(num);
        }) {
            return num;
        } else {
            return 0;
        }
    }
}

fn main() {
    let mut new_reg_string = String::from(r"(\d");
    let nums = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    nums.iter().for_each(|num| {
        let bar = String::from("|");
        let num = bar + num;
        new_reg_string.push_str(&num);
    });
    new_reg_string.push_str(")");

    let mut new_reg_string2 = String::from(new_reg_string.clone());
    new_reg_string2 = r".*".to_string() + &new_reg_string2 + ".*$";
    println!("{}", new_reg_string2);

    let reg = Regex::new(&new_reg_string).unwrap();
    let reg2 = Regex::new(&new_reg_string2).unwrap();
    let file = std::fs::read_to_string("input").expect("could not read file");

    let sum: usize = file
        .lines()
        .map(|line| {
            if let (Some(one), Some(two)) = (reg.captures(line), reg2.captures(line)) {
                let one = get_num(&one[1]).to_string();
                let two = get_num(&two[1]).to_string();

                let mut s = String::from(one);
                s.push_str(&two);
                return s;
            }
            return "".to_string();
        })
        .map(|num| {
            return num.parse::<usize>().unwrap_or(0);
        })
        .sum();
    println!("{}", sum);
}
