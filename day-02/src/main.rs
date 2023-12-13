fn main() {
    let input = include_str!("../input.txt");

    let result_one = part_one(input);

    println!("{result_one}");
}

struct Game {
    id: i32,
    runs: Vec<Run>,
}

impl Game {
    pub fn is_possible(&self, red: i32, green: i32, blue: i32) -> bool {
        self.runs
            .iter()
            .all(|run| run.is_possible(red, green, blue))
    }
}

struct Run {
    red: i32,
    green: i32,
    blue: i32,
}

impl Run {
    pub fn is_possible(&self, red: i32, green: i32, blue: i32) -> bool {
        self.red <= red && self.green <= green && self.blue <= blue
    }
}

fn part_one(_data: &str) -> i32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let data = include_str!("../example01.txt");
        let result = part_one(data);
        assert_eq!(8, result);
    }
}
