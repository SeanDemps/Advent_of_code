use std::ops::Range;

type MappedVal = Vec<(Range<i64>, i64)>;

trait GetNextVal {
    fn get_next(&self, index: i64) -> i64;
}

impl GetNextVal for MappedVal {
    fn get_next(&self, index: i64) -> i64 {
        self.iter()
            .find(|(range, _offset)| range.contains(&index))
            .map(|(_range, offset)| index + offset)
            .unwrap_or(index)
    }
}

fn get_location_number(mappings: &Vec<MappedVal>, id: i64) -> i64 {
    return mappings
        .iter()
        .fold(id, |index, mapping| mapping.get_next(index));
}

fn main() -> Result<(), anyhow::Error> {
    let file_data = std::fs::read_to_string("input").expect("could not read file");

    let blocks: Vec<&str> = file_data.split("\n\n").collect();

    let seeds = blocks
        .get(0)
        .ok_or(anyhow::anyhow!("failed to get first line"))?
        .strip_prefix("seeds: ")
        .ok_or(anyhow::anyhow!("failed to strip prefix"))?
        .split_whitespace()
        .into_iter()
        .map(|seed_str| seed_str.parse::<i64>().expect("no"))
        .collect::<Vec<_>>();

    let mappings = blocks
        .iter()
        .skip(1)
        .map(|block| {
            let lines: Vec<_> = block.trim().split("\n").collect();
            let _name = lines.get(0).unwrap().split_whitespace().nth(0);

            let ranges: Vec<(Range<i64>, i64)> = lines
                .iter()
                .skip(1)
                .map(|line| {
                    let map: Vec<_> = line
                        .split_whitespace()
                        .map(|val| {
                            return val.parse::<i64>().expect("could not parse map");
                        })
                        .collect();
                    let destination_range_start = map[0];
                    let source_range_start = map[1];
                    let range_len = map[2];

                    let range = source_range_start..source_range_start + range_len;
                    let offset = destination_range_start - source_range_start;
                    return (range, offset);
                })
                .collect();

            return ranges;
        })
        .collect::<Vec<_>>();

    let lowest_location_number = seeds
        .iter()
        .map(|seed| get_location_number(&mappings, *seed))
        .min();

    println!("part 1: {}", lowest_location_number.unwrap());

    let part2_seeds: Vec<i64> = blocks
        .get(0)
        .ok_or(anyhow::anyhow!("failed to get first line"))?
        .strip_prefix("seeds: ")
        .ok_or(anyhow::anyhow!("failed to strip prefix"))?
        .split_whitespace()
        .into_iter()
        .map(|number| number.parse::<i64>().expect("no"))
        .collect();

    let seed_ranges: Vec<Range<_>> = part2_seeds
        .chunks(2)
        .filter_map(|chunk| {
            if chunk.len() == 2 {
                return Some(chunk[0]..chunk[0] + chunk[1]);
            }

            return None;
        })
        .collect();

    let lowest_location_number = seed_ranges
        .into_iter()
        .map(|seed_range| {
            seed_range
                .map(|seed| get_location_number(&mappings, seed))
                .min()
                .unwrap()
        })
        .min();

    println!("part2: {}", lowest_location_number.unwrap());

    Ok(())
}
