pub fn process_part1(input: &str) -> String {
    println!("Hello World {}", input);
    let it = input.lines();

    let result: i32 = input
        .split("\n")
        .map(|line| {
            println!("line {}", line);
            line.split(",")
                .map(|range| {
                    println!(" range {}", range);
                    range
                        .split("-")
                        .map(|bound| {
                            println!(" bound {}", bound);
                            1
                        })
                        .sum::<i32>()
                })
                .sum::<i32>();

            2
        })
        .sum();

    result.to_string()
}
pub fn process_part2(input: &str) -> String {
    let mut result = input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    result.sort_by(|a, b| b.cmp(a));
    // dbg!(result);
    let sum: u32 = result.iter().take(3).sum();
    sum.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "OK");
    }

    #[test]
    #[ignore]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "45000");
    }
}
