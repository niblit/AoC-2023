fn main() {
    let data = include_str!("../input.txt");
    let result = solve(data);
    println!("{result}");
}

fn solve(data: &str) -> u32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let data = include_str!("../example01.txt");
        let result = solve(data);
        assert_eq!(142, result);
    }
}
