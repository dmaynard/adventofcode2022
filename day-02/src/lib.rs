pub fn process_part1(input: &str) -> String {
    let result = input
        .split("\n")
        .map(|item| match item {
            "A X" => 3 + 1,
            "A Y" => 6 + 2,
            "A Z" => 0 + 3,
            "B X" => 0 + 1,
            "B Y" => 3 + 2,
            "B Z" => 6 + 3,
            "C X" => 6 + 1,
            "C Y" => 0 + 2,
            "C Z" => 3 + 3,
            _ => 0,
        })
        .sum::<u32>();
    println!(" sum {}", result);
    result.to_string()
}
pub fn process_part2(input: &str) -> String {
    let result = input
        .split("\n")
        .map(|item| match item {
            "A X" => "A Z",
            "A Y" => "A X",
            "A Z" => "A Y",
            "B X" => "B X",
            "B Y" => "B Y",
            "B Z" => "B Z",
            "C X" => "C Y",
            "C Y" => "C Z",
            "C Z" => "C X",
            _ => {
                println!("unexpected input {}", item);
                "none"
            }
        })
        .map(|item| match item {
            "A X" => 3 + 1,
            "A Y" => 6 + 2,
            "A Z" => 0 + 3,
            "B X" => 0 + 1,
            "B Y" => 3 + 2,
            "B Z" => 6 + 3,
            "C X" => 6 + 1,
            "C Y" => 0 + 2,
            "C Z" => 3 + 3,
            _ => 0,
        })
        .sum::<u32>();
    println!(" sum {}", result);
    result.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "15");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "12");
    }
}
