pub fn process_part1(input: &str) -> String {
    let mut x: i64 = 1;
    let mut history: Vec<i64> = Vec::new();
    history.push(x); // 1 based indexes
    let _result: i64 = input
        .lines()
        .map(|line| {
            let opcode = &line[0..4];
            match opcode {
                "noop" => {
                    history.push(x);
                }
                "addx" => {
                    let val: i64 = line[5..line.len()].parse().unwrap();
                    history.push(x);
                    history.push(x);
                    x = x + val;
                }
                _ => {
                    println!(" unexpected opcode {}", opcode)
                }
            }
            x
        })
        .count()
        .try_into()
        .unwrap();
    // dbg!({ history });
    let indexes: [i64; 6] = [20, 60, 100, 140, 180, 220];
    let mut answer: i64 = 0;
    for index in indexes {
        answer = answer + (index * history[index as usize]);
    }
    //     let answer: i64 = indexes
    //         .iter()
    //         .map(|i| (*i as i64 * history[*i]) as i64)
    //         .sum:i64()
    //         .unwrap();
    answer.to_string()
}
pub fn process_part2(input: &str) -> String {
    let mut x: i64 = 1;
    let mut history: Vec<i64> = Vec::new();
    history.push(x); // 1 based indexes
    let _result: i64 = input
        .lines()
        .map(|line| {
            let opcode = &line[0..4];
            match opcode {
                "noop" => {
                    history.push(x);
                }
                "addx" => {
                    let val: i64 = line[5..line.len()].parse().unwrap();
                    history.push(x);

                    history.push(x);
                    x = x + val;
                }
                _ => {
                    println!(" unexpected opcode {}", opcode)
                }
            }
            x
        })
        .count()
        .try_into()
        .unwrap();
    // dbg!({ history });
    let indexes: [i64; 6] = [20, 60, 100, 140, 180, 220];
    let mut answer: i64 = 0;
    for index in indexes {
        answer = answer + (index * history[index as usize]);
    }
    for (i, val) in history.iter().enumerate() {
        let mut j: i64 = (i) as i64 % 40;
        if j == 0 {
            println!()
        }
        if ((j - 1) - val).abs() < 2 {
            print!("#");
        } else {
            print!(".");
        }
    }
    //     let answer: i64 = indexes
    //         .iter()
    //         .map(|i| (*i as i64 * history[*i]) as i64)
    //         .sum:i64()
    //         .unwrap();
    answer.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "13140");
    }

    #[test]

    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "13140");
    }
}
