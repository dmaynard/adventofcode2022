pub fn process_part1(input: &str) -> String {
    let result: i32 = input
        .split("\n")
        .map(|line| {
            // println!("line {}", line);
            let mut limits: Vec<i32> = Vec::new();
            line.split(",")
                .map(|range| {
                    // println!(" range {}", range);
                    range
                        .split("-")
                        .map(|bound| {
                            // println!(" bound {}", bound);
                            match bound.parse::<i32>() {
                                Ok(val) => limits.push(val),
                                Err(_e) => limits.push(0),
                            }
                            1
                        })
                        .sum::<i32>()
                })
                .sum::<i32>();
            if (limits[2] >= limits[0] && limits[2] <= limits[1])
                || (limits[3] >= limits[0] && limits[3] <= limits[1])
            {
                // println!(" found one {} ", line);
                1
            } else {
                0
            }
        })
        .sum();

    result.to_string()
}
pub fn process_part2(input: &str) -> String {
    let result: i32 = input
        .split("\n")
        .map(|line| {
            // println!("line {}", line);
            let mut limits: Vec<i32> = Vec::new();
            line.split(",")
                .map(|range| {
                    // println!(" range {}", range);
                    range
                        .split("-")
                        .map(|bound| {
                            // println!(" bound {}", bound);
                            match bound.parse::<i32>() {
                                Ok(val) => limits.push(val),
                                Err(_e) => limits.push(0),
                            }
                            1
                        })
                        .sum::<i32>()
                })
                .sum::<i32>();
            if ((limits[2] >= limits[0]) && (limits[2] <= limits[1]))
                || ((limits[3] >= limits[0]) && (limits[3] <= limits[1]))
                || ((limits[2] < limits[0]) && (limits[3] > limits[1]))
            {
                // println!(" found one {} ", line);
                1
            } else {
                0
            }
        })
        .sum();
    result.to_string()
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
        assert_eq!(result, "2");
    }

    #[test]

    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "4");
    }
}
