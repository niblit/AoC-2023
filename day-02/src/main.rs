fn main() {
    println!("Hello, world!");
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
