fn main() {
    let data = include_str!("../input.txt");

    let solution_one = part_one(data);
    println!("{solution_one}");

    let solution_two = part_two(data);
    println!("{solution_two}")
}

fn part_one(data: &str) -> u32 {
    data.lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>();
            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .sum()
}

fn part_two(data: &str) -> u32 {
    const CONVERT: [(&str, &str); 9] = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let mut parsed_data = data.to_owned();
    for (key, value) in CONVERT {
        parsed_data = parsed_data.replace(key, format!("{key}{value}{key}").as_str());
    }

    part_one(&parsed_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let data = include_str!("../example01.txt");
        let result = part_one(data);
        assert_eq!(142, result);
    }

    #[test]
    fn example_two() {
        let data = include_str!("../example02.txt");
        let result = part_two(data);
        assert_eq!(281, result);
    }
}
