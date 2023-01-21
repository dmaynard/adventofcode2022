use std::collections::VecDeque;

fn parse_section0(stacks: &mut Vec<VecDeque<char>>, section0: &str) -> u32 {
    let mut num_boxes = 0;
    for line in section0.split("\n") {
        let mut cindex = 0;
        if line.starts_with(&"1   2   3".to_string()) {
            //println!("found ending line {}", line);
            break;
        }
        for b in line.as_bytes() {
            if cindex % 4 == 1 {
                //println!(" char {} {} ", cindex, b);
                if *b != 32 {
                    stacks[cindex / 4].push_back(*b as char);
                    num_boxes = num_boxes + 1;
                    //println!(" pushing {} onto stack {} ", b, cindex / 4);
                }
            }
            cindex = cindex + 1;
        }
        // println!("line '{}' len {}", line, line.len());
    }
    num_boxes
}
fn parse_section1(stacks: &mut Vec<VecDeque<char>>, section1: &str) -> u32 {
    let mut num_moves = 0;
    for line in section1.split("\n") {
        let mut params: Vec<usize> = Vec::new();
        // println!("line {}", line);
        for token in line.split_whitespace() {
            // println!(" token {}", token);
            match token.parse() {
                Ok(val) => params.push(val),
                Err(_e) => {}
            }
        }

        for _i in 0..params[0] {
            let box_name = stacks[params[1] - 1].pop_front().unwrap();

            stacks[params[2] - 1].push_front(box_name);
            // println!("moving from {} to {} ", params[1], params[2]);
            num_moves = num_moves + 1;
        }
        // dbg!(&stacks[0]);
        // dbg!(&stacks[1]);
        // dbg!(&stacks[2]);
    }
    num_moves
}
pub fn process_part1(input: &str, width: u32) -> String {
    let mut stacks: Vec<VecDeque<char>> = Vec::new();

    for _i in 0..width {
        // println!(" pushing VecDeque");
        stacks.push(VecDeque::new());
    }
    //println!(" stacks.len () {}", stacks.len());
    let mut section_num = 0;

    for section in input.split("\n\n") {
        // println!(" section {}", section);
        if section_num == 0 {
            let num_boxes = parse_section0(&mut stacks, section);
            println!("num_boxes {}", num_boxes);
            // dbg!(&stacks[0]);
            // dbg!(&stacks[1]);
            // dbg!(&stacks[2]);
        }
        if section_num == 1 {
            let num_moves = parse_section1(&mut stacks, section);
            // println!("num_moves {}", num_moves);
            // dbg!(&stacks[0]);
            // dbg!(&stacks[1]);
            // dbg!(&stacks[2]);
        }
        section_num = section_num + 1;
    }

    // dbg!(&stacks[0]);
    // dbg!(&stacks[1]);
    // dbg!(&stacks[2]);
    let mut answer = String::new();

    for mut stack in stacks {
        dbg!(&stack);
        match &stack.pop_front() {
            Some(char) => answer.push(*char),
            None => {}
        }
    }
    answer
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
        assert_eq!(result, "CMZ");
    }

    #[test]
    #[ignore]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "4");
    }
}
