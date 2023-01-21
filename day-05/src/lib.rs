use std::collections::VecDeque;
pub fn process_part1(input: &str, width: u32) -> String {
    let mut stacks: Vec<VecDeque<u8>> = Vec::new();
    for i in 0..width {
        println!(" pusing VecDeque");
        stacks.push(VecDeque::new());
    }
    println!(" stacks.len () {}", stacks.len());

    for line in input.split("\n") {
        let mut cindex = 0;
        if line.len() < 5 {
            break;
        }
        for b in line.as_bytes() {
            if cindex % 4 == 1 {
                println!(" char {} {} ", cindex, b);
                if *b != 32 {
                    stacks[cindex / 4].push_back(*b);
                    println!(" pushing {} onto stack {} ", b, cindex / 4);
                }
            }

            cindex = cindex + 1;
        }

        println!("line '{}' len {}", line, line.len());
    }
    dbg!(&stacks[0]);
    println!(" pop_front {}", &stacks[0].pop_front().unwrap());
    dbg!(&stacks[1]);
    dbg!(&stacks[2]);

    "OK".to_string()
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

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 
    
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]

    fn it_works() {
        let result = process_part1(INPUT, 3);
        assert_eq!(result, "2");
    }

    #[test]
    #[ignore]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "4");
    }
}
